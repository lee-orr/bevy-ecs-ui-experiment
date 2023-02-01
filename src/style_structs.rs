use std::marker::PhantomData;
use std::hash::Hash;

use bevy::{ui::BackgroundColor, prelude::{Component, Color, Handle}, text::TextStyle};

pub trait StyleElementWrapper: Clone {

    type StyleComponent;
    
    fn apply(&self, style: Self::StyleComponent) -> Self::StyleComponent;
    fn wrap(&self) -> Styling<Self::StyleComponent, Self> where Self: Sized {
        Styling {
            inner: self.clone(),
            _phantom: PhantomData
        }
    }
}

#[derive(Component, PartialEq, Eq, Hash)]
pub struct Styling<StyleComponent, StyleElement> {
    inner: StyleElement,
    _phantom: PhantomData<StyleComponent>
}

#[derive(Clone)]
pub struct BgColor(pub Color);

#[derive(Clone)]
pub struct TextColor(pub Color);

#[derive(Clone)]
pub struct Font(pub Handle<bevy::prelude::Font>);

#[derive(Clone)]
pub struct FontSize(pub f32);

impl StyleElementWrapper for BgColor{
    type StyleComponent = BackgroundColor;


    fn apply(&self, mut style: BackgroundColor) -> BackgroundColor {
        style.0 = self.0;
        style
    }
}

impl StyleElementWrapper for TextColor{
    type StyleComponent = TextStyle;
    fn apply(&self, mut style: TextStyle) -> TextStyle {
        style.color = self.0;
        style
    }
}

impl StyleElementWrapper for Font{
    type StyleComponent = TextStyle;
    fn apply(&self, mut style: TextStyle) -> TextStyle {
        style.font = self.0.clone();
        style
    }
}

impl StyleElementWrapper for FontSize{
    type StyleComponent = TextStyle;
    fn apply(&self, mut style: TextStyle) -> TextStyle {
        style.font_size = self.0;
        style
    }
}