use std::f32::consts;
use crate::Vector;
use crate::geometry::Rectangle;

#[inline]
pub(crate) fn sample_hemisphere_uniform(rand1: f32, rand2:f32) -> (Vector, f32) {
    let sin_theta = (1.0 - rand1 * rand1).sqrt();
    let phi = 2.0 * consts::PI * rand2;

    let x = sin_theta * phi.cos();
    let z = sin_theta * phi.sin();
    let pdf = 1.0 / (2.0 * consts::PI);
    (Vector::vec3(x, rand1, z), pdf)
}

#[inline]
pub(crate) fn sample_hemisphere_cosine_weighted(rand1: f32, rand2:f32) -> (Vector, f32) {
    let sin2_theta  = rand1;
    let cos2_theta = 1.0 - sin2_theta;
    let sin_theta = sin2_theta.sqrt();
    let cos_theta = cos2_theta.sqrt();

    let phi = 2.0 * consts::PI * rand2;

    let x = sin_theta * phi.cos();
    let z = sin_theta * phi.sin();

    let pdf = cos_theta / consts::PI;

    (Vector::vec3(x, cos_theta, z), pdf)
}

#[inline]
pub(crate) fn importance_sample_ggx(rand1: f32, rand2:f32, roughness: f32) -> (Vector, f32) {
	let a2 = roughness * roughness;

    let phi = 2.0 * consts::PI * rand1;
    let cos_theta = ((1.0 - rand2) / ((a2 - 1.0) * rand2 + 1.0)).sqrt();
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

	// let cos_theta = ((1.0 - rand2) / ( 1.0 + (a2 - 1.0) * rand2 )).sqrt();
	// let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

	let x = sin_theta * phi.cos();
	let z = sin_theta * phi.sin();
	
	// let d = (cos_theta * a2 - cos_theta) * cos_theta + 1.0;
	// let d = a2 / (consts::PI * d * d);
    // let pdf = d * cos_theta;
    
    let d = (a2 - 1.0) * (cos_theta * cos_theta) + 1.0;
    let pdf = (2.0 * a2 * cos_theta * sin_theta) / (d * d);

    (Vector::vec3(x, cos_theta, z), pdf)
}

#[inline]
pub(crate) fn sample_rectangle_uniform(rand1: f32, rand2: f32, rec: &Rectangle) -> (Vector, f32) {
    let x = rand1 * rec.width - rec.width * 0.5;
    let y = rand2 * rec.height - rec.width * 0.5;

    (Vector::vec3(x, y, 0.0), rec.area())
}

#[inline]
pub(crate) fn sample_solid_angle_of_sphere(rand1: f32, rand2: f32, q: f32) -> Vector {
    let phi = 2.0 * consts::PI * rand1;
    let theta = (1.0 - rand2 + rand2 * q).acos();

    let cos_theta = theta.cos();
    let sin_theta = theta.sin();

    let x = sin_theta * phi.cos();
    let z = sin_theta * phi.sin();

    Vector::vec3(x, cos_theta, z)
}