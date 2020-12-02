mod breeding;
mod creature;
mod display;
mod food;
mod mutation;
mod round;
mod selection;
mod stats;
mod ui;

use crate::{creature::Creature, stats::Stats};
use anyhow::Error;
use clap::{
    AppSettings::{ColoredHelp, DeriveDisplayOrder, DisableVersion, NextLineHelp},
    Clap,
};
use rand::prelude::{Rng, SeedableRng, SmallRng};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::{
    thread,
    time::{Duration, Instant},
};

const NAME: &str = "Projet de session - 0SW - Raphaël Thériault";

/// Simulation génétique pour le cours 0SW.
/// Espace pour pauser/reprendre,
/// haut/bas pour controller le multiplicateur de vitesse,
/// D pour activer/désactiver la vue détaillée.
#[derive(Debug, Clap)]
#[clap(
    name = NAME,
    setting = DeriveDisplayOrder,
    setting = NextLineHelp,
    setting = DisableVersion,
    setting = ColoredHelp
)]
pub struct Opts {
    /// Nombre de créatures par génération
    #[clap(short, long, default_value = "100")]
    creature_count: usize,

    /// Quantité de nourriture disponible par génération
    #[clap(short, long, default_value = "100")]
    food_count: usize,

    /// Quantité minimale de créatures en vie pour continuer la génération
    #[clap(short, long, default_value = "50")]
    generation_threshold: usize,

    /// Multiplicateur de vitesse de départ
    #[clap(short, long, default_value = "1")]
    start_speed: usize,

    /// Probability de mutation
    #[clap(long, default_value = "0.05")]
    mutation_probability: f64,

    /// Ampleur des mutation
    #[clap(long, default_value = "1.0")]
    mutation_scale: f64,

    /// Énergie maximale
    #[clap(long, default_value = "2000.0")]
    pub max_energy: f64,
    /// Énergie de départ
    #[clap(long, default_value = "1000.0")]
    pub start_energy: f64,

    /// Vitesse de base pour toutes les créatures
    #[clap(long, default_value = "1.33")]
    pub base_speed: f64,
    /// Multiplicateur appliqué à la vitesse individuelle de chaque créature
    #[clap(long, default_value = "0.67")]
    pub speed_factor: f64,
    /// Multiplicateur appliqué quand une créature ne voit ni nourriture ni prédateurs
    #[clap(long, default_value = "0.75")]
    pub normal_speed: f64,

    /// Multiplicateur appliqué à l'endurance individuelle de chaque créature
    #[clap(long, default_value = "1.0")]
    pub stamina_factor: f64,

    /// Champ de vision de base pour toutes les créatures
    #[clap(long, default_value = "45.0")]
    pub base_fov: f64,
    /// Multiplicateur appliqué au champ de vision individuel de chaque créature
    #[clap(long, default_value = "55.0")]
    pub fov_factor: f64,

    /// Taille de base pour toutes les créatures
    #[clap(long, default_value = "5.0")]
    pub base_size: f64,
    /// Multiplicateur appliqué à la taille individuelle de chaque créature
    #[clap(long, default_value = "5.0")]
    pub size_factor: f64,
}

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    let sdl = sdl2::init().map_err(Error::msg)?;
    let video_subsystem = sdl.video().map_err(Error::msg)?;
    let window = video_subsystem
        .window(NAME, 500, 500)
        .position_centered()
        .resizable()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let mut events = sdl.event_pump().map_err(Error::msg)?;
    canvas.set_logical_size(1000, 1000)?;

    let ttf_ctx = sdl2::ttf::init()?;
    let font = ui::load_font(&ttf_ctx)?;
    let tc = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut rng = SmallRng::from_entropy();
    let (x, y) = ((0.0, 1000.0), (0.0, 1000.0));

    let mut creatures: Vec<Creature> = (0..opts.creature_count)
        .map(|_| Creature::random(&opts, &mut rng))
        .collect();

    let mut generation = 1;
    let mut paused = false;
    let mut debug = false;
    let mut speed = opts.start_speed;

    let mut text = ui::render(generation, paused, speed, debug, &font, &tc)?;
    let mut ui_needs_refresh = true;

    let mut stats = Vec::new();

    'main: loop {
        let mut living_creatures = Vec::with_capacity(creatures.len());
        crate::round::position_creatures(&mut creatures, &mut living_creatures, x, y, &mut rng);

        let mut food = Vec::with_capacity(opts.food_count);
        crate::round::generate_food(&mut food, opts.food_count, x, y, &mut rng);

        let mut last_frame = Instant::now();
        let mut delta_time;

        loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,

                    Event::KeyUp {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        paused = !paused;
                        ui_needs_refresh = true;
                    }

                    Event::KeyUp {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        debug = !debug;
                        ui_needs_refresh = true;
                    }

                    Event::KeyUp {
                        keycode: Some(Keycode::Up),
                        ..
                    } if speed < 256 => {
                        speed += 1;
                        ui_needs_refresh = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Down),
                        ..
                    } if speed > 1 => {
                        speed -= 1;
                        ui_needs_refresh = true;
                    }

                    _ => (),
                }
            }

            if !paused
                && crate::round::run(
                    &mut living_creatures,
                    &mut food,
                    speed,
                    opts.generation_threshold,
                    x,
                    y,
                    &opts,
                )
            {
                break;
            }

            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.clear();

            for f in &food {
                f.draw(&canvas)?;
            }
            for c in &living_creatures {
                c.draw(&canvas, debug, &opts)?;
            }

            if ui_needs_refresh {
                text = ui::render(generation, paused, speed, debug, &font, &tc)?;
                ui_needs_refresh = false;
            }
            canvas
                .copy(&text.0 .0, None, Some(text.0 .1))
                .map_err(Error::msg)?;
            canvas
                .copy(&text.1 .0, None, Some(text.1 .1))
                .map_err(Error::msg)?;
            canvas
                .copy(&text.2 .0, None, Some(text.2 .1))
                .map_err(Error::msg)?;

            canvas.present();

            delta_time = last_frame.elapsed();
            last_frame = Instant::now();
            thread::sleep(Duration::new(
                0,
                (1_000_000_000u32 / 60).saturating_sub(delta_time.subsec_nanos()),
            ));
        }

        creatures.retain(|c| c.energy > 0.0);
        if creatures.is_empty() {
            break;
        }

        Creature::select(&mut creatures, opts.creature_count, &opts, &mut rng);
        for c in creatures.iter_mut() {
            if rng.gen_bool(opts.mutation_probability) {
                c.mutate(opts.mutation_scale, &mut rng);
            }
            c.energy = opts.start_energy;
        }

        stats.push(Stats::collect(&creatures));
        generation += 1;
        ui_needs_refresh = true;
    }

    Stats::write(&stats)
}
