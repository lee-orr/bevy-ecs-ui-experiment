use crate::ExpressionEngine;
use crate::UiNode;

use crate::ui_plugin::spawn_ui;

use crate::ui_plugin::UiHandle;

use crate::ui_asset::UiNodeTree;

use crate::UIState;

use crate::reactive_expression_handlers::ComponentExpressionHandler;

use crate::reactive_expression_handlers::ReactiveExpressionHandler;

use bevy::prelude::*;

use crate::ExpressionArray;

use crate::reactive_expression_handlers::GetExpressionHandlers;

#[derive(Component)]
pub struct UiIfElse {
    pub current_condition: Option<usize>,
    pub num_conditions: usize,
    pub child_options: Vec<usize>,
    pub data_root: Entity,
    pub ui_parent: Entity,
    pub ui_child: (Option<usize>, Entity),
}

impl GetExpressionHandlers<UiIfElse, ExpressionArray> for UiIfElse {
    fn setup_expression_handlers(
        &self,
        root: &mut bevy::ecs::system::EntityCommands,
        target: Entity,
        e: ExpressionArray,
    ) {
        let reactive_handler = UiIfElseExpressionHandler::new(target, &e, &());
        root.with_children(|p| {
            p.spawn((
                reactive_handler,
                NodeBundle {
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                Name::new(format!("Conditional Expression: {e:?}")),
            ));
        });
    }
}

pub type UiIfElseExpressionHandler =
    ReactiveExpressionHandler<Option<usize>, ExpressionArray, UiIfElse, 0, ()>;

impl ComponentExpressionHandler<UiIfElse, ()> for UiIfElseExpressionHandler {
    fn get_source_entity(&self) -> Entity {
        self.entity
    }

    fn conditional_update<T: crate::UIState>(
        &mut self,
        c: &mut UiIfElse,
        state: &T,
        engine: &ExpressionEngine<T>,
        _added_data: (),
    ) {
        info!("Checking for change...");
        if let Some(nv) = self.internal_conditional_update(state, c.current_condition, engine) {
            info!("If Else Value Updated to {nv:?}");
            c.current_condition = nv;
        } else {
            info!("Current Val: {:?}", c.current_condition);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn ui_if_else_changed<T: UIState>(
    mut if_else_query: Query<(Entity, &mut UiIfElse), Changed<UiIfElse>>,
    mut commands: Commands,
    assets: Res<Assets<UiNodeTree>>,
    ui: Query<&T>,
    handle: Option<Res<UiHandle<T>>>,
    asset_server: Res<AssetServer>,
    parents: Query<&Children, With<Node>>,
    engine: Res<ExpressionEngine<T>>,
) {
    if if_else_query.is_empty() {
        return;
    }
    info!("If Else Changed!");
    let Some(handle) = handle else { return; };
    let Some(tree) = assets.get(&handle.handle) else { return; };

    info!("Grabbed Assets");

    for (entity, mut if_else) in if_else_query.iter_mut() {
        let (value, ui_child) = if_else.ui_child;

        if value == if_else.current_condition {
            info!("If Else Updated Already");
            return;
        }

        let Ok(ui_parent_children) = parents.get(if_else.ui_parent) else { continue;};

        let Ok(state) = ui.get(if_else.data_root) else { continue; };

        let new_ui_child = commands.spawn_empty().id();

        commands.entity(ui_child).despawn_recursive();
        commands.entity(entity).despawn_descendants();

        let children = ui_parent_children
            .into_iter()
            .map(|v| if *v == ui_child { new_ui_child } else { *v })
            .collect::<Vec<_>>();
        commands
            .entity(if_else.ui_parent)
            .replace_children(&children);

        if_else.ui_child = (if_else.current_condition, new_ui_child);

        let child = match if_else.current_condition {
            Some(id) => if_else.child_options.get(id).cloned(),
            None => if_else.child_options.get(if_else.num_conditions).cloned(),
        };

        let ui_child = match child {
            Some(child) => {
                if let Some(node) = tree.0.get(child) {
                    Some(spawn_ui(
                        (entity, new_ui_child, Some(if_else.ui_parent)),
                        &mut commands,
                        node,
                        state,
                        &asset_server,
                        tree,
                        if_else.data_root,
                        &engine,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        };

        if ui_child.is_none() {
            spawn_ui(
                (entity, new_ui_child, Some(if_else.ui_parent)),
                &mut commands,
                &UiNode::Empty,
                state,
                &asset_server,
                tree,
                if_else.data_root,
                &engine,
            );
        }
    }
}
