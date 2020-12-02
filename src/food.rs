use nalgebra::Point2;
use rand::Rng;

pub struct Food {
    pub eaten: bool,
    pub position: Point2<f64>,
}

impl Food {
    pub fn random(x: (f64, f64), y: (f64, f64), rng: &mut impl Rng) -> Self {
        let x = rng.gen_range(x.0, x.1);
        let y = rng.gen_range(y.0, y.1);
        Self {
            eaten: false,
            position: Point2::new(x, y),
        }
    }
}
