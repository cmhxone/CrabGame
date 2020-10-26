use sdl2::rect::Rect;

pub struct Item<'i> {
    pub src_rect: Option<Rect>,
    pub dst_rect: Option<Rect>,
    pub texture: Option<sdl2::render::Texture<'i>>,
}

impl Item<'_> {
    pub fn set_rect(x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect::new(x, y, width, height)
    }
}