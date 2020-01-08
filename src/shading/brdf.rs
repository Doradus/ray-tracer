use std::f32::consts;
use crate::Vector;

#[inline]
pub(crate) fn ggx_distribution(dot_nh: f32, a: f32) -> f32 {
    let a2 = a * a;
    let d = (dot_nh * dot_nh) * (a2 - 1.0) + 1.0;
    a2 / (consts::PI * d * d)
}

#[inline]
pub(crate) fn smith_for_ggx(dot_nl: f32, dot_nv: f32, a: f32) -> f32 {
    let a2 = a * a;
    let lambda_l = dot_nv * ((-dot_nl * a2 + dot_nl) * dot_nl + a2).sqrt();
    let lambda_v = dot_nl * ((-dot_nv * a2 + dot_nv) * dot_nv + a2).sqrt();
    0.5 / (lambda_l + lambda_v)  
}

#[inline]
pub(crate) fn schlick_fresnel_aprx(dot_lh: f32, spec_color: Vector) -> Vector {
    spec_color + (Vector::vec3(1.0, 1.0, 1.0) - spec_color) * (1.0 - dot_lh).powf(5.0)
}