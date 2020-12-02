use crate::creature::{Creature, Diet};
use anyhow::Error;
use chrono::Local;
use plotters::{
    prelude::{ChartBuilder, IntoDrawingArea, LineSeries, PathElement, SVGBackend},
    style::{Color, IntoFont, RGBColor, BLACK, WHITE},
};
use std::{fs, path::Path};

#[derive(Debug)]
pub struct Stats {
    all: StatsInner,
    count: usize,

    herbivores: StatsInner,
    herbivores_count: usize,

    carnivores: StatsInner,
    carnivores_count: usize,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct StatsInner {
    speed: f64,
    stamina: f64,
    fov: f64,
    size: f64,
}

impl Stats {
    pub fn collect(creatures: &[Creature]) -> Self {
        let mut all = StatsInner::default();
        let mut herbivores = StatsInner::default();
        let mut carnivores = StatsInner::default();

        let mut herbivores_count = 0;
        let mut carnivores_count = 0;

        for c in creatures {
            all.speed += c.speed / creatures.len() as f64;
            all.stamina += c.stamina / creatures.len() as f64;
            all.fov += c.fov / creatures.len() as f64;
            all.size += c.size / creatures.len() as f64;

            match c.diet {
                Diet::Herbivore => {
                    herbivores.speed += c.speed;
                    herbivores.stamina += c.stamina;
                    herbivores.fov += c.fov;
                    herbivores.size += c.size;

                    herbivores_count += 1;
                }
                Diet::Carnivore => {
                    carnivores.speed += c.speed;
                    carnivores.stamina += c.stamina;
                    carnivores.fov += c.fov;
                    carnivores.size += c.size;

                    carnivores_count += 1;
                }
            }
        }

        if carnivores_count > 0 {
            carnivores.speed /= carnivores_count as f64;
            carnivores.stamina /= carnivores_count as f64;
            carnivores.fov /= carnivores_count as f64;
            carnivores.size /= carnivores_count as f64;
        } else {
            carnivores.speed = 0.5;
            carnivores.stamina = 0.5;
            carnivores.fov = 0.5;
            carnivores.size = 0.5;
        }

        if herbivores_count > 0 {
            herbivores.speed /= herbivores_count as f64;
            herbivores.stamina /= herbivores_count as f64;
            herbivores.fov /= herbivores_count as f64;
            herbivores.size /= herbivores_count as f64;
        } else {
            herbivores.speed = 0.5;
            herbivores.stamina = 0.5;
            herbivores.fov = 0.5;
            herbivores.size = 0.5;
        }

        Self {
            all,
            count: creatures.len(),

            herbivores,
            herbivores_count,

            carnivores,
            carnivores_count,
        }
    }

    pub fn write(stats: &[Self]) -> Result<(), Error> {
        const RED: RGBColor = RGBColor(222, 66, 66);
        const GREEN: RGBColor = RGBColor(66, 222, 66);
        const BLUE: RGBColor = RGBColor(66, 66, 222);
        const GREY: RGBColor = RGBColor(111, 111, 111);

        macro_rules! draw {
            ($chart:expr, $stats:expr) => {
                $chart.configure_mesh().draw()?;

                $chart
                    .draw_series(LineSeries::new(
                        $stats.iter().map(|s| s.speed).enumerate(),
                        &RED,
                    ))?
                    .label("Vitesse")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 24, y)], &RED));

                $chart
                    .draw_series(LineSeries::new(
                        $stats.iter().map(|s| s.stamina).enumerate(),
                        &GREEN,
                    ))?
                    .label("Endurance")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 24, y)], &GREEN));

                $chart
                    .draw_series(LineSeries::new(
                        $stats.iter().map(|s| s.fov).enumerate(),
                        &BLUE,
                    ))?
                    .label("Champ de vision")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 24, y)], &BLUE));

                $chart
                    .draw_series(LineSeries::new(
                        $stats.iter().map(|s| s.size).enumerate(),
                        &GREY,
                    ))?
                    .label("Taille")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 24, y)], &GREY));

                $chart
                    .configure_series_labels()
                    .background_style(&WHITE.mix(0.75))
                    .border_style(&BLACK)
                    .draw()?;
            };
        }

        let title = Local::now().format("%Y-%m-%d-%H-%M-%S");

        fs::create_dir_all("stats")?;
        let path = Path::new("stats").join(format!("{}.svg", title));

        let mut root = SVGBackend::new(&path, (1920, 1080)).into_drawing_area();
        root.fill(&WHITE)?;
        root = root.margin(32, 32, 32, 32);
        let quadrants = root.split_evenly((2, 2));

        let mut all = ChartBuilder::on(&quadrants[0])
            .caption("Caractéristiques", ("sans-serif", 32).into_font())
            .margin(32)
            .x_label_area_size(16)
            .y_label_area_size(0)
            .build_cartesian_2d(0..stats.len(), 0f64..2f64)?;
        let stats_all: Vec<StatsInner> = stats.iter().map(|s| s.all).collect();
        draw!(all, stats_all);

        let mut diets = ChartBuilder::on(&quadrants[1])
            .caption("Alimentation", ("sans-serif", 32).into_font())
            .margin(32)
            .x_label_area_size(16)
            .y_label_area_size(0)
            .build_cartesian_2d(0..stats.len(), 0f64..1f64)?;

        diets.configure_mesh().draw()?;
        diets
            .draw_series(LineSeries::new(
                stats
                    .iter()
                    .map(|s| s.herbivores_count as f64 / s.count as f64)
                    .enumerate(),
                &GREEN,
            ))?
            .label("Herbivores")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 24, y)], &GREEN));
        diets
            .draw_series(LineSeries::new(
                stats
                    .iter()
                    .map(|s| s.carnivores_count as f64 / s.count as f64)
                    .enumerate(),
                &RED,
            ))?
            .label("Carnivores")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 24, y)], &RED));
        diets
            .configure_series_labels()
            .background_style(&WHITE.mix(0.75))
            .border_style(&BLACK)
            .draw()?;

        let mut herbivores = ChartBuilder::on(&quadrants[2])
            .caption(
                "Caractéristiques (herbivores)",
                ("sans-serif", 32).into_font(),
            )
            .margin(32)
            .x_label_area_size(16)
            .y_label_area_size(0)
            .build_cartesian_2d(0..stats.len(), 0f64..2f64)?;
        let stats_herbivores: Vec<StatsInner> = stats.iter().map(|s| s.herbivores).collect();
        draw!(herbivores, stats_herbivores);

        let mut carnivores = ChartBuilder::on(&quadrants[3])
            .caption(
                "Caractéristiques (carnivores)",
                ("sans-serif", 32).into_font(),
            )
            .margin(32)
            .x_label_area_size(16)
            .y_label_area_size(0)
            .build_cartesian_2d(0..stats.len(), 0f64..2f64)?;
        let stats_carnivores: Vec<StatsInner> = stats.iter().map(|s| s.carnivores).collect();
        draw!(carnivores, stats_carnivores);

        Ok(())
    }
}
