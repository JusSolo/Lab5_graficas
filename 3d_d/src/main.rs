mod fragment;
mod framebuffer;
mod line;
mod matrix;
mod obj;
mod shaders;
mod triangle;
mod vertex;

use crate::matrix::new_matrix4;
use framebuffer::Framebuffer;
use obj::Obj;
use raylib::prelude::*;
use shaders::{gas_shader, rocky_shader, star_shader, vertex_shader};
use std::f32::consts::PI;
use std::thread;
use std::time::Duration;
use triangle::triangle;
use vertex::Vertex;

pub struct Uniforms {
    pub model_matrix: Matrix,
}

fn create_model_matrix(translation: Vector3, scale: f32, rotation: Vector3) -> Matrix {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = new_matrix4(
        1.0, 0.0, 0.0, 0.0, 0.0, cos_x, -sin_x, 0.0, 0.0, sin_x, cos_x, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_y = new_matrix4(
        cos_y, 0.0, sin_y, 0.0, 0.0, 1.0, 0.0, 0.0, -sin_y, 0.0, cos_y, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_z = new_matrix4(
        cos_z, -sin_z, 0.0, 0.0, sin_z, cos_z, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;
    let scale_matrix = new_matrix4(
        scale, 0.0, 0.0, 0.0, 0.0, scale, 0.0, 0.0, 0.0, 0.0, scale, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let translation_matrix = new_matrix4(
        1.0,
        0.0,
        0.0,
        translation.x,
        0.0,
        1.0,
        0.0,
        translation.y,
        0.0,
        0.0,
        1.0,
        translation.z,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    scale_matrix * rotation_matrix * translation_matrix
}

fn render_with_shader(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    shader_fn: fn(&Vector3) -> Vector3,
) {
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|v| vertex_shader(v, uniforms))
        .collect();

    for tri in transformed_vertices.chunks(3) {
        if tri.len() < 3 {
            continue;
        }

        let fragments = triangle(&tri[0], &tri[1], &tri[2]);
        for frag in fragments {
            let color = shader_fn(&Vector3::new(frag.position.x, frag.position.y, frag.depth));
            framebuffer.point(frag.position.x as i32, frag.position.y as i32, color);
        }
    }
}

fn main() {
    let window_width = 900;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("🌌 Sistema Solar Procedural - Rust Renderer")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Vector3::new(0.02, 0.02, 0.05));
    framebuffer.init_texture(&mut window, &thread);

    // Rotación de cámara y zoom
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);
    let mut zoom = 1.0_f32;

    // Ángulo de órbita (para animación de los planetas)
    let mut orbit_angle: f32 = 0.0;

    // Cargar modelo de esfera
    let obj = Obj::load("assets/models/sphere.obj").expect("❌ No se pudo cargar sphere.obj");
    let vertex_array = obj.get_vertex_array();

    // Propiedades iniciales del sistema
    let sun_position = Vector3::new(450.0, 300.0, 0.0);

    let mut rocky_orbit_radius = 200.0;
    let mut gas_orbit_radius = 320.0;

    while !window.window_should_close() {
        // --- Controles de cámara ---
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            rotation.y -= PI / 180.0 * 2.0;
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            rotation.y += PI / 180.0 * 2.0;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            rotation.x -= PI / 180.0 * 2.0;
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            rotation.x += PI / 180.0 * 2.0;
        }

        if window.is_key_down(KeyboardKey::KEY_A) {
            zoom *= 1.02;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            zoom *= 0.98;
        }

        // --- Actualizar órbita ---
        orbit_angle += PI / 180.0 * 0.5; // velocidad orbital
        let rocky_pos = Vector3::new(
            sun_position.x + rocky_orbit_radius * orbit_angle.cos(),
            sun_position.y + rocky_orbit_radius * orbit_angle.sin(),
            0.0,
        );
        let gas_pos = Vector3::new(
            sun_position.x + gas_orbit_radius * (orbit_angle * 0.7).cos(),
            sun_position.y + gas_orbit_radius * (orbit_angle * 0.7).sin(),
            0.0,
        );

        framebuffer.clear();

        // --- Render del Sol ---
        let sun_matrix = create_model_matrix(sun_position, 185.0 * zoom, rotation);
        let uniforms = Uniforms {
            model_matrix: sun_matrix,
        };
        render_with_shader(&mut framebuffer, &uniforms, &vertex_array, star_shader);

        // --- Planeta rocoso ---
        let rocky_matrix = create_model_matrix(rocky_pos, 25.0 * zoom, rotation);
        let uniforms = Uniforms {
            model_matrix: rocky_matrix,
        };
        render_with_shader(&mut framebuffer, &uniforms, &vertex_array, rocky_shader);

        // --- Planeta gaseoso ---
        let gas_matrix = create_model_matrix(gas_pos, 60.0 * zoom, rotation);
        let uniforms = Uniforms {
            model_matrix: gas_matrix,
        };
        render_with_shader(&mut framebuffer, &uniforms, &vertex_array, gas_shader);

        framebuffer.swap_buffers(&mut window, &thread);
        thread::sleep(Duration::from_millis(16));
    }
}
