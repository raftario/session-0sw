use crate::Opts;
use nalgebra::{Point2, Vector2};
use rand::Rng;
use sdl2::pixels::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Creature {
    pub energy: f64,

    // Capped
    pub speed: f64,
    pub stamina: f64,
    pub fov: f64,
    pub size: f64,

    // Arbitrary
    pub diet: Diet,

    pub colour: Color,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Diet {
    Herbivore,
    Carnivore,
}

#[derive(Debug, PartialEq)]
pub struct LivingCreature<'a> {
    pub creature: &'a mut Creature,
    pub eaten: bool,
    pub position: Point2<f64>,
    pub direction: Vector2<f64>,
}

impl Creature {
    pub fn random(opts: &Opts, rng: &mut impl Rng) -> Self {
        let mut speed: f64 = rng.gen();
        let mut stamina: f64 = rng.gen();
        let mut fov: f64 = rng.gen();
        let mut size: f64 = rng.gen();

        let capped_factor = 2.0 / (speed + stamina + fov + size);

        speed *= capped_factor;
        stamina *= capped_factor;
        fov *= capped_factor;
        size *= capped_factor;

        let diet = if rng.gen_bool(0.5) {
            Diet::Carnivore
        } else {
            Diet::Herbivore
        };

        Self {
            energy: opts.start_energy,

            speed,
            stamina,
            fov,
            size,

            diet,

            colour: Self::colour(speed, stamina, fov),
        }
    }

    pub fn speed(&self, hurry: bool, opts: &Opts) -> f64 {
        let speed = self.speed * opts.speed_factor + opts.base_speed;
        if hurry {
            speed
        } else {
            speed * opts.normal_speed
        }
    }

    pub fn fov(&self, opts: &Opts) -> f64 {
        self.fov * opts.fov_factor + opts.base_fov
    }

    pub fn size(&self, opts: &Opts) -> f64 {
        self.size * opts.size_factor + opts.base_size
    }

    pub fn can_prey_on(&self, other: &Self) -> bool {
        match self.diet {
            Diet::Herbivore => false,
            Diet::Carnivore => match (other.diet, other.energy <= 0.0) {
                (_, true) => true,
                (Diet::Herbivore, _) => other.size - self.size <= 0.25,
                (Diet::Carnivore, _) => self.size > other.size,
            },
        }
    }
}

impl<'a> LivingCreature<'a> {
    pub fn random(
        creature: &'a mut Creature,
        x: (f64, f64),
        y: (f64, f64),
        rng: &mut impl Rng,
    ) -> Self {
        let x = rng.gen_range(x.0, x.1);
        let y = rng.gen_range(y.0, y.1);

        let mut direction =
            Point2::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0)) - Point2::origin();
        direction.set_magnitude((1.0 + creature.speed) * 0.75);

        Self {
            creature,
            eaten: false,
            position: Point2::new(x, y),
            direction,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.eaten || self.creature.energy <= 0.0
    }
}
