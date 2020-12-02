use crate::{creature::Creature, Opts};
use rand::Rng;

impl Creature {
    pub fn breed(creature_1: &Self, creature_2: &Self, opts: &Opts, rng: &mut impl Rng) -> Self {
        let factor_1: f64 = rng.gen();
        let factor_2 = 1.0 - factor_1;

        let mut speed = (creature_1.speed * factor_1) + (creature_2.speed * factor_2);
        let mut stamina = (creature_1.stamina * factor_1) + (creature_2.stamina * factor_2);
        let mut fov = (creature_1.fov * factor_1) + (creature_2.fov * factor_2);
        let mut size = (creature_1.size * factor_1) + (creature_2.size * factor_2);

        let capped_factor = 2.0 / (speed + stamina + fov + size);

        speed *= capped_factor;
        stamina *= capped_factor;
        fov *= capped_factor;
        size *= capped_factor;

        let diet = if factor_1 >= 0.5 {
            creature_1.diet
        } else {
            creature_2.diet
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
}
