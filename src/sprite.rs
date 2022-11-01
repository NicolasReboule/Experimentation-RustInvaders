use ggez::graphics;

pub trait Sprite {
    fn set_rect(&mut self);
    fn set_path(&mut self, path : String);
    fn get_path(&self) -> &String;
    fn get_rect(&self) -> &graphics::Rect;
    fn set_position(&mut self, position : (f32, f32));
    fn get_position(&self) -> &(f32, f32);
    fn set_width(&mut self, width : f32);
    fn get_width(&self) -> &f32;
    fn set_height(&mut self, height : f32);
    fn get_height(&self) -> &f32;
    fn set_scale(&mut self, scale : (f32, f32));
    fn get_scale(&self) -> &(f32, f32);
    fn get_hitbox(&self) -> graphics::Rect;
}