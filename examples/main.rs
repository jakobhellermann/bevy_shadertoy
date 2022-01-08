use bevy::prelude::*;
use bevy_shadertoy::ShadertoyPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 450.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShadertoyPlugin::new(|asset_server| {
            asset_server.load("uv.frag")
        }))
        .run();
}
