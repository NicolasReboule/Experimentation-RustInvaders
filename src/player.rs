use ggez::{graphics};
use crate::sprite::Sprite;

pub struct Player {
    _rect: graphics::Rect,
    _path : String,
    _scale: (f32, f32),
    _width: f32,
    _height: f32,
    _position: (f32, f32),
}

impl Player {
    pub fn new(path : String, scale : (f32, f32), width : f32, height : f32, position : (f32, f32)) -> Self {
        Self { _rect: (graphics::Rect { x: (0.0), y: (0.0), w: (0.0), h: (0.0) }), 
        _scale: (scale),
            _width: (width),
            _height: (height),
            _position: (position),
            _path: (path) }
    }
}

impl Sprite for Player {
    fn set_path(&mut self, path : String) {
        self._path = path;
    }

    fn get_path(&self) -> &String {
        &self._path
    }

    fn set_rect(&mut self) {
        self._rect = graphics::Rect::new(self._position.0, self._position.1, self._width, self._height);
    }

    fn get_rect(&self) -> &graphics::Rect {
        &self._rect
    }

    fn set_position(&mut self, position : (f32, f32)) {
        self._position = position;
    }
    
    fn get_position(&self) -> &(f32, f32) {
        &self._position
    }

    fn set_width(&mut self, width : f32) {
        self._width = width;
    }
    
    fn get_width(&self) -> &f32 {
        &self._width
    }

    fn set_height(&mut self, height : f32) {
        self._height = height;
    }
    
    fn get_height(&self) -> &f32 {
        &self._height
    }

    fn set_scale(&mut self, scale : (f32, f32)) {
        self._scale = scale;
    }
    
    fn get_scale(&self) -> &(f32, f32) {
        &self._scale
    }

    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = graphics::Rect::new(0.0, 0.0, self._width, self._height);
        r.scale(self._scale.0, self._scale.1);
        r
    }
 }