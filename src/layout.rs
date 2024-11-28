use bevy::prelude::*;

#[derive(Component, Clone, Debug, Default, Deref, DerefMut, Reflect)]
#[require()]
struct CustText2d {
    text: String,
}
