use glorious::{ResourceManager, Sprite};
use sdl2::rect::Rect;

use info::SpriteInfo;

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Up,
    Down,
    Left,
    Right,
    PreExit,
    Exit,

    NextLayer,
    PrevLayer,
    NextTile,
    PrevTile,
    NextColor,
    PrevColor,
    Save,

    LeftClickAt(i32, i32),
    LeftReleasedAt(i32, i32),
    RightClickAt(i32, i32),
    RightReleasedAt(i32, i32),
    MouseMovedTo(i32, i32),
}

pub struct State<'a> {
    pub resources: ResourceManager<'a, 'static>,
}

impl<'a> State<'a> {
    pub fn new(resources: ResourceManager<'a, 'static>) -> State<'a> {
        State { resources: resources }
    }

    #[inline]
    pub fn sprite(&self, info: &SpriteInfo) -> Sprite {
        let texture = self.resources.texture(&info.texture);
        let rect = info.area.map(|(x, y, w, h)| Rect::new(x as i32, y as i32, w, h));
        Sprite::new(texture, rect)
    }
}
