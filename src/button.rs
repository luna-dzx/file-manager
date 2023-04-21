use notan::{
    prelude::*,
    draw::{Draw, DrawShapes},
};

use super::State;

#[derive(Clone)]
pub struct Bounds {
    pos: (f32,f32),
    size: (f32,f32),
}

impl Bounds {
    pub fn new(pos: (f32,f32), size: (f32,f32)) -> Self {
        Self { pos, size }
    }

    pub fn contains(&self,pos:(f32,f32))->bool{
        let pos = (pos.0 - self.pos.0, pos.1 - self.pos.1);
        pos.0>=0.0 && pos.0<=self.size.0 &&
        pos.1>=0.0 && pos.1<=self.size.1
    }
}

#[derive(Clone)]
pub struct ButtonStyle {
    pub base_color: Color,
    pub hover_color: Color,
    pub click_color: Color,


}

#[derive(Clone)]
pub struct Button {
    pub func: fn(&mut State),
    pub bounds: Bounds,
    pub color: Color,
    pub mouse_buffer: bool,
}

impl Button {
    pub fn new(func: fn(&mut State), bounds: Bounds) -> Self {

        let color = Color::new(0.5,0.5,0.5,1.0);

        Self {
            func,
            bounds,
            color,
            mouse_buffer: false,
        }
    }

    pub fn draw(&self,draw:&mut Draw){
        draw.rect(self.bounds.pos, self.bounds.size)
            .corner_radius(4.0)
            .color(self.color);
    }
}