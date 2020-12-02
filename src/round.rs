use crate::{
    creature::{Creature, Diet, LivingCreature},
    food::Food,
    Opts,
};
use nalgebra::{Point2, Vector2};
use rand::Rng;

pub fn run(
    creatures: &mut Vec<LivingCreature>,
    food: &mut Vec<Food>,
    rounds: usize,
    threshold: usize,
    x: (f64, f64),
    y: (f64, f64),
    opts: &Opts,
) -> bool {
    let mut ended = false;

    let mut predators = Vec::with_capacity(creatures.len());
    let zero_vector = Point2::new(0.0, 0.0) - Point2::new(0.0, 0.0);

    for _ in 0..rounds {
        for i in 0..creatures.len() {
            let (first_half, second_half) = creatures.split_at_mut(i);
            let (current, second_half) = second_half.split_first_mut().unwrap();

            if current.is_dead() {
                continue;
            }

            let mut hurry = false;

            let (food_idx, prey_idx) = match current.creature.diet {
                Diet::Herbivore => match find_food(&food, current, opts) {
                    Some((mut v, _, i)) => {
                        v.set_magnitude(0.125);
                        current.direction += v;
                        hurry = true;

                        (Some(i), None)
                    }
                    None => (None, None),
                },

                Diet::Carnivore => match find_prey(first_half, second_half, current, opts) {
                    Some((mut v, _, i)) => {
                        v.set_magnitude(0.125);
                        current.direction += v;
                        hurry = true;

                        (None, Some(i))
                    }
                    None => (None, None),
                },
            };

            predators.clear();
            let mut min_distance = f64::MAX;
            let mut max_distance = f64::MIN;
            for (v, m) in find_predators(first_half, second_half, current, opts) {
                predators.push((v, m));
                min_distance = min_distance.min(m);
                max_distance = max_distance.max(m);
            }

            if predators.len() == 1 {
                let (v, _) = &mut predators[0];
                v.set_magnitude(0.15);
                current.direction += *v;
            } else if !predators.is_empty() {
                let gap_factor = 1.0 / (max_distance - min_distance);
                let mut predators_vector = zero_vector;

                for (v, m) in &mut predators {
                    let gap = *m - min_distance;
                    v.set_magnitude(1.25 - gap * gap_factor);
                    predators_vector += *v;
                }

                predators_vector.set_magnitude(predators_vector.magnitude().min(0.25));
                current.direction += predators_vector;
                hurry = true;
            }

            let magnitude = current
                .direction
                .magnitude()
                .min(current.creature.speed(hurry, opts));
            current.direction.set_magnitude(magnitude);
            current.creature.energy -=
                2.0 / current.creature.speed(true, opts) * magnitude - current.creature.stamina;
            current.position += current.direction;

            if current.position.x < x.0 {
                current.position.x = x.1 - (x.0 - current.position.x);
            } else if current.position.x > x.1 {
                current.position.x = x.0 + (current.position.x - x.1);
            }
            if current.position.y < y.0 {
                current.position.y = y.1 - (y.0 - current.position.y);
            } else if current.position.y > y.1 {
                current.position.y = y.0 + (current.position.y - y.1);
            }

            if let Some(idx) = food_idx {
                let food = &mut food[idx];
                let distance = (current.position - food.position).magnitude().abs();
                if distance < current.creature.size(opts) {
                    food.eaten = true;
                    current.creature.energy = opts.max_energy.min(current.creature.energy + 500.0);
                }
            }

            if let Some(idx) = prey_idx {
                let prey = if idx < first_half.len() {
                    &mut first_half[idx]
                } else {
                    &mut second_half[idx - first_half.len()]
                };
                let distance = (current.position - prey.position).magnitude().abs();
                if distance < current.creature.size(opts) {
                    prey.eaten = true;
                    current.creature.energy = opts
                        .max_energy
                        .min(current.creature.energy + 500.0 + prey.creature.energy);
                    prey.creature.energy = 0.0;
                }
            }
        }

        if creatures.iter().filter(|c| !c.is_dead()).count() <= threshold {
            ended = true;
            break;
        }
    }

    creatures.retain(|c| !c.eaten);
    food.retain(|f| !f.eaten);
    ended
}

fn find_food(
    food: &[Food],
    current: &LivingCreature,
    opts: &Opts,
) -> Option<(Vector2<f64>, f64, usize)> {
    food.iter()
        .enumerate()
        .filter_map(|(i, f)| {
            if f.eaten {
                return None;
            }

            let vector = f.position - current.position;
            let magnitude_abs = vector.magnitude().abs();
            if magnitude_abs < current.creature.fov(opts) + 5.0 {
                Some((vector, magnitude_abs, i))
            } else {
                None
            }
        })
        .fold(None, food_prey_folder)
}

fn find_prey(
    first_half: &[LivingCreature],
    second_half: &[LivingCreature],
    current: &LivingCreature,
    opts: &Opts,
) -> Option<(Vector2<f64>, f64, usize)> {
    first_half
        .iter()
        .chain(second_half.iter())
        .enumerate()
        .filter_map(|(i, c)| {
            if c.eaten || !current.creature.can_prey_on(c.creature) {
                return None;
            }

            let vector = c.position - current.position;
            let magnitude_abs = vector.magnitude().abs();
            if magnitude_abs < current.creature.fov(opts) + c.creature.size(opts) {
                Some((vector, magnitude_abs, i))
            } else {
                None
            }
        })
        .fold(None, food_prey_folder)
}

fn find_predators<'a>(
    first_half: &'a [LivingCreature<'a>],
    second_half: &'a [LivingCreature<'a>],
    current: &'a LivingCreature<'a>,
    opts: &'a Opts,
) -> impl Iterator<Item = (Vector2<f64>, f64)> + 'a {
    first_half
        .iter()
        .chain(second_half.iter())
        .filter_map(move |p| {
            if p.is_dead() || !p.creature.can_prey_on(current.creature) {
                return None;
            }

            let vector = current.position - p.position;
            let magnitude_abs = vector.magnitude().abs();
            if magnitude_abs < current.creature.fov(opts) + p.creature.size(opts) {
                Some((vector, magnitude_abs))
            } else {
                None
            }
        })
}

fn food_prey_folder(
    acc: Option<(Vector2<f64>, f64, usize)>,
    current: (Vector2<f64>, f64, usize),
) -> Option<(Vector2<f64>, f64, usize)> {
    let (v, m, i) = current;
    match acc {
        Some((av, am, ai)) => {
            if m < am {
                Some((v, m, i))
            } else {
                Some((av, am, ai))
            }
        }
        None => Some((v, m, i)),
    }
}

pub fn generate_food(
    food_buf: &mut Vec<Food>,
    qty: usize,
    x: (f64, f64),
    y: (f64, f64),
    rng: &mut impl Rng,
) {
    food_buf.clear();
    for _ in 0..qty {
        food_buf.push(Food::random(x, y, rng));
    }
}

pub fn position_creatures<'a>(
    creatures: &'a mut [Creature],
    creatures_buf: &mut Vec<LivingCreature<'a>>,
    x: (f64, f64),
    y: (f64, f64),
    rng: &mut impl Rng,
) {
    creatures_buf.clear();
    for c in creatures
        .iter_mut()
        .map(|c| LivingCreature::random(c, x, y, rng))
    {
        creatures_buf.push(c);
    }
}
