use bevy::prelude::*;

use crate::{
    reactive_expression_handlers::{ComponentExpressionHandler, ReactiveExpressionHandler},
    ExpressionValue,
};

// #[derive(Component)]
// pub struct UiMatch {
//     pub condition_value: ExpressionValue,
//     pub ui_parent: Entity,
// }

// #[derive(Component)]
// pub struct UiMatchCondition {
//     pub current_value: ExpressionValue,
//     pub spawned_ui: Option<Entity>,
// }

// pub type UiMatchExpressionHandler = ReactiveExpressionHandler<String, >

// impl ComponentExpressionHandler<UiMatch, ()> for
