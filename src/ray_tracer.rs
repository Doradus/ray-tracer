use crate::vector::Vector;
use crate::geometry::Vertex;
use crate::geometry::IntersectResult;
use crate::geometry::intersect_triangle;
use std::f32;


pub fn cast_ray(origin: Vector, direction: Vector, scene_objects: &[Vertex]) -> Vector {
    match trace(origin, direction, scene_objects) {
        None => Vector::vec3(0.0, 0.0, 0.0),
        Some(i) => {
            let r = Vector::vec3(255.0, 0.0, 0.0);
            let g = Vector::vec3(0.0, 255.0, 0.0);
            let b = Vector::vec3(0.0, 0.0, 255.0);

            let color = r * i.u + g * i.v + b * (1.0 - i.u - i.v);

            color
        }
    }
}

fn trace(origin: Vector, direction: Vector, scene_objects: &[Vertex]) -> Option<IntersectResult> {
    let mut found:Option<IntersectResult> = None;
    let mut closest = f32::INFINITY;

        match intersect_triangle(origin, direction, scene_objects[0].pos, scene_objects[1].pos, scene_objects[2].pos) {
            Some(result) => {
                if result.t < closest {
                    closest = result.t;
                    found = Some(result);
                }
            },
            None => ()
        }


    // for i in 0..scene_objects.len() {
    //     match scene_objects[i].intersect(origin, direction) {
    //         Some(dist) => {
    //             if dist < closest {
    //                 found = Some(i);
    //                 closest = dist;
    //             }
    //         },
    //         None => ()
    //     }
    // }

    found
}