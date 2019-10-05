use crate::vec3::Vec3;
use crate::geometry::RayIntersect;
use std::f32;


pub fn cast_ray(origin: Vec3, direction: Vec3, scene_objects: &Vec<Box<dyn RayIntersect>>) -> Vec3 {
    match trace(origin, direction, scene_objects) {
        None => Vec3::new(0.0, 0.0, 0.0),
        Some(i) => scene_objects[i].get_color()
    }
}

fn trace(origin: Vec3, direction: Vec3, scene_objects: &Vec<Box<dyn RayIntersect>>) -> Option<usize> {
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