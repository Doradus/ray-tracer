use std::f32::consts;
use crate::Vector;

#[inline]
pub(crate) fn ggx_distribution(dot_nh: f32, a: f32) -> f32 {
    let a2 = a * a;

    let d = (dot_nh * dot_nh) * (a2 - 1.0) + 1.0;
    (positive(dot_nh) * a2) / (consts::PI * d * d)
}

#[inline]
pub(crate) fn smith_for_ggx(dot_nl: f32, dot_nv: f32, a: f32) -> f32 {
    let a2 = a * a;
    let lambda_l = dot_nv * ((-dot_nl * a2 + dot_nl) * dot_nl + a2).sqrt();
    let lambda_v = dot_nl * ((-dot_nv * a2 + dot_nv) * dot_nv + a2).sqrt();
    0.5 / (lambda_l + lambda_v)  
}

#[inline]
fn smith_a_value(n: Vector, s: Vector, a: f32) -> f32 {
    let dot_ns = n.vec3_dot_f32(s);

    dot_ns / (a * (1.0 - dot_ns * dot_ns).sqrt())
}

#[inline]
fn smith_lambda(a: f32) -> f32 {
    let a2 = a * a;
    (-1.0 + (1.0 + 1.0 / a2).sqrt()) / 2.0
}

#[inline]
pub(crate) fn height_correlated_smith_shadow_and_masking_for_ggx(n: Vector, l: Vector, v: Vector, a: f32) -> f32 {
    let lambda_v = smith_lambda(smith_a_value(n, v, a));
    let lambda_l = smith_lambda(smith_a_value(n, l, a));

    (positive(n.vec3_dot_f32(l)) * positive(n.vec3_dot_f32(v))) / (1.0 + lambda_l + lambda_v)
}

#[inline]
pub(crate) fn schlick_fresnel_aprx(dot_lh: f32, spec_color: Vector) -> Vector {
    let a  = 1.0 - dot_lh;
    let exponent = a * a * a * a * a;
    (spec_color + (Vector::vec3(1.0, 1.0, 1.0) - spec_color) * exponent)
}

#[inline]
fn positive(value : f32) -> f32 {
    if value == 0.0 {
        return 0.0;
    } else {
        1.0
    }
}