use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_ecss::{Class, StyleSheet, StyleSheetAsset};

use crate::{
    expression::Expression,
    logical_nodes::UiIfElse,
    reactive_expression_handlers::{
        GetCachedExpressionHandlers, GetExpressionHandlers, ReactiveExpressionPlugin,
    },
    string_expression::StringExpression,
    ui_asset::{IfElse, Image, Node, Text, UiNodeTree},
    ArrayExpression, UiNode,
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
        spawn_ui(
            entity,
            entity,
            None,
            &mut commands,
            root,
            state,
            asset_server.as_ref(),
            tree,
            entity,
        );
    }
}

pub fn spawn_ui<T: UIState>(
    root: Entity,
    entity: Entity,
    parent: Option<Entity>,
    commands: &mut Commands,
    node: &UiNode,
    state: &T,
    asset_server: &AssetServer,
    tree: &UiNodeTree,
    data_root: Entity,
) -> Entity {
    match node {
        UiNode::Node(Node {
            children,
            name,
            class,
            style: _,
        }) => {
            setup_common_components(name, commands, entity, root, state, class);
            commands.entity(entity).insert(NodeBundle::default());
            let children = children
                .iter()
                .filter_map(|child| {
                    let Some(child) = tree.0.get(*child) else {
                    return None;
                };
                    Some(spawn_ui(
                        root,
                        commands.spawn_empty().id(),
                        Some(entity),
                        commands,
                        child,
                        state,
                        asset_server,
                        tree,
                        data_root,
                    ))
                })
                .collect::<Vec<_>>();
            commands.entity(entity).push_children(&children);
        }
        UiNode::Image(Image {
            name,
            class,
            style: _,
            image_path,
        }) => {
            setup_common_components(name, commands, entity, root, state, class);

            let path = image_path.process(state);

            let texture: Handle<bevy::prelude::Image> = asset_server.load(&path);

            let image = UiImage {
                texture,
                ..Default::default()
            };

            image.setup_cached_expression_handlers(
                &mut commands.entity(root),
                entity,
                image_path.clone(),
                path,
            );

            commands.entity(entity).insert(ImageBundle {
                image,
                ..Default::default()
            });
        }
        UiNode::Text(Text {
            name,
            class,
            style: _,
            text: text_expression,
        }) => {
            setup_common_components(name, commands, entity, root, state, class);

            let value = text_expression.process(state);
            let text = bevy::text::Text::from_section(value, TextStyle::default());
            text.setup_expression_handlers(
                &mut commands.entity(root),
                entity,
                text_expression.clone(),
            );

            commands.entity(entity).insert(TextBundle {
                text,
                ..Default::default()
            });
        }
        UiNode::RawText(text_expression) => {
            let value = text_expression.process(state);
            let text = bevy::text::Text::from_section(value, TextStyle::default());
            text.setup_expression_handlers(
                &mut commands.entity(root),
                entity,
                text_expression.clone(),
            );

            commands.entity(entity).insert(TextBundle {
                text,
                ..Default::default()
            });
        }
        UiNode::IfElse(IfElse { conditions }) => {
            let mut id = Entity::PLACEHOLDER;

            commands.entity(root).with_children(|p| {
                id = p.spawn_empty().id(); //((NodeBundle::default(), ui_if_else)).id();
            });

            let condition_expression = ArrayExpression(
                conditions
                    .iter()
                    .filter_map(|(expression, _)| expression.clone())
                    .collect(),
            );

            let child_options: Vec<usize> = conditions.iter().map(|(_, id)| *id).collect();

            let current_condition = condition_expression.process(state);
            let num_conditions = conditions.len();

            let child = match current_condition {
                Some(id) => child_options.get(id).cloned(),
                None => child_options.get(num_conditions).cloned(),
            };

            let ui_child = match child {
                Some(child) => tree.0.get(child).map(|node| {
                    spawn_ui(
                        id,
                        entity,
                        parent,
                        commands,
                        node,
                        state,
                        asset_server,
                        tree,
                        data_root,
                    )
                }),
                _ => None,
            };

            let ui_child = ui_child.unwrap_or(spawn_ui(
                id,
                entity,
                parent,
                commands,
                &UiNode::Empty,
                state,
                asset_server,
                tree,
                data_root,
            ));

            let ui_if_else = UiIfElse {
                current_condition,
                num_conditions,
                child_options,
                data_root,
                ui_parent: parent.unwrap_or(root),
                ui_child: (current_condition, ui_child),
            };

            ui_if_else.setup_expression_handlers(
                &mut commands.entity(root),
                id,
                condition_expression,
            );

            commands
                .entity(id)
                .insert((ui_if_else, NodeBundle::default()));
        }
        UiNode::Empty => {
            commands.entity(entity).insert(NodeBundle {
                visibility: Visibility::Hidden,
                ..Default::default()
            });
        }
    }
    entity
}

fn setup_common_components<T: UIState>(
    name: &Option<StringExpression>,
    commands: &mut Commands,
    entity: Entity,
    root: Entity,
    state: &T,
    class: &Option<StringExpression>,
) {
    if let Some(name) = name {
        let n = Name::new(name.process(state));
        n.setup_expression_handlers(&mut commands.entity(root), entity, name.clone());
        commands.entity(entity).insert(n);
    }
    if let Some(class) = class {
        let c = Class::new(class.process(state));
        c.setup_expression_handlers(&mut commands.entity(root), entity, class.clone());
        commands.entity(entity).insert(c);
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
