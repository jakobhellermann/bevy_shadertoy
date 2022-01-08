use std::num::NonZeroU64;

use bevy::prelude::*;
use bevy::render::render_resource::std140::AsStd140;
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;

use crate::shadertoy_bindings::ShadertoyBindings;
use crate::FULLSCREEN_TRIANGLE_SHADER;

pub struct ShadertoyPipeline {
    pub shader: Handle<Shader>,
    pub shadertoy_bindings: Buffer,
    pub shadertoy_bindings_bind_group: BindGroup,
    pub shadertoy_bindings_bind_group_layout: BindGroupLayout,
}

impl ShadertoyPipeline {
    pub fn new(render_world: &mut World, shader: Handle<Shader>) -> Self {
        let render_device = render_world.get_resource::<RenderDevice>().unwrap();

        let shadertoy_bindings_bind_group_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(
                            NonZeroU64::new(ShadertoyBindings::std140_size_static() as u64)
                                .unwrap(),
                        ),
                    },
                    count: None,
                }],
            });

        let shadertoy_bindings = render_device.create_buffer(&BufferDescriptor {
            label: None,
            size: ShadertoyBindings::std140_size_static() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shadertoy_bindings_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &shadertoy_bindings_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: shadertoy_bindings.as_entire_binding(),
            }],
        });

        ShadertoyPipeline {
            shader,
            shadertoy_bindings,
            shadertoy_bindings_bind_group,
            shadertoy_bindings_bind_group_layout,
        }
    }
}

impl SpecializedPipeline for ShadertoyPipeline {
    type Key = ();

    fn specialize(&self, _: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: None,
            layout: Some(vec![self.shadertoy_bindings_bind_group_layout.clone()]),
            vertex: VertexState {
                shader: FULLSCREEN_TRIANGLE_SHADER.typed(),
                shader_defs: vec![],
                entry_point: "vs_main".into(),
                buffers: vec![],
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone_weak(),
                shader_defs: vec![],
                entry_point: "main".into(),
                targets: vec![TextureFormat::bevy_default().into()],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
        }
    }
}

pub struct CompiledShadertoyPipeline(pub CachedPipelineId);

pub fn compile_pipeline(
    mut commands: Commands,
    pipeline: Res<ShadertoyPipeline>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
) {
    let pipeline = pipeline_cache.queue(pipeline.specialize(()));
    commands.insert_resource(CompiledShadertoyPipeline(pipeline));
}
