use bevy::{
    prelude::*,
    sprite::{Anchor, SpriteSource},
    text::{TextBounds, TextRoot, TextSpanAccess},
};

pub mod layout;
pub mod postproc_shader;
pub mod postprocess_plugin;

#[derive(Component, Clone, Debug, Default, Deref, DerefMut, Reflect)]
#[reflect(Component, Default, Debug)]
#[require(
    TextLayout,
    TextFont,
    TextColor,
    TextBounds,
    Anchor,
    SpriteSource,
    Visibility,
    Transform
)]
pub struct CustText2d(pub String);

impl CustText2d {
    /// Makes a new 2d text component.
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}

impl TextRoot for CustText2d {}

impl TextSpanAccess for CustText2d {
    fn read_span(&self) -> &str {
        self.as_str()
    }
    fn write_span(&mut self) -> &mut String {
        &mut *self
    }
}

impl From<&str> for CustText2d {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<String> for CustText2d {
    fn from(value: String) -> Self {
        Self(value)
    }
}
