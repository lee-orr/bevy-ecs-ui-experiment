use std::marker::PhantomData;

use bevy::{ecs::system::EntityCommands, prelude::*, text::Text, ui::UiImage};
use bevy_ecss::Class;

use crate::{string_expression::StringExpression, Expression, UIState};

#[derive(Component)]
pub struct ReactiveExpressionHandler<
    V: PartialEq + Clone,
    E: Expression<V>,
    C: Component,
    const FIELD_ID: usize = 0,
    CachedCurrent: PartialEq + Clone = (),
> {
    pub expression: E,
    pub entity: Entity,
    pub cached_current: CachedCurrent,
    phantom: PhantomData<(C, V)>,
}

impl<
        V: PartialEq + Clone,
        E: Expression<V>,
        C: Component,
        const FIELD_ID: usize,
        CachedCurrent: PartialEq + Clone,
    > ReactiveExpressionHandler<V, E, C, FIELD_ID, CachedCurrent>
{
    pub fn new(entity: Entity, expression: &E, cache: &CachedCurrent) -> Self {
        Self {
            expression: expression.clone(),
            entity,
            cached_current: cache.clone(),
            phantom: default(),
        }
    }

    pub fn internal_conditional_update<T: UIState>(
        &mut self,
        state: &T,
        current: impl PartialEq<V>,
    ) -> Option<V> {
        let new_val = self.expression.process(state);
        if !current.eq(&new_val) {
            info!("process found: new val");
            Some(new_val)
        } else {
            info!("process found: old val");
            None
        }
    }
}

pub trait ComponentExpressionHandler<C: Component, D> {
    fn get_source_entity(&self) -> Entity;
    fn conditional_update<T: UIState>(&mut self, c: &mut C, state: &T, added_data: D);
}

pub trait GetExpressionHandlers<C: Component, Expressions> {
    fn setup_expression_handlers(&self, root: &mut EntityCommands, target: Entity, e: Expressions);
}

pub trait GetCachedExpressionHandlers<C: Component, Expressions, CachedCurrent> {
    fn setup_cached_expression_handlers(
        &self,
        root: &mut EntityCommands,
        target: Entity,
        e: Expressions,
        cached: CachedCurrent,
    );
}

fn component_expression_change_handler<
    T: UIState,
    C: Component,
    H: ComponentExpressionHandler<C, ()> + Component,
>(
    roots: Query<(&T, &Children), Changed<T>>,
    reactive: Query<&mut H>,
    components: Query<&mut C>,
) {
    component_expression_change_handler_with_added_data::<T, C, (), H>(
        roots,
        reactive,
        components,
        (),
    );
}

fn component_expression_change_handler_with_resource<
    T: UIState,
    C: Component,
    R: Resource,
    H: for<'a> ComponentExpressionHandler<C, &'a R> + Component,
>(
    roots: Query<(&T, &Children), Changed<T>>,
    reactive: Query<&mut H>,
    components: Query<&mut C>,
    resource: Res<R>,
) {
    component_expression_change_handler_with_added_data::<T, C, &R, H>(
        roots,
        reactive,
        components,
        resource.as_ref(),
    );
}

fn component_expression_change_handler_with_added_data<
    T: UIState,
    C: Component,
    R: Copy,
    H: ComponentExpressionHandler<C, R> + Component,
>(
    roots: Query<(&T, &Children), Changed<T>>,
    mut reactive: Query<&mut H>,
    mut components: Query<&mut C>,
    resource: R,
) {
    for (state, children) in roots.iter() {
        for child in children.iter() {
            let Ok(mut reactive) = reactive.get_mut(*child) else { continue;};
            let Ok(mut c) = components.get_mut(reactive.get_source_entity()) else { continue; };
            reactive.conditional_update(&mut c, state, resource);
        }
    }
}

pub struct ReactiveExpressionPlugin<T: UIState>(PhantomData<T>);

impl<T: UIState> Default for ReactiveExpressionPlugin<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: UIState> Plugin for ReactiveExpressionPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(
            component_expression_change_handler::<T, Name, NameExpressionHandler>
                .in_base_set(CoreSet::PostUpdate),
        )
        .add_system(
            component_expression_change_handler::<T, Class, ClassExpressionHandler>
                .in_base_set(CoreSet::PostUpdate),
        )
        .add_system(
            component_expression_change_handler::<T, Text, TextExpressionHandler>
                .in_base_set(CoreSet::PostUpdate),
        )
        .add_system(
            component_expression_change_handler_with_resource::<
                T,
                UiImage,
                AssetServer,
                UiImageExpressionHandler,
            >
                .in_base_set(CoreSet::PostUpdate),
        );
    }
}

