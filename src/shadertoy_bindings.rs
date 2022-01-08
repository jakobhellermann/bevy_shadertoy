use bevy::prelude::*;
use bevy::render::render_resource::std140::{AsStd140, Std140};
use bevy::render::renderer::RenderQueue;
use bevy::render::view::ExtractedWindows;
use bevy::window::WindowId;

use crate::pipeline::ShadertoyPipeline;

#[derive(AsStd140)]
pub struct ShadertoyBindings {
    resolution: Vec3,
    time: f32,
    mouse_state: Vec4,
    time_delta: f32,
}

pub struct ExtractedTime {
    seconds_since_startup: f32,
    delta: f32,
}

pub fn extract_time(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(ExtractedTime {
        seconds_since_startup: time.seconds_since_startup() as f32,
        delta: time.delta_seconds(),
    });
}

pub fn write_shadertoy_buffer(
    pipeline: Res<ShadertoyPipeline>,
    render_queue: Res<RenderQueue>,

    windows: Res<ExtractedWindows>,
    time: Res<ExtractedTime>,
    mouse_state: Res<MouseState>,
) {
    let window = windows.get(&WindowId::primary()).unwrap();

    let resolution = Vec3::new(
        window.physical_width as f32,
        window.physical_height as f32,
        0.0,
    );
    let mouse_state = Vec4::new(
        mouse_state.position.x.clamp(0.0, resolution.x),
        mouse_state.position.y.clamp(0.0, resolution.y),
        mouse_state.down as u8 as f32,
        mouse_state.just_down as u8 as f32,
    );
    let bindings = ShadertoyBindings {
        resolution,
        time: time.seconds_since_startup,
        mouse_state,
        time_delta: time.delta,
    };
    render_queue.write_buffer(
        &pipeline.shadertoy_bindings,
        0,
        bindings.as_std140().as_bytes(),
    )
}

#[derive(Default, Clone)]
pub struct MouseState {
    position: Vec2,
    down: bool,
    just_down: bool,
}

const MOUSE_BUTTONS: [MouseButton; 2] = [MouseButton::Left, MouseButton::Right];

pub fn mouse_state(
    mut mouse_state: ResMut<MouseState>,
    windows: Res<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        mouse_state.position = pos;
    }

    mouse_state.down = mouse_buttons.any_pressed(MOUSE_BUTTONS);
    mouse_state.just_down = mouse_buttons.any_just_pressed(MOUSE_BUTTONS);
}
