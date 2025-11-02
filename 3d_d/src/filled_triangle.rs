use crate::fragment::Fragment;
use crate::vertex::Vertex;
use raylib::math::Vector3;

pub fn filled_triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Convertir a coordenadas de pantalla
    let p1 = (
        v1.transformed_position.x as i32,
        v1.transformed_position.y as i32,
    );
    let p2 = (
        v2.transformed_position.x as i32,
        v2.transformed_position.y as i32,
    );
    let p3 = (
        v3.transformed_position.x as i32,
        v3.transformed_position.y as i32,
    );

    // Encontrar el bounding box del triángulo
    let min_x = p1.0.min(p2.0).min(p3.0);
    let max_x = p1.0.max(p2.0).max(p3.0);
    let min_y = p1.1.min(p2.1).min(p3.1);
    let max_y = p1.1.max(p2.1).max(p3.1);

    // Función para calcular el área del triángulo (para coordenadas baricéntricas)
    fn edge_function(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> f32 {
        ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)) as f32
    }

    let area = edge_function(p1, p2, p3);

    // Probar cada píxel en el bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // Calcular coordenadas baricéntricas
            let w1 = edge_function(p2, p3, (x, y)) / area;
            let w2 = edge_function(p3, p1, (x, y)) / area;
            let w3 = edge_function(p1, p2, (x, y)) / area;

            // Si el punto está dentro del triángulo
            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                // Interpolar el color
                let color = Vector3::new(
                    w1 * v1.color.x + w2 * v2.color.x + w3 * v3.color.x,
                    w1 * v1.color.y + w2 * v2.color.y + w3 * v3.color.y,
                    w1 * v1.color.z + w2 * v2.color.z + w3 * v3.color.z,
                );

                // Interpolar la profundidad Z
                let depth = w1 * v1.transformed_position.z
                    + w2 * v2.transformed_position.z
                    + w3 * v3.transformed_position.z;

                fragments.push(Fragment::new(x as f32, y as f32, color, depth));
            }
        }
    }

    fragments
}
