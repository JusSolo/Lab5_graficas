use crate::Uniforms;
use crate::vertex::Vertex;
use raylib::prelude::*;

// ==========================================
// === Transformación del vértice (igual) ===
// ==========================================
fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position_vec4 = Vector4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed_vec4 = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);

    let transformed_position = if transformed_vec4.w != 0.0 {
        Vector3::new(
            transformed_vec4.x / transformed_vec4.w,
            transformed_vec4.y / transformed_vec4.w,
            transformed_vec4.z / transformed_vec4.w,
        )
    } else {
        Vector3::new(transformed_vec4.x, transformed_vec4.y, transformed_vec4.z)
    };

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: vertex.normal,
    }
}

// ==========================================
// === Shaders personalizados por esfera ===
// ==========================================

fn simple_noise(x: f32, y: f32, z: f32) -> f32 {
    ((x * 12.9898 + y * 78.233 + z * 37.719).sin() * 43758.5453).fract()
}

pub fn star_shader(pos: &Vector3) -> Vector3 {
    // Calculamos un “ruido” basado en la posición
    let n = simple_noise(pos.x * 0.1, pos.y * 0.1, pos.z * 0.1);
    // Mezclamos variación de color
    let base = Vector3::new(1.0, 0.94, 0.6);
    let variation = Vector3::new(0.2 * n, 0.1 * n, 0.05 * n);
    let color = base + variation;
    // Emisión/un brillo extra — podrías multiplicar por un factor para que parezca que “emite”
    let brightness = 1.0 + n * 0.5;
    color * brightness
}

// 🪨 Planeta rocoso rojizo con relieve
pub fn rocky_shader(pos: &Vector3) -> Vector3 {
    let noise = ((pos.x * 0.3).sin() * (pos.y * 0.7).cos() * (pos.z * 0.3).sin()) * 0.5 + 0.5;
    let r = 0.64 + noise * 0.35;
    let g = 0.25 + noise * 0.2;
    let b = 0.2;
    Vector3::new(r, g, b)
}

// ☁️ Planeta gaseoso celeste con remolino
pub fn gas_shader(pos: &Vector3) -> Vector3 {
    let swirl = ((pos.x * 0.15).sin() + (pos.y * 0.2).cos()) * 0.5 + 0.5;
    let storm = ((pos.x * 0.3 + pos.y * 0.3).sin() * 0.5 + 0.5) * swirl;
    let r = 0.25 + 0.1 * storm;
    let g = 0.6 + 0.2 * storm;
    let b = 0.9 + 0.1 * swirl;
    Vector3::new(r, g, b)
}
