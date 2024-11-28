use bevy::{
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin},
        render_graph::RenderLabel,
        render_resource::ShaderType,
        RenderApp,
    },
};

// struct PostProcessPlugin2d;
//
// #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
// struct PostProcessLabel2d;
//
// // The post process node used for the render graph
// #[derive(Default)]
// struct PostProcessNode2d;

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct PostProcessSettings {
    intensity: f32,
    // WebGL2 structs must be 16 byte aligned.
    #[cfg(feature = "webgl2")]
    _webgl2_padding: Vec3,
}

pub struct PostProcessPlugin {}

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<PostProcessSettings>::default(),
            UniformComponentPlugin::<PostProcessSettings>::default(),
        ));
    }

    fn finish(&self, app: &mut App) {}
}
