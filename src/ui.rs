use anyhow::Error;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, TextureQuery},
    rwops::RWops,
    ttf::{Font, Sdl2TtfContext},
    video::WindowContext,
};

static FONT_DATA: &[u8] = include_bytes!("../resources/Roboto-Medium.ttf");

pub fn load_font<'ttf>(ctx: &'ttf Sdl2TtfContext) -> Result<Font<'ttf, 'static>, Error> {
    let rwops = RWops::from_bytes(FONT_DATA).map_err(Error::msg)?;
    ctx.load_font_from_rwops(rwops, 32).map_err(Error::msg)
}

type UI<'a> = (
    (Texture<'a>, Rect),
    (Texture<'a>, Rect),
    (Texture<'a>, Rect),
);
pub fn render<'a>(
    generation: usize,
    paused: bool,
    speed: usize,
    debug: bool,
    font: &Font,
    tc: &'a TextureCreator<WindowContext>,
) -> Result<UI<'a>, Error> {
    const PADDING: i32 = 16;

    let l1 = format!("Génération {}", generation);
    let l2 = format!("{} (x{})", if paused { "Pause" } else { "Play" }, speed);
    let l3 = if debug {
        "Vue détaillée"
    } else {
        "Vue normale"
    };

    let t1 = tc.create_texture_from_surface(font.render(&l1).blended(Color::BLACK)?)?;
    let t2 = tc.create_texture_from_surface(font.render(&l2).blended(Color::BLACK)?)?;
    let t3 = tc.create_texture_from_surface(font.render(l3).blended(Color::BLACK)?)?;

    let TextureQuery {
        width: w1,
        height: h1,
        ..
    } = t1.query();
    let r1 = Rect::new(PADDING, PADDING, w1, h1);

    let TextureQuery {
        width: w2,
        height: h2,
        ..
    } = t2.query();
    let r2 = Rect::new(PADDING, PADDING + h1 as i32 + PADDING, w2, h2);

    let TextureQuery {
        width: w3,
        height: h3,
        ..
    } = t3.query();
    let r3 = Rect::new(
        PADDING,
        PADDING + h1 as i32 + PADDING + h2 as i32 + PADDING,
        w3,
        h3,
    );

    Ok(((t1, r1), (t2, r2), (t3, r3)))
}
