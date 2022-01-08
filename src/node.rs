use bevy::prelude::*;
use bevy::render::render_graph::{Node, RenderGraphContext};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::ExtractedWindows;
use bevy::window::WindowId;

use crate::pipeline::{CompiledShadertoyPipeline, ShadertoyPipeline};

pub struct ShadertoyNode;

impl Node for ShadertoyNode {
    fn run(
        &self,
        _: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let window = world
            .get_resource::<ExtractedWindows>()
            .unwrap()
            .get(&WindowId::primary())
            .unwrap();

        let shadertoy_pipeline = world.get_resource::<ShadertoyPipeline>().unwrap();

        let pipeline_cache = world.get_resource::<RenderPipelineCache>().unwrap();
        let pipeline = world.get_resource::<CompiledShadertoyPipeline>().unwrap().0;

        let pipeline = match pipeline_cache.get(pipeline) {
            Some(pipeline) => pipeline,
            None => return Ok(()),
        };

        let view = match &window.swap_chain_texture {
            Some(view) => view,
            None => return Ok(()),
        };

        let mut pass = render_context
            .command_encoder
            .begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Default::default()),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &shadertoy_pipeline.shadertoy_bindings_bind_group, &[]);
        pass.draw(0..3, 0..1);

        Ok(())
    }
}
