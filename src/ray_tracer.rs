use crate::vector::Vector;
use crate::geometry::RayIntersect;
use std::f32;


pub fn cast_ray(origin: Vector, direction: Vector, scene_objects: &Vec<Box<dyn RayIntersect>>) -> Vector {
    match trace(origin, direction, scene_objects) {
        None => Vector::vec3(0.0, 0.0, 0.0),
        Some(i) => scene_objects[i].get_color()
    }
}

fn trace(origin: Vector, direction: Vector, scene_objects: &Vec<Box<dyn RayIntersect>>) -> Option<usize> {
    let mut found:Option<usize> = None;
    let mut closest = f32::INFINITY;

    for i in 0..scene_objects.len() {
        match scene_objects[i].intersect(origin, direction) {
            Some(dist) => {
                if dist < closest {
                    found = Some(i);
                    closest = dist;
                }
            },
            None => ()
        }
    }

    found
}