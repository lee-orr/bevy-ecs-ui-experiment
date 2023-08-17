use std::sync::Arc;

use crate::*;

pub trait ExternalUiSpawner<'w, 's, St: Styler> {
    type InternalSpawner: InternalUiSpawner<'w, 's>;

    fn get_spawner(&mut self) -> &mut Self::InternalSpawner;
    fn get_styler(&self) -> Arc<St>;

    fn node<'a>(
        &'a mut self,
    ) -> UiComponent<'w, 's, 'a, UiNodeBundle, Self::InternalSpawner, St, usize> {
        let styler = self.get_styler();
        UiComponent::new(UiNodeBundle::default(), self.get_spawner(), styler)
    }

    fn text<'a>(
        &'a mut self,
        text: impl Into<String>,
    ) -> UiComponent<'w, 's, 'a, UiTextBundle, Self::InternalSpawner, St, usize> {
        let styler = self.get_styler();
        UiComponent::new(
            UiTextBundle {
                node_bundle: TextBundle {
                    text: Text::from_section(text, TextStyle::default()),
                    ..default()
                },
                ..Default::default()
            },
            self.get_spawner(),
            styler,
        )
    }

    fn raw_text<'a>(
        &'a mut self,
    ) -> UiComponent<'w, 's, 'a, UiTextBundle, Self::InternalSpawner, St, usize> {
        let styler = self.get_styler();
        UiComponent::new(UiTextBundle::default(), self.get_spawner(), styler)
    }

    fn image<'a>(
        &'a mut self,
        image: Handle<Image>,
    ) -> UiComponent<'w, 's, 'a, UiImageBundle, Self::InternalSpawner, St, usize> {
        let styler = self.get_styler();
        UiComponent::new(
            UiImageBundle {
                node_bundle: ImageBundle {
                    image: UiImage {
                        texture: image,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            self.get_spawner(),
            styler,
        )
    }

    fn button<'a>(
        &'a mut self,
    ) -> UiComponent<'w, 's, 'a, ButtonBundle, Self::InternalSpawner, St, usize> {
        let styler = self.get_styler();
        UiComponent::new(ButtonBundle::default(), self.get_spawner(), styler)
    }
}

impl<'w, 's> ExternalUiSpawner<'w, 's, NullStyler> for Commands<'w, 's> {
    type InternalSpawner = Self;

    fn get_spawner(&mut self) -> &mut Self::InternalSpawner {
        self
    }

    fn get_styler(&self) -> Arc<NullStyler> {
        Arc::new(NullStyler)
    }
}

impl<'w, 's, 'l, St: Styler> ExternalUiSpawner<'w, 's, St>
    for (&mut ChildBuilder<'w, 's, 'l>, Arc<St>)
{
    type InternalSpawner = ChildBuilder<'w, 's, 'l>;

    fn get_spawner(&mut self) -> &mut Self::InternalSpawner {
        self.0
    }

    fn get_styler(&self) -> Arc<St> {
        self.1.clone()
    }
}
