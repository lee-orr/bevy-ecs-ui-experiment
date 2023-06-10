use std::marker::PhantomData;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_ecss::{Class, StyleSheet, StyleSheetAsset};

use crate::{
    expression::Expression,
    reactive_expression_handlers::{ReactiveComponentExpressionHandler, ReactiveExpressionPlugin},
    string_expression::StringExpression,
    ui_asset::{Conditional, Image, Node, Text, UiNodeTree},
    UiNode,
};

pub struct UiPlugin<T: UIState>(PhantomData<T>, Option<(String, String)>);

impl<T: UIState> Plugin for UiPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(display_ui::<T>.in_base_set(CoreSet::PostUpdate))
            .add_system(hot_reload_assets::<T>.in_base_set(CoreSet::PreUpdate))
            .add_plugin(ReactiveExpressionPlugin::<T>::default());
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
    pub handle: Handle<UiNodeTree>,
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
    assets: Res<Assets<UiNodeTree>>,
    ui: Query<(Entity, &T), Without<InitializedUi<T>>>,
    handle: Option<Res<UiHandle<T>>>,
    asset_server: Res<AssetServer>,
) {
    let Some(handle) = handle else { return; };
    let sheet = &handle.style;
    let Some(tree) = assets.get(&handle.handle) else { return; };
    let Some(root) = tree.0.get(0) else { return; };

    for (entity, state) in ui.iter() {
        let mut cmd = commands.entity(entity);
        cmd.insert(InitializedUi::<T>::new());
        cmd.insert(StyleSheet::new(sheet.clone()));
        cmd.insert(NodeBundle::default());
        spawn_ui(entity, &mut cmd, root, state, asset_server.as_ref(), tree);
    }
}

fn spawn_ui<T: UIState>(
    root: Entity,
    e: &mut EntityCommands,
    node: &UiNode,
    state: &T,
    asset_server: &AssetServer,
    tree: &UiNodeTree,
) -> Entity {
    match node {
        UiNode::Node(Node {
            children,
            name,
            class,
            style: _,
        }) => {
            setup_common_components(name, e, root, state, class);
            e.insert(NodeBundle::default());
            e.with_children(|p| {
                for child in children.iter() {
                    let Some(child) = tree.0.get(*child) else {
                        continue;
                    };
                    let mut e = p.spawn_empty();
                    let _ = spawn_ui(root, &mut e, child, state, asset_server, tree);
                }
            });
        }
        UiNode::Image(Image {
            name,
            class,
            style: _,
            image_path,
        }) => {
            setup_common_components(name, e, root, state, class);

            if matches!(image_path, StringExpression::Expression(_)) {
                e.insert(ReactiveComponentExpressionHandler::<
                    UiImage,
                    String,
                    StringExpression,
                >::new(root, &[("src", image_path)]));
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
            setup_common_components(name, e, root, state, class);

            if matches!(text, StringExpression::Expression(_)) {
                e.insert(ReactiveComponentExpressionHandler::<
                    bevy::text::Text,
                    String,
                    StringExpression,
                >::new(root, &[("text", text)]));
            }

            e.insert(TextBundle::from_section(
                text.process(state),
                TextStyle::default(),
            ));
        }
        UiNode::RawText(text) => {
            if matches!(text, StringExpression::Expression(_)) {
                e.insert(ReactiveComponentExpressionHandler::<
                    bevy::text::Text,
                    String,
                    StringExpression,
                >::new(root, &[("text", text)]));
            }
            e.insert(TextBundle::from_section(
                text.process(state),
                TextStyle::default(),
            ));
        }
        UiNode::Conditional(Conditional {
            condition,
            if_true,
            if_false,
        }) => {}
        UiNode::Empty => {}
    }
    e.id()
}

fn setup_common_components<T: UIState>(
    name: &Option<StringExpression>,
    e: &mut EntityCommands,
    root: Entity,
    state: &T,
    class: &Option<StringExpression>,
) {
    if let Some(name) = name {
        if let StringExpression::Value(v) = name {
            e.insert(Name::new(v.to_string()));
        } else {
            let reactive_handler = ReactiveComponentExpressionHandler::<
                Name,
                String,
                StringExpression,
            >::new(root, &[("name", name)]);
            e.insert((reactive_handler, Name::new(name.process(state))));
        }
    }
    if let Some(class) = class {
        if let StringExpression::Value(v) = class {
            e.insert(Class::new(v.to_string()));
        } else {
            let reactive_handler = ReactiveComponentExpressionHandler::<
                Class,
                String,
                StringExpression,
            >::new(root, &[("class", class)]);
            e.insert((reactive_handler, Class::new(class.process(state))));
        }
    }
}

fn load_ui_on_startup<T: UIState>(
    mut commands: Commands,
    load: Option<Res<LoadUiHandle<T>>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(load) = load {
        let handle: Handle<UiNodeTree> = asset_server.load(&load.0);
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
    mut ev_node_asset: EventReader<AssetEvent<UiNodeTree>>,
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

pub trait UIState: Component + Reflect {}

impl<T: Component + Reflect> UIState for T {}
