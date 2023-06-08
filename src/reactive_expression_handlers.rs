use std::marker::PhantomData;

use bevy::{prelude::*, text::Text, ui::UiImage, utils::HashMap};
use bevy_ecss::Class;

use crate::{string_expression::StringExpression, Expression, UIState};

pub struct ReactiveExpressionHandler<V: PartialEq + Clone, E: Expression<V>> {
    pub expression: E,
    pub current: Option<V>,
}

impl<E: Expression<V>, V: PartialEq + Clone> From<&E> for ReactiveExpressionHandler<V, E> {
    fn from(value: &E) -> Self {
        Self {
            expression: value.clone(),
            current: None,
        }
    }
}

impl<V: PartialEq + Clone, E: Expression<V>> ReactiveExpressionHandler<V, E> {
    pub fn conditional_update<T: UIState>(&mut self, state: &T) -> Option<V> {
        let new_val = self.expression.process(state);
        if let Some(current) = &self.current {
            if &new_val != current {
                self.current = Some(new_val.clone());
                Some(new_val)
            } else {
                None
            }
        } else {
            self.current = Some(new_val.clone());
            Some(new_val)
        }
    }
}

#[derive(Component)]
pub struct ReactiveComponentExpressionHandler<C: Component, V: PartialEq + Clone, E: Expression<V>>
{
    pub source_entity: Entity,
    pub expressions: HashMap<String, ReactiveExpressionHandler<V, E>>,
    phantom: PhantomData<C>,
}

impl<C: Component, V: PartialEq + Clone, E: Expression<V>>
    ReactiveComponentExpressionHandler<C, V, E>
{
    pub fn new<S: ToString>(root: Entity, expressions: &[(S, &E)]) -> Self {
        Self {
            source_entity: root,
            expressions: expressions
                .iter()
                .map(|(k, e)| (k.to_string(), ReactiveExpressionHandler::<V, E>::from(*e)))
                .collect(),
            phantom: Default::default(),
        }
    }
}

pub trait ComponentExpressionHandler<C: Component, D> {
    fn get_source_entity(&self) -> Entity;
    fn conditional_update<T: UIState>(&mut self, c: &C, state: &T, added_data: D) -> Option<C>;
}

impl ComponentExpressionHandler<Name, ()>
    for ReactiveComponentExpressionHandler<Name, String, StringExpression>
{
    fn conditional_update<T: UIState>(
        &mut self,
        _c: &Name,
        state: &T,
        _added_data: (),
    ) -> Option<Name> {
        let Some(handler) = self.expressions.get_mut("name") else { return None; };
        handler.conditional_update(state).map(Name::new)
    }

    fn get_source_entity(&self) -> Entity {
        self.source_entity
    }
}

impl ComponentExpressionHandler<Class, ()>
    for ReactiveComponentExpressionHandler<Class, String, StringExpression>
{
    fn conditional_update<T: UIState>(
        &mut self,
        _c: &Class,
        state: &T,
        _added_data: (),
    ) -> Option<Class> {
        let Some(handler) = self.expressions.get_mut("class") else { return None; };
        handler.conditional_update(state).map(Class::new)
    }

    fn get_source_entity(&self) -> Entity {
        self.source_entity
    }
}

impl ComponentExpressionHandler<Text, ()>
    for ReactiveComponentExpressionHandler<Text, String, StringExpression>
{
    fn conditional_update<T: UIState>(
        &mut self,
        text: &Text,
        state: &T,
        _added_data: (),
    ) -> Option<Text> {
        let Some(handler) = self.expressions.get_mut("text") else { return None; };
        handler.conditional_update(state).map(|v| {
            Text::from_section(
                v,
                text.sections
                    .get(0)
                    .map(|v| v.style.clone())
                    .unwrap_or_default(),
            )
        })
    }

    fn get_source_entity(&self) -> Entity {
        self.source_entity
    }
}

impl ComponentExpressionHandler<UiImage, &AssetServer>
    for ReactiveComponentExpressionHandler<UiImage, String, StringExpression>
{
    fn conditional_update<T: UIState>(
        &mut self,
        img: &UiImage,
        state: &T,
        added_data: &AssetServer,
    ) -> Option<UiImage> {
        let Some(handler) = self.expressions.get_mut("src") else { return None; };
        handler
            .conditional_update(state)
            .map(|v| added_data.load(v))
            .map(|v| UiImage {
                texture: v,
                flip_x: img.flip_x,
                flip_y: img.flip_y,
            })
    }

    fn get_source_entity(&self) -> Entity {
        self.source_entity
    }
}

fn component_expression_change_handler<
    T: UIState,
    C: Component,
    H: ComponentExpressionHandler<C, ()> + Component,
>(
    commands: Commands,
    roots: Query<&T, Changed<T>>,
    components: Query<(Entity, &mut H, &C)>,
) {
    component_expression_change_handler_with_added_data::<T, C, (), H>(
        commands,
        roots,
        components,
        (),
    );
}

fn component_expression_change_handler_with_added_data<
    T: UIState,
    C: Component,
    R: Copy,
    H: ComponentExpressionHandler<C, R> + Component,
>(
    mut commands: Commands,
    roots: Query<&T, Changed<T>>,
    mut components: Query<(Entity, &mut H, &C)>,
    resource: R,
) {
    for (entity, mut reactive, component) in components.iter_mut() {
        let Ok(state) = roots.get(reactive.get_source_entity()) else { continue; };
        let Some(c) = reactive.conditional_update(component, state, resource) else { continue; };
        info!("Updating");
        commands.entity(entity).insert(c);
    }
}

fn image_url_change_handler<T: UIState>(
    commands: Commands,
    roots: Query<&T, Changed<T>>,
    components: Query<(
        Entity,
        &mut ReactiveComponentExpressionHandler<UiImage, String, StringExpression>,
        &UiImage,
    )>,
    resource: Res<AssetServer>,
) {
    component_expression_change_handler_with_added_data::<
        T,
        UiImage,
        &AssetServer,
        ReactiveComponentExpressionHandler<_, String, _>,
    >(commands, roots, components, resource.as_ref());
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
            component_expression_change_handler::<
                T,
                Name,
                ReactiveComponentExpressionHandler<_, String, StringExpression>,
            >
                .in_base_set(CoreSet::PostUpdate),
        )
        .add_system(
            component_expression_change_handler::<
                T,
                Class,
                ReactiveComponentExpressionHandler<_, String, StringExpression>,
            >
                .in_base_set(CoreSet::PostUpdate),
        )
        .add_system(
            component_expression_change_handler::<
                T,
                Text,
                ReactiveComponentExpressionHandler<_, String, StringExpression>,
            >
                .in_base_set(CoreSet::PostUpdate),
        )
        .add_system(image_url_change_handler::<T>.in_base_set(CoreSet::PostUpdate));
    }
}
