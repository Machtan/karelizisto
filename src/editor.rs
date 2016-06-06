use std::cmp;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use glorious::{Behavior, Color, Label, Renderer};
use sdl2::rect::Rect;

use common::{Message, State};
use level::{Layer, Level};
use toolbox::Tool;

#[derive(Debug, Clone)]
pub struct Viewport {
    // (x, y, x', y')
    pub model: (i32, i32, i32, i32),
    pub view: (i32, i32, i32, i32),
}

fn convert(value: (i32, i32), from: (i32, i32, i32, i32), to: (i32, i32, i32, i32)) -> (i32, i32) {
    // Conversion to float to ensure rounding towards negative infinity.
    let x = (((value.0 - from.0) * (to.2 - to.0)) as f64 /
             (from.2 - from.0) as f64)
        .floor() as i32 + to.0;
    let y = (((value.1 - from.1) * (to.3 - to.1)) as f64 /
             (from.3 - from.1) as f64)
        .floor() as i32 + to.1;
    (x, y)
}

fn convert_rect(rect: Rect, from: (i32, i32, i32, i32), to: (i32, i32, i32, i32)) -> Rect {
    let a = (rect.x(), rect.y());
    let b = (rect.x() + rect.width() as i32, rect.y() + rect.height() as i32);

    let c = convert(a, from, to);
    let d = convert(b, from, to);

    let x1 = cmp::min(c.0, d.0);
    let x2 = cmp::max(c.0, d.0);
    let y1 = cmp::min(c.1, d.1);
    let y2 = cmp::max(c.1, d.1);

    Rect::new(x1, y1, (x2 - x1) as u32, (y2 - y1) as u32)
}

impl Viewport {
    #[inline]
    pub fn model_to_view(&self, point: (i32, i32)) -> (i32, i32) {
        convert(point, self.model, self.view)
    }

    #[inline]
    pub fn view_to_model(&self, point: (i32, i32)) -> (i32, i32) {
        convert(point, self.view, self.model)
    }

    #[inline]
    pub fn model_to_view_rect(&self, rect: Rect) -> Rect {
        convert_rect(rect, self.model, self.view)
    }

    #[inline]
    pub fn view_to_model_rect(&self, rect: Rect) -> Rect {
        convert_rect(rect, self.view, self.model)
    }

    #[inline]
    pub fn translate(&mut self, delta: (i32, i32)) {
        self.model.0 += delta.0;
        self.model.1 += delta.1;
        self.model.2 += delta.0;
        self.model.3 += delta.1;
    }
}

fn insert_tile(layer: &mut Layer, tile: &str, pos: (i32, i32)) {
    remove_tile(layer, pos);
    if !layer.contains_key(tile) {
        layer.insert(tile.to_owned(), BTreeSet::new());
    }
    layer.get_mut(tile).expect("unreachable; insert failed").insert(pos);
}

fn remove_tile(layer: &mut Layer, pos: (i32, i32)) {
    for positions in layer.values_mut() {
        positions.remove(&pos);
    }
}

#[derive(Debug, Clone)]
pub struct Editor {
    layers: Vec<String>,
    tile_textures: BTreeMap<String, String>,
    current_layer: usize,
    level: Level,
    viewport: Viewport,
    tool: Tool,
    current_tile: usize,
    tiles: Vec<String>,
    button_down: u8,
    prev_point: (i32, i32),
    save_to: Option<PathBuf>,
}

impl Editor {
    pub fn new<P>(layers: Vec<String>,
                  tile_textures: BTreeMap<String, String>,
                  level: Level,
                  save_to: Option<P>)
                  -> Editor
        where P: Into<PathBuf>
    {
        if save_to.is_none() {
            warn!("The editor is in no-save mode!");
        }
        for layer in level.layers.keys() {
            assert!(layers.iter().any(|l| l == layer),
                    "layer not known to schema: {:?}",
                    layer);
        }
        let num_layers = layers.len();
        let tiles = tile_textures.keys().cloned().collect();
        Editor {
            layers: layers,
            tile_textures: tile_textures,
            current_layer: num_layers - 1,
            level: level,
            viewport: Viewport {
                model: (0, 0, 20, 15),
                view: (0, 600, 800, 0),
            },
            tool: Tool::Paint,
            current_tile: 0,
            tiles: tiles,
            button_down: 0,
            prev_point: (0, 0),
            save_to: save_to.map(|p| p.into()),
        }
    }

