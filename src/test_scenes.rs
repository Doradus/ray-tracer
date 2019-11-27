use crate::matrix::Matrix;
use crate::geometry::*;
use crate::shading::*;
use crate::scene::*;
use crate::vector::*;
use crate::math::*;
use std::f32::consts;

pub fn multi_spheres() -> SceneData {
    let sphere1 = create_scene_object(
        create_sphere(0.15, 40, 20),
        Material::new(Vector::vec3(0.05, 0.05, 0.05), Vector::vec3(0.04, 0.04, 0.04), 0.1, 1.0, 0.0),
        Vector::vec3(-1.6, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let sphere2 = create_scene_object(
        create_sphere(0.15, 40, 20),
        Material::new(Vector::vec3(0.05, 0.05, 0.05), Vector::vec3(0.04, 0.04, 0.04), 0.2, 1.0, 0.0),
        Vector::vec3(-1.2, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let sphere3 = create_scene_object(
        create_sphere(0.15, 40, 20),
        Material::new(Vector::vec3(0.05, 0.05, 0.05), Vector::vec3(0.04, 0.04, 0.04), 0.3, 1.0, 0.0),
        Vector::vec3(-0.8, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let sphere4 = create_scene_object(
        create_sphere(0.15, 40, 20),
        Material::new(Vector::vec3(0.05, 0.05, 0.05), Vector::vec3(0.04, 0.04, 0.04), 0.4, 1.0, 0.0),
        Vector::vec3(-0.4, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let sphere5 = create_scene_object(
        create_sphere(0.15, 40, 20),
        Material::new(Vector::vec3(0.05, 0.05, 0.05), Vector::vec3(0.04, 0.04, 0.04), 0.5, 1.0, 0.0),
        Vector::vec3(0.0, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let sphere6 = create_scene_object(
        create_sphere(0.15, 40, 20),
        Material::new(Vector::vec3(0.05, 0.05, 0.05), Vector::vec3(0.04, 0.04, 0.04), 0.6, 1.0, 0.0),
        Vector::vec3(0.4, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let sphere7 = create_scene_object(
        create_sphere(0.15, 40, 20),
        Material::new(Vector::vec3(0.05, 0.05, 0.05), Vector::vec3(0.04, 0.04, 0.04), 0.7, 1.0, 0.0),
        Vector::vec3(0.8, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let sphere8 = create_scene_object(
        create_sphere(0.15, 40, 20),
        Material::new(Vector::vec3(0.05, 0.05, 0.05), Vector::vec3(0.04, 0.04, 0.04), 0.8, 1.0, 0.0),
        Vector::vec3(1.2, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let sphere9 = create_scene_object(
        create_sphere(0.15, 40, 20),
        Material::new(Vector::vec3(0.05, 0.05, 0.05), Vector::vec3(0.04, 0.04, 0.04), 0.9, 1.0, 0.0),
        Vector::vec3(1.6, -0.35, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let cube = create_scene_object(
        create_box(1.0, 1.0 , 1.0),
        Material::new(Vector::vec3(0.05, 0.05, 0.05), Vector::vec3(0.04, 0.04, 0.04), 0.1, 1.0, 0.0),
        Vector::vec3(0.75, 0.0, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0 * consts::PI / 180.0, 45.0 * consts::PI / 180.0, 0.0),
        0
    );
   
    let plane = create_scene_object(
        create_plane(8.0, 8.0, 5, 5),
        Material::new(Vector::vec3(0.7, 0.7, 0.7), Vector::vec3(0.04, 0.04, 0.04), 0.1, 1.0, 0.0),
        Vector::vec3(0.0, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let scene_objects = vec![sphere1, sphere2, sphere3, sphere4, sphere5, sphere6, sphere7, sphere8, sphere9, plane];

    let directional_light = Lights::Directional(DirectionalLight::new(Vector::vec3(-0.0, -0.6, -1.0), 1.0, Vector::vec3(1.0, 1.0, 1.0)));
    let point_light_1 = Lights::Point(PointLight::new(Vector::vec3(-1.3, 1.0, -2.5), 1.0, Vector::vec3(1.0, 1.0, 1.0), 2.0, Vector::vec3(0.0, 0.0, 1.0)));
    let point_light_2 = Lights::Point(PointLight::new(Vector::vec3(0.0, 1.0, -2.5), 1.0, Vector::vec3(1.0, 1.0, 1.0), 2.0, Vector::vec3(0.0, 0.0, 1.0)));
    let point_light_3 = Lights::Point(PointLight::new(Vector::vec3(1.3, 1.0, -2.5), 1.0, Vector::vec3(1.0, 1.0, 1.0), 2.0, Vector::vec3(0.0, 0.0, 1.0)));
    let lights = vec![directional_light];

    let scene = SceneData {
        scene_objects,
        lights
    };

    scene
}

pub fn transmission_test() -> SceneData {
    let sphere = create_scene_object(
        create_sphere(0.4, 40, 20),
        Material::new(Vector::vec3(0.6, 0.6, 0.6), Vector::vec3(0.04, 0.04, 0.04), 0.3, 1.0, 0.0),
        Vector::vec3(0.0, -0.1, -2.5),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let cube = create_scene_object(
        create_box(1.0, 1.0 , 1.0),
        Material::new(Vector::vec3(0.3, 0.3, 0.7), Vector::vec3(0.04, 0.04, 0.04), 0.7, 1.0, 0.0),
        Vector::vec3(0.75, 0.0, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0 * consts::PI / 180.0, 45.0 * consts::PI / 180.0, 0.0),
        0
    );

    let plane = create_scene_object(
        create_plane(8.0, 8.0, 5, 5),
        Material::new(Vector::vec3(0.3, 0.3, 0.3), Vector::vec3(0.04, 0.04, 0.04), 0.1, 1.0, 0.0),
        Vector::vec3(0.0, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let scene_objects = vec![plane, cube, sphere];

    let directional_light = Lights::Directional(DirectionalLight::new(Vector::vec3(-0.4, -0.6, -0.8), 1.0, Vector::vec3(1.0, 1.0, 1.0)));
    let lights = vec![directional_light];

    let scene = SceneData {
        scene_objects,
        lights
    };

    scene
}

pub fn area_ligt() -> SceneData {
    let sphere = create_scene_object(
        create_sphere(0.4, 40, 20),
        Material::new(Vector::vec3(0.6, 0.6, 0.6), Vector::vec3(0.04, 0.04, 0.04), 0.3, 1.0, 0.0),
        Vector::vec3(0.0, -0.1, -2.5),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        1
    );

    let bottom_plane = create_scene_object(
        create_plane(8.0, 8.0, 5, 5),
        Material::new(Vector::vec3(0.3, 0.3, 0.3), Vector::vec3(0.04, 0.04, 0.04), 0.1, 1.0, 0.0),
        Vector::vec3(0.0, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let wall_plane = create_scene_object(
        create_plane(8.0, 8.0, 5, 5),
        Material::new(Vector::vec3(0.3, 0.3, 0.3), Vector::vec3(0.04, 0.04, 0.04), 0.1, 1.0, 0.0),
        Vector::vec3(0.0, 0.0, -8.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), 0.0, 0.0),
        0
    );

    let scene_objects = vec![bottom_plane, wall_plane, sphere];

    let point_light = Lights::Point(PointLight::new(Vector::vec3(0.3, 1.0, -1.5), 1.0, Vector::vec3(1.0, 1.0, 1.0), 2.0, Vector::vec3(0.0, 0.0, 1.0)));

    let lights = vec![point_light];

    let scene = SceneData {
        scene_objects,
        lights
    };

    scene
}

pub fn gi_test() -> SceneData {
    let white = Vector::vec3(0.5, 0.5, 0.5);
    let black = Vector::vec3(0.0, 0.0, 0.0);
    let red = Vector::vec3(0.6, 0.1, 0.1);
    let green = Vector::vec3(0.1, 0.6, 0.1);
    let cube_green = create_scene_object(
        create_box(0.8, 1.5 , 1.0),
        Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.7, 1.0, 0.0),
        Vector::vec3(-0.5, 0.2, -3.5),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, degree_to_radians(30.0), 0.0),
        0
    );

    let cube_red = create_scene_object(
        create_box(0.7, 0.7 , 0.7),
        Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.7, 1.0, 0.0),
        Vector::vec3(0.5, -0.21, -2.5),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, degree_to_radians(155.0), 0.0),
        0
    );

    let sphere = create_scene_object(
        create_sphere(0.4, 40, 20),
        Material::new(black, Vector::vec3(1.00, 0.782, 0.344), 0.3, 1.0, 0.0),
        Vector::vec3(0.8, -0.1, -2.5),
        Vector::vec3(0.5, 0.5, 0.5),
        Vector::vec3(0.0, 0.0, 0.0),
        1
    );

    let bottom_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.3, 1.0, 0.0),
        Vector::vec3(0.0, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0),
        0
    );

    let top_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.5, 1.0, 0.0),
        Vector::vec3(0.0, 4.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, degree_to_radians(180.0)),
        0
    );

    let left_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        Material::new(red, Vector::vec3(0.04, 0.04, 0.04), 0.5, 1.0, 0.0),
        Vector::vec3(-2.0, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), degree_to_radians(90.0), 0.0),
        0
    ); 

    let right_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        Material::new(green, Vector::vec3(0.04, 0.04, 0.04), 0.5, 1.0, 0.0),
        Vector::vec3(2.0, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), degree_to_radians(270.0), 0.0),
        0
    ); 

    let wall_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.5, 1.0, 0.0),
        Vector::vec3(0.0, 0.0, -8.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), 0.0, 0.0),
        0
    );

    let back_plane = create_scene_object(
        create_plane(8.0, 8.0, 2, 2),
        Material::new(white, Vector::vec3(0.04, 0.04, 0.04), 0.5, 1.0, 0.0),
        Vector::vec3(0.0, 0.0, 1.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(degree_to_radians(90.0), degree_to_radians(180.0), 0.0),
        0
    );

    let scene_objects = vec![cube_green, cube_red, top_plane, bottom_plane, back_plane, wall_plane, right_plane, left_plane];

    let point_light = Lights::Point(PointLight::new(Vector::vec3(0.3, 2.0, -1.6), 2.0, Vector::vec3(1.0, 1.0, 1.0), 2.0, Vector::vec3(0.0, 1.0, 0.0)));
    let lights = vec![point_light];
    // let directional_light = Lights::Directional(DirectionalLight::new(Vector::vec3(-0.0, -0.6, -1.0), 1.0, Vector::vec3(1.0, 1.0, 1.0)));
    // let lights = vec![directional_light];

    let scene = SceneData {
        scene_objects,
        lights
    };

    scene
}

fn create_scene_object(mesh: Mesh, material: Material, position:Vector, scale: Vector, rotation: Vector, bounding_volume_type: u32) -> SceneObject {
    let scale_matrix = Matrix::scaling_matrix(scale);
    let translation_matrix = Matrix::translation_matrix(position);
    let rotation_matrix = Matrix::roatation_x(rotation.x()) * Matrix::roatation_y(rotation.y());
    let world_matrix = scale_matrix * rotation_matrix * translation_matrix;
    let inv_world = world_matrix.inverse().transpose();

    let mut transformed_vertices = Vec::new();
    let mut bounding_box = BoundingBox::new(mesh.vertices[0].pos * world_matrix);

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

    SceneObject::new(mesh_data, material, bounding_box)
}