// Player Struct
use super::velocity::Velocity;

pub struct Player<'p> {
    pub dst_rect: Option<sdl2::rect::Rect>,
    pub src_rect: Option<sdl2::rect::Rect>,
    pub texture: Option<sdl2::render::Texture<'p>>,
    pub velocity: Option<Velocity>,
}

impl Player<'_> {
    // Create player struct with x, y, w, h Rect
    pub fn set_dst_rect(x: i32, y: i32, width: u32, height: u32) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(x, y, width, height)
    }

    pub fn set_src_rect(x: i32, y: i32, width: u32, height: u32) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(x, y, width, height)
    }
}