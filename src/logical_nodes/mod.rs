mod for_node;
mod if_node;

use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{reactive_expression_handlers::component_expression_change_handler, UIState};

pub use for_node::*;
pub use if_node::*;

pub struct LogicalNodesPlugin<T: UIState>(PhantomData<T>);

impl<T: UIState> Default for LogicalNodesPlugin<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: UIState> Plugin for LogicalNodesPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(if_node::ui_if_else_changed::<T>.in_base_set(CoreSet::PostUpdate))
            .add_system(
                component_expression_change_handler::<
                    T,
                    if_node::UiIfElse,
                    if_node::UiIfElseExpressionHandler,
                >,
            )
            .add_system(for_node::ui_for_changed::<T>.in_base_set(CoreSet::PostUpdate))
            .add_system(
                component_expression_change_handler::<
                    T,
                    for_node::UiFor,
                    for_node::UiForExpressionHandler,
                >,
            );
    }
}
