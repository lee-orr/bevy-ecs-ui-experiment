use std::marker::PhantomData;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_ecss::{Class, StyleSheet, StyleSheetAsset};

use crate::{
    ui_asset::{Image, Node, Text},
    UiNode,
};

pub struct UiPlugin<T: UIState>(PhantomData<T>, Option<(String, String)>);

impl<T: UIState> Plugin for UiPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(display_ui::<T>.in_base_set(CoreSet::Last))
            .add_system(update_state::<T>.in_base_set(CoreSet::PostUpdate))
            .add_system(hot_reload_assets::<T>.in_base_set(CoreSet::PreUpdate));
        if let Some((uri, style)) = &self.1 {
            app.insert_resource(LoadUiHandle::<T>(uri.clone(), style.clone(), PhantomData));
            app.add_startup_system(load_ui_on_startup::<T>);
        }
    }
}

impl<T: UIState> UiPlugin<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(mut self, asset: impl Into<String>, stylesheet: impl Into<String>) -> Self {
        self.1 = Some((asset.into(), stylesheet.into()));
        self
    }
}

impl<T: UIState> Default for UiPlugin<T> {
    fn default() -> Self {
        Self(PhantomData, None)
    }
}

#[derive(Resource)]
pub struct UiHandle<T: UIState> {
    pub handle: Handle<UiNode>,
    pub style: Handle<StyleSheetAsset>,
    phantom: PhantomData<T>,
}

#[derive(Resource)]
struct LoadUiHandle<T: UIState>(String, String, PhantomData<T>);

#[derive(Component, Default)]
pub struct InitializedUi<T: UIState>(PhantomData<T>);

impl<T: UIState> InitializedUi<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

fn display_ui<T: UIState>(
    mut commands: Commands,
    assets: Res<Assets<UiNode>>,
    ui: Query<(Entity, &T), Without<InitializedUi<T>>>,
    handle: Option<Res<UiHandle<T>>>,
    asset_server: Res<AssetServer>,
) {
    let Some(handle) = handle else { return; };
    let sheet = &handle.style;
    let Some(asset) = assets.get(&handle.handle) else { return; };

    for (entity, state) in ui.iter() {
        let mut cmd = commands.entity(entity);
        cmd.insert(InitializedUi::<T>::new());
        cmd.insert(StyleSheet::new(sheet.clone()));
        cmd.insert(NodeBundle::default());
        cmd.with_children(|p| {
            let mut cmd = p.spawn_empty();
            let _ = spawn_ui(&mut cmd, asset, state, asset_server.as_ref());
        });
    }
}

fn spawn_ui<T: UIState>(
    e: &mut EntityCommands,
    node: &UiNode,
    state: &T,
    asset_server: &AssetServer,
) -> Entity {
    match node {
        UiNode::Node(Node {
            children,
            name,
            class,
            style: _,
        }) => {
            if let Some(name) = name {
                e.insert(Name::new(name.process(state)));
            }
            if let Some(class) = class {
                e.insert(Class::new(class.process(state)));
            }
            e.insert(NodeBundle::default());
            e.with_children(|p| {
                for child in children.iter() {
                    let mut e = p.spawn_empty();
                    let _ = spawn_ui(&mut e, child, state, asset_server);
                }
            });
        }
        UiNode::Image(Image {
            name,
            class,
            style: _,
            image_path,
        }) => {
            if let Some(name) = name {
                e.insert(Name::new(name.process(state)));
            }
            if let Some(class) = class {
                e.insert(Class::new(class.process(state)));
            }
            let handle: Handle<bevy::prelude::Image> = asset_server.load(image_path.process(state));
            e.insert(ImageBundle {
                image: UiImage {
                    texture: handle,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
        UiNode::Text(Text {
            name,
            class,
            style: _,
            text,
        }) => {
            if let Some(name) = name {
                e.insert(Name::new(name.process(state)));
            }
            if let Some(class) = class {
                e.insert(Class::new(class.process(state)));
            }
            e.insert(TextBundle::from_section(
                text.process(state),
                TextStyle::default(),
            ));
        }
        UiNode::RawText(text) => {
            e.insert(TextBundle::from_section(
                text.process(state),
                TextStyle::default(),
            ));
        }
    }
    e.id()
}

fn load_ui_on_startup<T: UIState>(
    mut commands: Commands,
    load: Option<Res<LoadUiHandle<T>>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(load) = load {
        let handle: Handle<UiNode> = asset_server.load(&load.0);
        let style: Handle<StyleSheetAsset> = asset_server.load(&load.1);
        commands.insert_resource(UiHandle {
            handle,
            style,
            phantom: PhantomData::<T>,
        });
        commands.remove_resource::<LoadUiHandle<T>>();
    }
}

fn hot_reload_assets<T: UIState>(
    mut commands: Commands,
    mut ev_node_asset: EventReader<AssetEvent<UiNode>>,
    mut ev_style_asset: EventReader<AssetEvent<StyleSheetAsset>>,
    handle: Option<Res<UiHandle<T>>>,
    ui: Query<Entity, With<InitializedUi<T>>>,
) {
    let Some(handle) = handle else { return; };
    let mut reset = false;
    for ev in ev_node_asset.iter() {
        if let AssetEvent::Modified { handle: h } = ev {
            if *h == handle.handle {
                reset = true;
                break;
            }
        }
    }
    if !reset {
        for ev in ev_style_asset.iter() {
            if let AssetEvent::Modified { handle: h } = ev {
                if *h == handle.style {
                    reset = true;
                    break;
                }
            }
        }
    }

    if !reset {
        return;
    }

    for e in ui.iter() {
        commands
            .entity(e)
            .remove::<InitializedUi<T>>()
            .despawn_descendants();
    }
}

fn update_state<T: UIState>(
    mut commands: Commands,
    ui: Query<Entity, (With<InitializedUi<T>>, Changed<T>)>,
) {
    for e in ui.iter() {
        commands
            .entity(e)
            .remove::<InitializedUi<T>>()
            .despawn_descendants();
    }
}

pub trait UIState: Component + Reflect {}

impl<T: Component + Reflect> UIState for T {}
