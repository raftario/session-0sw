use rand::Rng;

use crate::{creature::Creature, Opts};

impl Creature {
    pub fn select(creatures: &mut Vec<Self>, qty: usize, opts: &Opts, rng: &mut impl Rng) {
        let mut total_energy = 0.0;
        for c in creatures.iter() {
            total_energy += c.energy;
        }

        while creatures.len() < qty {
            let idx1 = rng.gen_range(0.0, total_energy);
            let idx2 = rng.gen_range(0.0, total_energy);

            let mut c1 = None;
            let mut i1 = 0.0;
            for c in creatures.iter() {
                i1 += c.energy;
                if i1 >= idx1 {
                    c1 = Some(c);
                    break;
                }
            }
            let c1 = c1.unwrap();

            let mut c2 = None;
            let mut i2 = 0.0;
            for c in creatures.iter() {
                i2 += c.energy;
                if i2 >= idx2 {
                    c2 = Some(c);
                    break;
                }
            }
            let c2 = c2.unwrap();

            let c = Creature::breed(c1, c2, opts, rng);
            creatures.push(c);
        }
    }
}
