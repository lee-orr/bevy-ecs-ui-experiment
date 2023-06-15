use crate::ui_plugin::UiHandle;

use crate::ui_asset::UiNodeTree;

use crate::UIState;

use crate::reactive_expression_handlers::ComponentExpressionHandler;

use crate::reactive_expression_handlers::ReactiveExpressionHandler;

use bevy::prelude::*;

use crate::ArrayExpression;

use crate::reactive_expression_handlers::GetExpressionHandlers;

#[derive(Component)]
pub struct UiFor {
    pub current_condition: Option<usize>,
    pub num_conditions: usize,
    pub child_options: Vec<usize>,
    pub data_root: Entity,
    pub ui_parent: Entity,
    pub ui_child: (Option<usize>, Entity),
}

impl GetExpressionHandlers<UiFor, ArrayExpression> for UiFor {
    fn setup_expression_handlers(
        &self,
        root: &mut bevy::ecs::system::EntityCommands,
        target: Entity,
        e: ArrayExpression,
    ) {
        let reactive_handler = UiForExpressionHandler::new(target, &e, &());
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

pub type UiForExpressionHandler =
    ReactiveExpressionHandler<Option<usize>, ArrayExpression, UiFor, 0, ()>;

impl ComponentExpressionHandler<UiFor, ()> for UiForExpressionHandler {
    fn get_source_entity(&self) -> Entity {
        self.entity
    }

    fn conditional_update<T: crate::UIState>(
        &mut self,
        _c: &mut UiFor,
        _state: &T,
        _added_data: (),
    ) {
        todo!()
    }
}

pub fn ui_for_changed<T: UIState>(
    _for_query: Query<(Entity, &mut UiFor), Changed<UiFor>>,
    _commands: Commands,
    _assets: Res<Assets<UiNodeTree>>,
    _ui: Query<&T>,
    _handle: Option<Res<UiHandle<T>>>,
    _asset_server: Res<AssetServer>,
    _parents: Query<&Children, With<Node>>,
) {
    todo!()
    // if for_query.is_empty() {
    //     return;
    // }
    // info!("If Else Changed!");
    // let Some(handle) = handle else { return; };
    // let Some(tree) = assets.get(&handle.handle) else { return; };

    // info!("Grabbed Assets");

    // for (entity, mut if_else) in for_query.iter_mut() {
    //     let (value, ui_child) = if_else.ui_child;

    //     if value == if_else.current_condition {
    //         info!("If Else Updated Already");
    //         return;
    //     }

    //     let Ok(ui_parent_children) = parents.get(if_else.ui_parent) else { continue;};

    //     let Ok(state) = ui.get(if_else.data_root) else { continue; };

    //     let new_ui_child = commands.spawn_empty().id();

    //     commands.entity(ui_child).despawn_recursive();
    //     commands.entity(entity).despawn_descendants();

    //     let children = ui_parent_children
    //         .into_iter()
    //         .map(|v| if *v == ui_child { new_ui_child } else { *v })
    //         .collect::<Vec<_>>();
    //     commands
    //         .entity(if_else.ui_parent)
    //         .replace_children(&children);

    //     if_else.ui_child = (if_else.current_condition, new_ui_child);

    //     let child = match if_else.current_condition {
    //         Some(id) => if_else.child_options.get(id).cloned(),
    //         None => if_else.child_options.get(if_else.num_conditions).cloned(),
    //     };

    //     let ui_child = match child {
    //         Some(child) => {
    //             if let Some(node) = tree.0.get(child) {
    //                 Some(spawn_ui(
    //                     (entity, new_ui_child, Some(if_else.ui_parent)),
    //                     &mut commands,
    //                     node,
    //                     state,
    //                     &asset_server,
    //                     tree,
    //                     if_else.data_root,
    //                 ))
    //             } else {
    //                 None
    //             }
    //         }
    //         _ => None,
    //     };

    //     if ui_child.is_none() {
    //         spawn_ui(
    //             (entity, new_ui_child, Some(if_else.ui_parent)),
    //             &mut commands,
    //             &UiNode::Empty,
    //             state,
    //             &asset_server,
    //             tree,
    //             if_else.data_root,
    //         );
    //     }
    // }
}
