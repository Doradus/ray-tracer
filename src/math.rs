use std::f32::consts;

pub fn clamp<T>(value: T, min: T, max: T) -> T 
    where T : PartialOrd {
        if value < min {
            return min;
        } else if value > max {
            return max;
        } else {
            value
        }
}

pub fn degree_to_radians(deg: f32) -> f32 {
    deg * consts::PI / 180.0 
}