    #[inline]
    pub fn next_layer(&mut self) {
        self.current_layer = cmp::min(self.current_layer + 1, self.layers.len() - 1);
    }

    #[inline]
    pub fn prev_layer(&mut self) {
        self.current_layer = self.current_layer.saturating_sub(1);
    }

    #[inline]
    pub fn next_tile(&mut self) {
        self.current_tile = (self.current_tile + 1) % self.tiles.len();
    }

    #[inline]
    pub fn prev_tile(&mut self) {
        self.current_tile = (self.current_tile + self.tiles.len() - 1) % self.tiles.len();
    }

    pub fn mouse_click(&mut self, view_coord: (i32, i32), button: u8) {
        self.prev_point = view_coord;
        self.button_down = button;
        self.mouse_move(view_coord);
    }

    pub fn paint(&mut self, view_coord: (i32, i32), erase: bool) {
        let layer_name = &self.layers[self.current_layer];

        if !self.level.layers.contains_key(layer_name) {
            self.level.layers.insert(layer_name.to_owned(), Layer::new());
        }
        let layer = self.level.layers.get_mut(layer_name).expect("unreachable; insert failed");
        let pos = self.viewport.view_to_model(view_coord);
        let tile = &self.tiles[self.current_tile];

        if erase {
            remove_tile(layer, pos);
        } else {
            insert_tile(layer, tile, pos);
        }
    }

    pub fn mouse_move(&mut self, view_coord: (i32, i32)) {
        match self.button_down {
            0 => {}
            1 => self.paint(view_coord, false),
            2 => self.paint(view_coord, true),
            _ => unreachable!(),
        }
    }
}

impl<'a> Behavior<State<'a>> for Editor {
    type Message = Message;

    fn handle(&mut self, state: &mut State<'a>, message: Message, queue: &mut Vec<Message>) {
        use common::Message::*;

        trace!("{:?}", message);
        match message {
            Up => self.viewport.translate((0, 1)),
            Left => self.viewport.translate((-1, 0)),
            Down => self.viewport.translate((0, -1)),
            Right => self.viewport.translate((1, 0)),

            NextLayer => self.next_layer(),
            PrevLayer => self.prev_layer(),
            NextTile => self.next_tile(),
            PrevTile => self.prev_tile(),
            Save => {
                match self.save_to {
                    Some(ref path) => {
                        info!("Saving level to {:?}", path);
                        self.level.save(path).unwrap();
                    }
                    None => warn!("Editor is in no-save mode!"),
                }
            }

            LeftClickAt(x, y) => self.mouse_click((x, y), 1),
            RightClickAt(x, y) => self.mouse_click((x, y), 2),
            MouseMovedTo(x, y) => self.mouse_move((x, y)),
            LeftReleasedAt(..) |
            RightReleasedAt(..) => self.button_down = 0,

            PreExit => {
                if let Some(ref path) = self.save_to {
                    info!("Saving level to {:?}", path);
                    self.level.save(path).unwrap();
                }
                queue.push(Exit);
            }
            Exit => unreachable!(),
        }
    }

    fn render(&mut self, state: &State<'a>, renderer: &mut Renderer) {
        for (i, layer_name) in self.layers[..self.current_layer + 1].iter().enumerate() {
            if i == self.current_layer {
                renderer.set_draw_color(Color(0xff, 0xff, 0xff, 0x77));
                renderer.fill_rect(Rect::new(0, 0, 800, 600)).unwrap();
            }
            let layer = match self.level.layers.get(layer_name) {
                Some(layer) => layer,
                None => continue,
            };
            for (tile, positions) in layer {
                let path = &self.tile_textures[tile];
                let texture = &state.resources.texture(path);
                for pos in positions {
                    let model_rect = Rect::new(pos.0 as i32, pos.1 as i32, 1, 1);
                    let view_rect = self.viewport.model_to_view_rect(model_rect);
                    renderer.copy(texture, None, Some(view_rect));
                }
            }
        }

        let info_box = Rect::new(700, 20, 80, 80);
        renderer.set_draw_color(Color(0x00, 0x00, 0x00, 0x77));
        renderer.fill_rect(info_box).unwrap();
        let tile_rect = Rect::new(708, 28, 64, 64);
        let tile_name = &self.tiles[self.current_tile];
        let texture = &state.resources.texture(&self.tile_textures[tile_name]);
        renderer.copy(texture, None, Some(tile_rect));
    }
}
