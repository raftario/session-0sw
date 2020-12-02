use crate::{
    creature::{Creature, Diet, LivingCreature},
    food::Food,
    Opts,
};
use anyhow::Error;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color};

const U8_MAX_F64: f64 = u8::MAX as f64;

impl Creature {
    pub fn colour(speed: f64, stamina: f64, fov: f64) -> Color {
        let r = (speed * U8_MAX_F64) as u8;
        let g = (stamina * U8_MAX_F64) as u8;
        let b = (fov * U8_MAX_F64) as u8;
        Color::RGB(r, g, b)
    }
}

impl LivingCreature<'_> {
    pub fn draw(
        &self,
        renderer: &impl DrawRenderer,
        debug: bool,
        opts: &Opts,
    ) -> Result<(), Error> {
        let (x, y) = (self.position.x as _, self.position.y as _);
        renderer
            .filled_circle(x, y, self.creature.size(opts) as _, self.creature.colour)
            .map_err(Error::msg)?;

        let mut eye_vector = self.direction;
        eye_vector.set_magnitude(self.creature.size(opts) * 0.33);
        let eye_position = self.position + eye_vector;
        let (eye_x, eye_y) = (eye_position.x as _, eye_position.y as _);
        let eye_colour = match self.creature.diet {
            Diet::Herbivore => (0, u8::MAX / 2, 0, u8::MAX),
            Diet::Carnivore => (u8::MAX / 2, 0, 0, u8::MAX),
        };

        renderer
            .filled_circle(eye_x, eye_y, 4, Color::WHITE)
            .map_err(Error::msg)?;
        renderer
            .filled_circle(eye_x, eye_y, 2, eye_colour)
            .map_err(Error::msg)?;

        if debug && !self.is_dead() {
            renderer
                .circle(x, y, self.creature.fov(opts) as _, Color::BLACK)
                .map_err(Error::msg)?;

            let display_direction = self.direction * 16.0;
            let display_direction_end = self.position + display_direction;
            let (direction_x, direction_y) =
                (display_direction_end.x as _, display_direction_end.y as _);
            renderer
                .line(x, y, direction_x, direction_y, Color::RED)
                .map_err(Error::msg)?;
        }

        Ok(())
    }
}

impl Food {
    pub fn draw(&self, renderer: &impl DrawRenderer) -> Result<(), Error> {
        renderer
            .filled_circle(
                self.position.x as _,
                self.position.y as _,
                5,
                (0, 0, 0, u8::MAX),
            )
            .map_err(Error::msg)
    }
}
