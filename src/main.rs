mod model;
use model::*;
use ultraviolet::Vec2;
const PI: f32 = 3.141592653589;

fn main() {
    println!("Hello, world!");
}

struct Car {
    model: &'static Model,
    pos: Vec2,
    rot: f32,
}

impl Car {
    fn sensor_rotations() -> [f32; 5] {
        [-PI / 3.0, -PI / 6.0, 0.0, PI / 6.0, PI / 3.0]
    }

    pub fn state(&self) -> [f32; 5] {
        let mut res = [0.0; 5];

        for (i, rot) in Car::sensor_rotations().iter().enumerate() {
            let model = self
                .model
                .clone()
                .transform(self.pos.clone(), self.rot.clone() + rot);

            let distance = model
                .edges()
                .iter()
                .map(|line| distance_on_y_axis(*line))
                .fold(std::f32::INFINITY, |acc, dis| match dis {
                    Some(dis) if dis >= 0.0 => acc.min(dis),
                    _ => acc,
                });
            res[i] = distance;
        }

        return res;
    }
}
