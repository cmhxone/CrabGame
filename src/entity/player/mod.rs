// Player Struct
use super::velocity::Velocity;
use sdl2::rect::Rect;

pub struct Player<'p> {
    pub dst_rect: Option<Rect>,
    pub src_rect: Option<Rect>,
    pub texture: Option<sdl2::render::Texture<'p>>,
    pub velocity: Option<Velocity>,
}

impl Player<'_> {
    // Create player struct with x, y, w, h Rect
    pub fn set_rect(x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect::new(x, y, width, height)
    }
}