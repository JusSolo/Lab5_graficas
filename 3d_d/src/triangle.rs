use crate::fragment::Fragment;
use crate::vertex::Vertex;
use raylib::prelude::*;

pub fn triangle(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let x0 = v0.transformed_position.x;
    let y0 = v0.transformed_position.y;
    let x1 = v1.transformed_position.x;
    let y1 = v1.transformed_position.y;
    let x2 = v2.transformed_position.x;
    let y2 = v2.transformed_position.y;

    let min_x = x0.min(x1).min(x2).floor().max(0.0) as i32;
    let max_x = x0.max(x1).max(x2).ceil() as i32;
    let min_y = y0.min(y1).min(y2).floor().max(0.0) as i32;
    let max_y = y0.max(y1).max(y2).ceil() as i32;

    let denom = (y1 - y2) * (x0 - x2) + (x2 - x1) * (y0 - y2);
    if denom.abs() < f32::EPSILON {
        return fragments;
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let w0 = ((y1 - y2) * (x as f32 - x2) + (x2 - x1) * (y as f32 - y2)) / denom;
            let w1 = ((y2 - y0) * (x as f32 - x2) + (x0 - x2) * (y as f32 - y2)) / denom;
            let w2 = 1.0 - w0 - w1;

            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let depth = 0.0;
                fragments.push(Fragment {
                    position: Vector2::new(x as f32, y as f32),
                    depth,
                    color: Vector3::new(0.3, 0.7, 0.9),
                });
            }
        }
    }

    fragments
}
