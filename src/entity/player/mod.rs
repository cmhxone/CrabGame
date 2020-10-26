// Player Struct
use super::velocity::Velocity;
use super::item::Item;
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

    pub fn is_collide(&self, item: &Item<'_>) -> bool {
        if ((item.dst_rect.unwrap().x < self.dst_rect.unwrap().x) && (self.dst_rect.unwrap().x < item.dst_rect.unwrap().x + item.dst_rect.unwrap().width() as i32) && (item.dst_rect.unwrap().y < self.dst_rect.unwrap().y) && (self.dst_rect.unwrap().y < item.dst_rect.unwrap().y + item.dst_rect.unwrap().height() as i32)) ||
        ((self.dst_rect.unwrap().x < item.dst_rect.unwrap().x) && (item.dst_rect.unwrap().x < self.dst_rect.unwrap().x + self.dst_rect.unwrap().width() as i32) && (self.dst_rect.unwrap().y < item.dst_rect.unwrap().y) && (item.dst_rect.unwrap().y < self.dst_rect.unwrap().y + self.dst_rect.unwrap().height() as i32)) {
            true
        } else {
            false
        }
    }
}