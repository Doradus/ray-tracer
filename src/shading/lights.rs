
use crate::geometry::Rectangle;
use crate::Vector;
use crate::matrix::Matrix;

pub struct LightColorInfo {
    pub brightness: f32,
    pub color: Vector,
    pub exposure: i32
}

impl LightColorInfo {
    pub fn intensity(&self) -> Vector {
        let exp = if self.exposure < 1 {1.0} else {(2.0 as f32).powi(self.exposure)};
        self.color * self.brightness * exp        
    }
}

pub struct LightDistanceInfo {
    pub range: f32,
    pub attenuation: Vector
}

pub struct DirectionalLight {
    pub direction: Vector,
    pub color_info: LightColorInfo 
} 

pub struct SphericalLight {
    pub position: Vector,
    pub color_info: LightColorInfo,
    pub distance_info: LightDistanceInfo,
    pub radius: f32,
    pub samples: u32
} 

pub struct RectangularLight {
    pub position: Vector,
    pub direction: Vector,
    pub rec: Rectangle,
    pub samples: u32,
    pub color_info: LightColorInfo,
    pub distance_info: LightDistanceInfo,
    pub world: Matrix,
    pub s: Vector,
    pub v1: Vector,
    pub v2: Vector,
}

impl DirectionalLight {
    pub fn new(dir: Vector, brightness: f32, color: Vector) -> Self {
        Self {
            direction: dir,
            color_info: LightColorInfo {
                brightness: brightness,
                color: color,
                exposure: 0
            }
        }
    } 

    pub fn intensity(&self) -> Vector {
        self.color_info.intensity()
    }
}

impl SphericalLight {
    pub fn new(pos: Vector, brightness: f32, color: Vector, range: f32, attenuation: Vector, radius: f32, samples: u32) -> Self {
        Self {
            position: pos,
            color_info: LightColorInfo {
                brightness: brightness,
                color: color,
                exposure: 0
            },
            distance_info: LightDistanceInfo {
                range: range,
                attenuation: attenuation
            },
            radius: radius,
            samples: samples
        }
    }

    pub fn intensity(&self) -> Vector {
        self.color_info.intensity()
    }
}

impl RectangularLight {
    pub fn new(pos: Vector, dir: Vector, width: f32, height: f32, samples: u32, brightness: f32, color: Vector, range: f32, attenuation: Vector) -> Self {
        let up = Vector::vec3(0.0, 1.0, 0.0);

        let look_at = Matrix::look_at_rh(pos, dir, up);
        let world = look_at;

        let s = Vector::vec3(-width * 0.5, -height * 0.5, 0.0) * look_at;
        let v1 = Vector::vec3(width, 0.0, 0.0);
        let v2 = Vector::vec3(0.0, height, 0.0);

        Self {
            position: pos,
            direction: dir,
            rec: Rectangle {
                width: width,
                height: height, 
            },
            samples: samples,
            color_info: LightColorInfo {
                brightness: brightness,
                color: color,
                exposure: 0
            },
            distance_info: LightDistanceInfo {
                range: range,
                attenuation: attenuation
            },
            world: world,
            s: s,
            v1: v1,
            v2: v2
        }
    }

    pub fn intensity(&self) -> Vector {
        self.color_info.intensity() / self.rec.area()
    }
}

pub enum Lights {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
    Rectangular(RectangularLight)
}