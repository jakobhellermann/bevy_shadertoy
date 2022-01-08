use bevy::ecs::system::Resource;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_graph::RenderGraph;
use bevy::render::{RenderApp, RenderStage};
use node::ShadertoyNode;
use pipeline::ShadertoyPipeline;

pub const FULLSCREEN_TRIANGLE_SHADER: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 5055891631612190481);

pub const SHADERTOY_BINDINGS: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 993833722855091815);
pub const SHADERTOY_MAIN: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8820494265073802242);

mod node;
mod pipeline;
mod shadertoy_bindings;

pub struct ShadertoyPlugin<F> {
    shader: F,
}

impl<F: Fn(&mut AssetServer) -> Handle<Shader>> ShadertoyPlugin<F> {
    pub fn new(shader: F) -> Self {
        ShadertoyPlugin { shader }
    }
}

impl<F: Fn(&mut AssetServer) -> Handle<Shader> + Send + Sync + 'static> Plugin
    for ShadertoyPlugin<F>
{
    fn build(&self, app: &mut App) {
        let mut asset_server = app.world.get_resource_mut::<AssetServer>().unwrap();
        let shader = (self.shader)(&mut asset_server);

        app.init_resource::<shadertoy_bindings::MouseState>()
            .add_system(shadertoy_bindings::mouse_state);

        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        setup_shaders(&mut shaders);

        let render_app = app.sub_app_mut(RenderApp);

        let pipeline = ShadertoyPipeline::new(&mut render_app.world, shader);
        render_app
            .insert_resource(pipeline)
            .add_system_to_stage(
                RenderStage::Extract,
                extract_resource::<shadertoy_bindings::MouseState>,
            )
            .add_system_to_stage(RenderStage::Extract, shadertoy_bindings::extract_time)
            .add_system_to_stage(RenderStage::Prepare, pipeline::compile_pipeline)
            .add_system_to_stage(
                RenderStage::Prepare,
                shadertoy_bindings::write_shadertoy_buffer,
            );

        let mut render_graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();
        render_graph.add_node("shadertoy", ShadertoyNode);
    }
}

fn setup_shaders(shaders: &mut Assets<Shader>) {
    shaders.set_untracked(
        FULLSCREEN_TRIANGLE_SHADER.typed::<Shader>(),
        Shader::from_wgsl(include_str!("fullscreen_triangle.wgsl")),
    );
    shaders.set_untracked(
        SHADERTOY_BINDINGS.typed::<Shader>(),
        Shader::from_glsl(
            include_str!("shader/shadertoy_bindings.frag"),
            naga::ShaderStage::Vertex,
        )
        .with_import_path("bevy_shadertoy::bindings"),
    );
    shaders.set_untracked(
        SHADERTOY_MAIN.typed::<Shader>(),
        Shader::from_glsl(
            include_str!("shader/shadertoy_main.frag"),
            naga::ShaderStage::Vertex,
        )
        .with_import_path("bevy_shadertoy::main"),
    );
}

fn extract_resource<T: Resource + Clone>(mut commands: Commands, res: Res<T>) {
    commands.insert_resource(res.into_inner().clone());
}
