use crate::creature::{Creature, Diet};
use rand::Rng;

impl Creature {
    pub fn mutate(&mut self, scale: f64, rng: &mut impl Rng) {
        let speed = self.speed + rng.gen_range(0.0, scale);
        let stamina = self.stamina + rng.gen_range(0.0, scale);
        let fov = self.fov + rng.gen_range(0.0, scale);
        let size = self.size + rng.gen_range(0.0, scale);

        let capped_factor = 2.0 / (speed + stamina + fov + size);

        self.speed = speed * capped_factor;
        self.stamina = stamina * capped_factor;
        self.fov = fov * capped_factor;
        self.size = size * capped_factor;

        if rng.gen_bool((scale / 2.0).min(0.5)) {
            self.diet = match self.diet {
                Diet::Herbivore => Diet::Carnivore,
                Diet::Carnivore => Diet::Herbivore,
            }
        }

        self.colour = Self::colour(self.speed, self.stamina, self.fov);
    }
}