type NameExpressionHandler = ReactiveExpressionHandler<String, StringExpression, Name, 0>;

impl GetExpressionHandlers<Name, StringExpression> for Name {
    fn setup_expression_handlers(
        &self,
        root: &mut EntityCommands,
        target: Entity,
        e: StringExpression,
    ) {
        if !matches!(e, StringExpression::Value(_)) {
            let reactive_handler = NameExpressionHandler::new(target, &e, &());
            root.with_children(|p| {
                p.spawn((
                    reactive_handler,
                    NodeBundle {
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    },
                ));
            });
        }
    }
}

impl ComponentExpressionHandler<Name, ()> for NameExpressionHandler {
    fn get_source_entity(&self) -> Entity {
        self.entity
    }

    fn conditional_update<T: UIState>(&mut self, c: &mut Name, state: &T, _added_data: ()) {
        if let Some(nv) = self.internal_conditional_update(state, c.as_str()) {
            c.set(nv);
        }
    }
}

type ClassExpressionHandler = ReactiveExpressionHandler<String, StringExpression, Class, 0>;

impl GetExpressionHandlers<Class, StringExpression> for Class {
    fn setup_expression_handlers(
        &self,
        root: &mut EntityCommands,
        target: Entity,
        e: StringExpression,
    ) {
        if !matches!(e, StringExpression::Value(_)) {
            let reactive_handler = ClassExpressionHandler::new(target, &e, &());
            root.with_children(|p| {
                p.spawn((
                    reactive_handler,
                    NodeBundle {
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    },
                ));
            });
        }
    }
}

impl ComponentExpressionHandler<Class, ()> for ClassExpressionHandler {
    fn get_source_entity(&self) -> Entity {
        self.entity
    }

    fn conditional_update<T: UIState>(&mut self, c: &mut Class, state: &T, _added_data: ()) {
        if let Some(nv) = self.internal_conditional_update(state, c.to_string()) {
            *c = Class::new(nv);
        }
    }
}

type TextExpressionHandler = ReactiveExpressionHandler<String, StringExpression, Text, 0>;

impl GetExpressionHandlers<Text, StringExpression> for Text {
    fn setup_expression_handlers(
        &self,
        root: &mut EntityCommands,
        target: Entity,
        e: StringExpression,
    ) {
        if !matches!(e, StringExpression::Value(_)) {
            let reactive_handler = TextExpressionHandler::new(target, &e, &());
            root.with_children(|p| {
                p.spawn((
                    reactive_handler,
                    NodeBundle {
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    },
                ));
            });
        }
    }
}

impl ComponentExpressionHandler<Text, ()> for TextExpressionHandler {
    fn get_source_entity(&self) -> Entity {
        self.entity
    }

    fn conditional_update<T: UIState>(&mut self, c: &mut Text, state: &T, _added_data: ()) {
        let current = c
            .sections
            .iter()
            .map(|c| c.value.clone())
            .collect::<Vec<_>>()
            .join("");
        println!("Current Text: {current}");
        if let Some(nv) = self.internal_conditional_update(state, current) {
            println!("Processed Text: {nv}");
            let style = c
                .sections
                .get(0)
                .map(|sec| sec.style.clone())
                .unwrap_or_default();
            *c = Text::from_section(nv, style);
        }
    }
}

type UiImageExpressionHandler =
    ReactiveExpressionHandler<String, StringExpression, UiImage, 0, String>;

impl GetCachedExpressionHandlers<UiImage, StringExpression, String> for UiImage {
    fn setup_cached_expression_handlers(
        &self,
        root: &mut EntityCommands,
        target: Entity,
        e: StringExpression,
        cached: String,
    ) {
        if !matches!(e, StringExpression::Value(_)) {
            let reactive_handler = UiImageExpressionHandler::new(target, &e, &cached);
            root.with_children(|p| {
                p.spawn((
                    reactive_handler,
                    NodeBundle {
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    },
                ));
            });
        }
    }
}

impl ComponentExpressionHandler<UiImage, &AssetServer> for UiImageExpressionHandler {
    fn get_source_entity(&self) -> Entity {
        self.entity
    }

    fn conditional_update<T: UIState>(
        &mut self,
        c: &mut UiImage,
        state: &T,
        added_data: &AssetServer,
    ) {
        let current = self.cached_current.clone();
        if let Some(nv) = self.internal_conditional_update(state, current) {
            self.cached_current = nv.clone();
            c.texture = added_data.load(nv);
        }
    }
}
