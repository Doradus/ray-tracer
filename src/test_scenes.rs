#![allow(dead_code)]
#![allow(unused_variables)]

use crate::shading::*;
use crate::matrix::Matrix;
use crate::geometry::*;
use crate::scene::*;
use crate::vector_simd::*;
use crate::math::*;
use crate::bvh::*;
use crate::camera::Camera;
use std::f32::consts;
use std::time::Instant;

pub fn multi_spheres() -> SceneData {
    let diffuse = Vector::vec3(0.01, 0.01, 0.01);
    let specular = Vector::vec3(1.0, 0.782, 0.344);

    let sphere1 = create_scene_object(
        create_sphere(0.15, 40, 20),
        materials::Material::new(diffuse, specular, 0.1, 1.0, 0.0, 0.0),
        Vector::vec3(-1.6, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let sphere2 = create_scene_object(
        create_sphere(0.15, 40, 20),
        materials::Material::new(diffuse, specular, 0.2, 1.0, 0.0, 0.0),
        Vector::vec3(-1.2, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let sphere3 = create_scene_object(
        create_sphere(0.15, 40, 20),
        materials::Material::new(diffuse, specular, 0.3, 1.0, 0.0, 0.0),
        Vector::vec3(-0.8, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let sphere4 = create_scene_object(
        create_sphere(0.15, 40, 20),
        materials::Material::new(diffuse, specular, 0.4, 1.0, 0.0, 0.0),
        Vector::vec3(-0.4, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let sphere5 = create_scene_object(
        create_sphere(0.15, 40, 20),
        materials::Material::new(diffuse, specular, 0.5, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let sphere6 = create_scene_object(
        create_sphere(0.15, 40, 20),
        materials::Material::new(diffuse, specular, 0.6, 1.0, 0.0, 0.0),
        Vector::vec3(0.4, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let sphere7 = create_scene_object(
        create_sphere(0.15, 40, 20),
        materials::Material::new(diffuse, specular, 0.7, 1.0, 0.0, 0.0),
        Vector::vec3(0.8, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let sphere8 = create_scene_object(
        create_sphere(0.15, 40, 20),
        materials::Material::new(diffuse, specular, 0.8, 1.0, 0.0, 0.0),
        Vector::vec3(1.2, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let sphere9 = create_scene_object(
        create_sphere(0.15, 40, 20),
        materials::Material::new(diffuse, specular, 0.9, 1.0, 0.0, 0.0),
        Vector::vec3(1.6, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );
   
    let plane = create_scene_object(
        create_plane(8.0, 8.0, 5, 5),
        materials::Material::new(Vector::vec3(0.4, 0.4, 0.4), Vector::vec3(0.04, 0.04, 0.04), 0.1, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let scene_objects = vec![sphere1, sphere2, sphere3, sphere4, sphere5, sphere6, sphere7, sphere8, sphere9, plane];

    let directional_light = lights::Lights::Directional(lights::DirectionalLight::new(Vector::vec3(-0.0, -0.6, -1.0), 1.0, Vector::vec3(1.0, 1.0, 1.0)));
    let lights = vec![directional_light];

    let bvh_res = build_bvh(&scene_objects);
    let bvh = bvh_res.0;
    let indices = bvh_res.1;
    let camera = Camera::new(Vector::vec3(0.0, 0.0, 0.0), Vector::vec3(0.0, 0.0, -1.0));


    let scene = SceneData {
        bvh: bvh,
        object_indices: indices,
        scene_objects: scene_objects,
        lights: lights,
        camera: camera
    };

    scene
}

pub fn transmission_test() -> SceneData {
    let sphere = create_scene_object(
        create_sphere(0.4, 40, 20),
        materials::Material::new(Vector::vec3(0.6, 0.6, 0.6), Vector::vec3(0.04, 0.04, 0.04), 0.3, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, -0.1, -2.5),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let cube = create_scene_object(
        create_box(1.0, 1.0 , 1.0),
        materials::Material::new(Vector::vec3(0.3, 0.3, 0.7), Vector::vec3(0.04, 0.04, 0.04), 0.7, 1.0, 0.0, 0.0),
        Vector::vec3(0.75, 0.0, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0 * consts::PI / 180.0, 45.0 * consts::PI / 180.0, 0.0)
    );

    let plane = create_scene_object(
        create_plane(8.0, 8.0, 5, 5),
        materials::Material::new(Vector::vec3(0.3, 0.3, 0.3), Vector::vec3(0.04, 0.04, 0.04), 0.1, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let scene_objects = vec![plane, cube, sphere];

    let directional_light = lights::Lights::Directional(lights::DirectionalLight::new(Vector::vec3(-0.4, -0.6, -0.8), 1.0, Vector::vec3(1.0, 1.0, 1.0)));
    let lights = vec![directional_light];

    let bvh_res = build_bvh(&scene_objects);
    let bvh = bvh_res.0;
    let indices = bvh_res.1;
    let camera = Camera::new(Vector::vec3(0.0, 0.0, 0.0), Vector::vec3(0.0, 0.0, -1.0));


    let scene = SceneData {
        bvh: bvh,
        object_indices: indices,
        scene_objects: scene_objects,
        lights: lights,
        camera: camera
    };

    scene
}

pub fn area_ligt() -> SceneData {
    let color = Vector::vec3(0.7, 0.7, 0.7);
    let sphere_color = Vector::vec3(0.8, 0.8, 0.8);

    let sphere1 = create_scene_object(
        create_sphere(0.4, 20, 20),
        materials::Material::new(sphere_color, Vector::vec3(0.04, 0.04, 0.04), 0.1, 1.0, 0.0, 0.0),
        Vector::vec3(-0.42, -0.3, -1.4),
        Vector::vec3(0.5, 0.5, 0.5),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let sphere2 = create_scene_object(
        create_sphere(0.4, 40, 20),
        materials::Material::new(sphere_color, Vector::vec3(0.04, 0.04, 0.04), 0.1, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, -0.3, -1.4),
        Vector::vec3(0.5, 0.5, 0.5),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let sphere3 = create_scene_object(
        create_sphere(0.4, 20, 20),
        materials::Material::new(sphere_color, Vector::vec3(0.04, 0.04, 0.04), 0.1, 1.0, 0.0, 0.0),
        Vector::vec3(0.42, -0.3, -1.4),
        Vector::vec3(0.5, 0.5, 0.5),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let bottom_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(color, Vector::vec3(0.04, 0.04, 0.04), 0.6, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let top_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(color, Vector::vec3(0.04, 0.04, 0.04), 0.6, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, 1.1, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(180.0), 0.0, 0.0)
    );

    let left_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(color, Vector::vec3(0.04, 0.04, 0.04), 0.6, 1.0, 0.0, 0.0),
        Vector::vec3(-1.2, -1.0, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), degree_to_radians(90.0), 0.0)
    ); 

    let right_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(color, Vector::vec3(0.04, 0.04, 0.04), 0.6, 1.0, 0.0, 0.0),
        Vector::vec3(1.2, -1.0, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), degree_to_radians(270.0), 0.0)
    ); 

    let wall_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(color, Vector::vec3(0.04, 0.04, 0.04), 0.6, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, -0.5, -5.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), 0.0, 0.0)
    );

    let back_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(color, Vector::vec3(0.04, 0.04, 0.04), 0.6, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, 0.0, 1.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), degree_to_radians(180.0), 0.0)
    );

    let scene_objects = vec![top_plane, bottom_plane, back_plane, wall_plane, right_plane, left_plane, sphere2];

    // let point_light = Lights::Point(PointLight::new(Vector::vec3(0.3, 1.0, -1.5), 1.0, Vector::vec3(1.0, 1.0, 1.0), 2.0, Vector::vec3(0.0, 0.0, 1.0)));
    // let lights = vec![point_light];

    let directional_light = lights::Lights::Directional(lights::DirectionalLight::new(Vector::vec3(-0.0, -0.6, -0.8), 1.5, Vector::vec3(1.0, 1.0, 1.0)));
    let rec_light = lights::Lights::Rectangular(lights::RectangularLight::new(Vector::vec3(0.0, 0.0, 0.0), Vector::vec3(0.42, -0.3, -1.4), 1.45, 0.5, 9, 20.0, Vector::vec3(1.0, 1.0, 1.0), 10.0, Vector::vec3(1.0, 1.0, 1.0)));
    let lights = vec![rec_light];

    let bvh_res = build_bvh(&scene_objects);
    let bvh = bvh_res.0;
    let indices = bvh_res.1;
    let camera = Camera::new(Vector::vec3(-0.6, 0.25, -1.4), Vector::vec3(0.0, -0.3, -1.4));

    let scene = SceneData {
        bvh: bvh,
        object_indices: indices,
        scene_objects: scene_objects,
        lights: lights,
        camera: camera
    };

    scene
}

pub fn furnance_test() -> SceneData {
    let sphere = create_scene_object(
        create_sphere(1.0, 40, 20),
        materials::Material::new(Vector::vec3(0.18, 0.18, 0.18), Vector::vec3(0.04, 0.04, 0.04), 0.3, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, 0.0, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let scene_objects = vec![sphere];

    let point_light = lights::Lights::Spherical(lights::SphericalLight::new(Vector::vec3(0.3, 1.5, -1.6), 0.0, Vector::vec3(1.0, 0.945, 0.878), 2.0, Vector::vec3(0.0, 0.0, 1.0), 0.0, 0));
    let lights = vec![point_light];

    let bvh_res = build_bvh(&scene_objects);
    let bvh = bvh_res.0;
    let indices = bvh_res.1;
    let camera = Camera::new(Vector::vec3(0.0, 0.0, 0.0), Vector::vec3(0.0, 0.0, -1.0));

    let scene = SceneData {
        bvh: bvh,
        object_indices: indices,
        scene_objects: scene_objects,
        lights: lights,
        camera: camera
    };

    scene    
}

pub fn spehres() -> SceneData {
    let spec = Vector::vec3(0.04, 0.04, 0.04);
    let white = Vector::vec3(0.8, 0.8, 0.8);
    let black = Vector::vec3(0.0, 0.0, 0.0);
    let red = Vector::vec3(0.82, 0.6, 0.6);
    let green = Vector::vec3(0.7, 0.82, 0.69);
    let brown = Vector::vec3(0.46, 0.40, 0.25);
    let orange = Vector::vec3(0.83, 0.53, 0.33);
    let blue = Vector::vec3(0.30, 0.55, 0.68);
    let purple = Vector::vec3(0.51, 0.13, 0.68);


    let colors = vec![red, green, orange, white, blue, purple];
    let roughness = vec![1.0, 0.65, 0.4, 0.6, 0.7, 0.5, 0.7, 0.35, 0.8, 0.37];

    let chrome_spec = Vector::vec3(0.549, 0.556, 0.554);
    let silver_spec = Vector::vec3(0.972, 0.960, 0.915);
    let gold_spec = Vector::vec3(1.0, 0.782, 0.344);


    let bottom_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(brown, Vector::vec3(0.04, 0.04, 0.04), 0.6, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let large_sphere1 = create_scene_object(
        create_sphere(0.4, 20, 20),
        materials::Material::new(black, gold_spec, 0.35, 1.0, 0.0, 1.0),
        Vector::vec3(1.0, -0.3, -2.0),
        Vector::vec3(0.5, 0.5, 0.5),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let large_sphere2 = create_scene_object(
        create_sphere(0.4, 20, 20),
        materials::Material::new(black, silver_spec, 0.3, 1.0, 0.0, 1.0),
        Vector::vec3(-1.2, -0.3, -2.4),
        Vector::vec3(0.5, 0.5, 0.5),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let large_sphere3 = create_scene_object(
        create_sphere(0.4, 20, 20),
        materials::Material::new(Vector::vec3(0.4, 0.1, 0.1), Vector::vec3(0.04, 0.04, 0.04), 0.25, 1.0, 0.0, 0.0),
        Vector::vec3(-1.0, -0.3, -3.0),
        Vector::vec3(0.5, 0.5, 0.5),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let mut scene_objects = vec![large_sphere1, large_sphere2, large_sphere3, bottom_plane];


    for i in 0..20 {
        let x = (i / 5) as f32 * 0.4;
        let z = (i % 4) as f32 * 0.5;

        // let color_index = rand::thread_rng().gen_range(0.0, 5.0) as usize;


        let sphere = create_scene_object(
            create_sphere(0.2, 20, 20),
            materials::Material::new(colors[i % 5], spec, roughness[i % 9], 1.0, 0.0, 0.0),
            Vector::vec3(x - 0.6, -0.4, z -3.0),
            Vector::vec3(0.5, 0.5, 0.5),
            Vector::vec3(0.0, 0.0, 0.0)
        );
    
        scene_objects.push(sphere);
    }

    // scene_objects.push(bottom_plane);

    let bvh_res = build_bvh(&scene_objects);
    let bvh = bvh_res.0;
    let indices = bvh_res.1;

    let directional_light = lights::Lights::Directional(lights::DirectionalLight::new(Vector::vec3(-0.0, -0.6, -1.0), 1.5, Vector::vec3(1.0, 1.0, 1.0)));
    let rec_light = lights::Lights::Rectangular(lights::RectangularLight::new(Vector::vec3(0.0, 1.599, -3.0), Vector::vec3(0.0, -1.0, 0.0), 0.75, 0.75, 10, 5.0, Vector::vec3(1.0, 0.945, 0.878), 10.0, Vector::vec3(1.0, 1.0, 1.0)));
    let point_light = lights::Lights::Spherical(lights::SphericalLight::new(Vector::vec3(0.0, 0.5, -2.0), 30.0, Vector::vec3(1.0, 1.0, 1.0), 10.0, Vector::vec3(0.0, 0.0, 1.0), 0.15, 100));
    
    let lights = vec![directional_light];
    let camera = Camera::new(Vector::vec3(0.0, 0.0, 0.0), Vector::vec3(0.0, 0.0, -1.0));

    let scene = SceneData {
        bvh: bvh,
        object_indices: indices,
        scene_objects: scene_objects,
        lights: lights,
        camera: camera
    };

    scene
}

pub fn area_ligt_specular_test() -> SceneData {
    // let plane1 = create_scene_object(
    //     create_plane(2.0, 0.5, 2, 2),
    //     materials::Material::new(Vector::vec3(0.1, 0.1, 0.1), Vector::vec3(0.04, 0.04, 0.04), 0.3, 1.0, 0.0, 0.0),
    //     Vector::vec3(0.0, 0.0, -2.0),
    //     Vector::vec3(1.0, 1.0, 1.0),
    //     Vector::vec3(degree_to_radians(10.0), 0.0, 0.0)
    // );

    // let plane2 = create_scene_object(
    //     create_plane(2.0, 0.5, 2, 2),
    //     materials::Material::new(Vector::vec3(0.1, 0.1, 0.1), Vector::vec3(0.04, 0.04, 0.04), 0.2, 1.0, 0.0, 0.0),
    //     Vector::vec3(0.0, 0.14, -2.5),
    //     Vector::vec3(1.0, 1.0, 1.0),
    //     Vector::vec3(degree_to_radians(20.0), 0.0, 0.0)
    // );

    // let plane3 = create_scene_object(
    //     create_plane(2.0, 0.5, 2, 2),
    //     materials::Material::new(Vector::vec3(0.1, 0.1, 0.1), Vector::vec3(0.04, 0.04, 0.04), 0.4, 1.0, 0.0, 0.0),
    //     Vector::vec3(0.0, 0.36, -3.0),
    //     Vector::vec3(1.0, 1.0, 1.0),
    //     Vector::vec3(degree_to_radians(30.0), 0.0, 0.0)
    // );

    // let plane4 = create_scene_object(
    //     create_plane(2.0, 0.5, 2, 2),
    //     materials::Material::new(Vector::vec3(0.1, 0.1, 0.1), Vector::vec3(0.04, 0.04, 0.04), 0.01, 1.0, 0.0, 0.0),
    //     Vector::vec3(0.0, 0.66, -3.4),
    //     Vector::vec3(1.0, 1.0, 1.0),
    //     Vector::vec3(degree_to_radians(40.0), 0.0, 0.0)
    // );

    let wall_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(Vector::vec3(0.1, 0.1, 0.1), Vector::vec3(0.04, 0.04, 0.04), 0.3, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, -0.5, -3.5),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), 0.0, 0.0)
    );

    let scene_objects = vec![wall_plane];

    let blue = Vector::vec3(0.30, 0.55, 0.68);
    let orange = Vector::vec3(0.83, 0.53, 0.33);
    let green = Vector::vec3(0.6, 0.87, 0.59);
    let red = Vector::vec3(0.82, 0.5, 0.5);

    let point_light1 = lights::Lights::Spherical(lights::SphericalLight::new(Vector::vec3(-0.75, 0.95, -2.8), 50.0, orange, 100.0, Vector::vec3(0.0, 0.0, 1.0), 0.02, 16));
    let point_light2 = lights::Lights::Spherical(lights::SphericalLight::new(Vector::vec3(-0.3, 0.95, -2.8), 50.0, blue, 100.0, Vector::vec3(0.0, 0.0, 1.0), 0.04, 16));
    let point_light3 = lights::Lights::Spherical(lights::SphericalLight::new(Vector::vec3(0.3, 0.95, -2.8), 50.0, green, 100.0, Vector::vec3(0.0, 0.0, 1.0), 0.08, 16));
    let point_light4 = lights::Lights::Spherical(lights::SphericalLight::new(Vector::vec3(0.75, 0.95, -2.8), 50.0, red, 100.0, Vector::vec3(0.0, 0.0, 1.0), 0.16, 16));

    let lights = vec![point_light1, point_light2, point_light3, point_light4];

    let bvh_res = build_bvh(&scene_objects);
    let bvh = bvh_res.0;
    let indices = bvh_res.1;
    let camera = Camera::new(Vector::vec3(0.0, 0.5, 0.0), Vector::vec3(0.0, 0.5, -1.0));

    let scene = SceneData {
        bvh: bvh,
        object_indices: indices,
        scene_objects: scene_objects,
        lights: lights,
        camera: camera
    };

    scene    
}

pub fn gi_test() -> SceneData {
    let white = Vector::vec3(0.8, 0.8, 0.8);
    let black = Vector::vec3(0.0, 0.0, 0.0);
    let red = Vector::vec3(0.4, 0.15, 0.15);
    let green = Vector::vec3(0.15, 0.4, 0.1);
    
    let silver_spec = Vector::vec3(0.972, 0.960, 0.915);
    let aluminum_spec = Vector::vec3(0.913, 0.922, 0.924);
    let chrome_spec = Vector::vec3(0.549, 0.556, 0.554);

    let cube_green = create_scene_object(
        create_box(0.6, 1.0 , 0.6),
        materials::Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.3, 1.0, 0.0, 0.0),
        Vector::vec3(-0.4, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, degree_to_radians(30.0), 0.0)
    );

    let cube_red = create_scene_object(
        create_box(0.5, 0.7 , 0.5),
        materials::Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.3, 1.0, 0.0, 0.0),
        Vector::vec3(0.4, -0.71, -3.5),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, degree_to_radians(155.0), 0.0)
    );

    let sphere = create_scene_object(
        create_sphere(0.4, 20, 20),
        materials::Material::new(black, aluminum_spec, 0.2, 1.0, 0.0, 1.0),
        Vector::vec3(-1.2, -0.3, -4.4),
        Vector::vec3(0.5, 0.5, 0.5),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let bottom_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.3, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, -1.0, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let top_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.6, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, 1.1, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(180.0), 0.0, 0.0)
    );

    let left_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(red, Vector::vec3(0.04, 0.04, 0.04), 0.2, 1.0, 0.0, 0.0),
        Vector::vec3(-1.2, -1.0, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), degree_to_radians(90.0), 0.0)
    ); 

    let right_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(green, Vector::vec3(0.04, 0.04, 0.04), 0.2, 1.0, 0.0, 0.0),
        Vector::vec3(1.2, -1.0, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), degree_to_radians(270.0), 0.0)
    ); 

    let wall_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.6, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, -0.5, -6.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), 0.0, 0.0)
    );

    let back_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        materials::Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.6, 1.0, 0.0, 0.0),
        Vector::vec3(0.0, 0.0, 1.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), degree_to_radians(180.0), 0.0)
    );

    let scene_objects = vec![cube_green, cube_red, top_plane, bottom_plane, back_plane, wall_plane, right_plane, left_plane];

    let bvh_res = build_bvh(&scene_objects);
    let bvh = bvh_res.0;
    let indices = bvh_res.1;

    let point_light = lights::Lights::Spherical(lights::SphericalLight::new(Vector::vec3(0.0, 1.099, -3.0), 150.0, Vector::vec3(1.0, 0.945, 0.878), 10.0, Vector::vec3(0.0, 0.0, 1.0), 0.0, 0));
    let rec_light = lights::Lights::Rectangular(lights::RectangularLight::new(Vector::vec3(0.0, 1.099, -3.0), Vector::vec3(0.0, 0.0, -3.0), 0.75, 0.75, 10, 5.0, Vector::vec3(1.0, 0.945, 0.878), 10.0, Vector::vec3(1.0, 1.0, 1.0)));
    let lights = vec![rec_light];
    let camera = Camera::new(Vector::vec3(0.0, 0.0, 0.0), Vector::vec3(0.0, 0.0, -1.0));

    let scene = SceneData {
        bvh: bvh,
        object_indices: indices,
        scene_objects: scene_objects,
        lights: lights,
        camera: camera
    };

    scene
}

fn create_scene_object(mesh: Mesh, material: materials::Material, position:Vector, scale: Vector, rotation: Vector) -> SceneObject {
    let now = Instant::now();

    let scale_matrix = Matrix::scaling_matrix(scale);
    let translation_matrix = Matrix::translation_matrix(position);
    let rotation_matrix = Matrix::roatation_x(rotation.x()) * Matrix::roatation_y(rotation.y());
    let world_matrix = scale_matrix * rotation_matrix * translation_matrix;
    let inv_world = world_matrix.inverse().transpose();

    let mut transformed_vertices = Vec::new();
    let mut bounding_box = BoundingBox::new();

    for i in 0..mesh.vertices.len() {
        let vertex = Vertex::new(mesh.vertices[i].pos * world_matrix, mesh.vertices[i].normal * inv_world);
        bounding_box.extend_bounds(vertex.pos);   
        transformed_vertices.push(vertex);
    }
    
    let mesh_data = Mesh {
        vertices: transformed_vertices,
        indices: mesh.indices,
        num_tris: mesh.num_tris
    };

    let end = now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 1e-9;
    // println!("generated scene in {} s", end);

    SceneObject::new(mesh_data, material, bounding_box)
}