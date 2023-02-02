use bevy::prelude::*;
use tuple_utils::*;

use crate::style_structs::{BgColor, StyleElementWrapper, Styling};

#[derive(Bundle)]
pub struct UiBundle<Element: Component + Clone, StyleBundle: Bundle> {
    value: Element,
    style: StyleBundle
}

pub trait UiBundleGenerator<Element: Component + Clone> {
    fn ui(self) -> UiBundle<Element, ()>;
}

impl<Element: Component + Clone> UiBundleGenerator<Element> for Element {
    fn ui(self) -> UiBundle<Element, ()> {
        UiBundle { value: self, style: () }
    }
}

impl<Element: Component + Clone, StyleBundle: Bundle> UiBundle<Element, StyleBundle> {
    pub fn background(self, c: Color) -> UiBundle<Element, (Styling<BackgroundColor, BgColor>, StyleBundle)> {
        let t = BgColor(c).wrap();
        UiBundle { value: self.value, style: (t, self.style) }
    }
}