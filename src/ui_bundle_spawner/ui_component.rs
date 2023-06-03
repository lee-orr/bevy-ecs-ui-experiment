use bevy::ecs::system::EntityCommands;

use crate::style::StyleComponentApplier;
use crate::UiComponentSpawner;
use crate::UiComponentSpawnerActivator;
use crate::UiId;

use std::sync::Arc;

use std::marker::PhantomData;

use std::hash::Hash;

use std::fmt::Debug;

use crate::Styler;

use super::InternalUiSpawner;

use super::UiBundleGeneratorStyler;

use super::UiBundleGenerator;

pub struct UiComponent<
    'w,
    's,
    'a,
    T: UiBundleGenerator + UiBundleGeneratorStyler,
    S: InternalUiSpawner<'w, 's>,
    St: Styler,
    Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy + 'static,
> {
    pub value: T,
    pub(crate) spawner: Option<&'a mut S>,
    pub(crate) phantom: PhantomData<&'w T>,
    pub(crate) phantom_2: PhantomData<&'s T>,
    pub(crate) styler: Arc<St>,
    pub(crate) id: Option<Id>,
}

impl<
        'w,
        's,
        'a,
        Inner: Default,
        Bg: UiBundleGenerator + UiBundleGeneratorStyler + StyleComponentApplier<Inner>,
        S: InternalUiSpawner<'w, 's>,
        St: Styler,
        Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy,
    > StyleComponentApplier<Inner> for UiComponent<'w, 's, 'a, Bg, S, St, Id>
{
    fn get_component<T: FnMut(&mut Inner)>(mut self, apply: T) -> Self {
        let value = self.value.clone();
        self.value = value.get_component(apply);
        self
    }
}

impl<
        'w,
        's,
        'a,
        T: UiBundleGenerator + UiBundleGeneratorStyler,
        S: InternalUiSpawner<'w, 's>,
        St: Styler,
        Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy + 'static,
    > UiComponent<'w, 's, 'a, T, S, St, Id>
{
    pub fn new(value: T, spawner: &'a mut S, styler: Arc<St>) -> Self {
        let result = Self {
            value,
            spawner: Some(spawner),
            phantom: PhantomData,
            phantom_2: PhantomData,
            styler,
            id: None,
        };
        result.style_with_styler()
    }

    pub fn style<StB: Styler + 'static>(
        mut self,
        styler: StB,
    ) -> UiComponent<'w, 's, 'a, T, S, StB, Id> {
        let styler = styler;
        let styler: Arc<StB> = Arc::new(styler);
        let result = UiComponent {
            value: self.value.clone(),
            spawner: self.spawner.take(),
            phantom: PhantomData,
            phantom_2: PhantomData,
            styler,
            id: self.id,
        };
        result.style_with_styler()
    }

    pub fn id<IdB: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy + 'static>(
        mut self,
        id: IdB,
    ) -> UiComponent<'w, 's, 'a, T, S, St, IdB> {
        let id = id;
        UiComponent {
            value: self.value.clone(),
            spawner: self.spawner.take(),
            phantom: PhantomData,
            phantom_2: PhantomData,
            styler: self.styler.clone(),
            id: Some(id),
        }
    }

    pub(crate) fn style_with_styler(mut self) -> Self {
        self.value = self.value.clone().apply_styler(self.styler.as_ref());
        self
    }
}

impl<
        'w,
        's,
        'a,
        T: UiBundleGenerator + UiBundleGeneratorStyler,
        S: InternalUiSpawner<'w, 's>,
        St: Styler,
        Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy + 'static,
    > Drop for UiComponent<'w, 's, 'a, T, S, St, Id>
{
    fn drop(&mut self) {
        let id = self.id.take();
        let spawner = self.spawner.take();
        if let Some(spawner) = spawner {
            let mut result = spawner.spawn_ui_component(&self.value);
            if let Some(id) = id {
                result.insert(UiId::new(id));
            }
        }
    }
}

impl<
        'w,
        's,
        'a,
        T: UiBundleGenerator + UiBundleGeneratorStyler,
        S: InternalUiSpawner<'w, 's>,
        St: Styler,
        Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy,
    > UiComponentSpawner<T> for UiComponent<'w, 's, 'a, T, S, St, Id>
{
    fn update_value<U: FnMut(&mut T) -> &mut T>(mut self, mut updator: U) -> Self {
        updator(&mut self.value);
        self
    }
}

impl<
        'w,
        's,
        'a,
        T: UiBundleGenerator + UiBundleGeneratorStyler,
        S: InternalUiSpawner<'w, 's>,
        St: Styler,
        Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy + 'static,
    > UiComponentSpawnerActivator<'w, 's, 'a, T, S, St> for UiComponent<'w, 's, 'a, T, S, St, Id>
{
    fn spawn(mut self) -> Option<EntityCommands<'w, 's, 'a>> {
        let id = self.id.take();
        let spawner = self.spawner.take();
        spawner.map(|spawner| {
            let mut result = spawner.spawn_ui_component(&self.value);
            if let Some(id) = id {
                result.insert(UiId::new(id));
            }
            result
        })
    }

    fn get_component_styler(&self) -> Arc<St> {
        self.styler.clone()
    }

    fn apply_id(&self, commands: &mut EntityCommands) {
        if let Some(id) = self.id.as_ref() {
            commands.insert(UiId::new(*id));
        }
    }
}
