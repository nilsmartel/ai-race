// Code to hold information about 2d structres (such as the racetrack)

use ultraviolet::{Lerp, Mat2, Vec2};

#[derive(Clone, Debug)]
pub struct Model {
    verts: Vec<Vec2>,
    edges: Vec<(usize, usize)>,
}

impl Model {
    pub fn new(verts: Vec<Vec2>, edges: Vec<(usize, usize)>) -> Self {
        Self { verts, edges }
    }

    pub fn transform(mut self, pos: Vec2, rot: f32) -> Model {
        let rot: Mat2 = {
            let c = rot.cos();
            let s = rot.sin();
            Mat2::new(Vec2::new(c, -s), Vec2::new(s, c))
        };

        for vert in self.verts.iter_mut() {
            *vert -= pos;
            *vert = rot * *vert;
        }

        self
    }

    pub fn edges(&self) -> Vec<(Vec2, Vec2)> {
        self.edges
            .iter()
            .map(|(a, b)| (self.verts[*a], self.verts[*b]))
            .collect()
    }
}

// returns the distance from (0, 0), where a line intersects the y axis
pub fn distance_on_y_axis(line: (Vec2, Vec2)) -> Option<f32> {
    let (mut first, mut second) = line;

    // ensure that first is on the left and second is on the right
    if first.x > second.x {
        let tmp = second.clone();
        second = first;
        first = tmp;
    }

    // return None, if line doesn't cross the y axis
    if first.x > 0.0 || second.x <= 0.0 {
        return None;
    }

    let len_x = second.x - first.x;
    let fraction = (-first.x) as f32 / len_x as f32;

    // distance to y axis is fraction between first and second points distance
    let distance = first.y.lerp(second.x, fraction);

    Some(distance)
}
