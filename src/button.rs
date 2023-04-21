use notan::{
    prelude::*,
    draw::{Draw, DrawShapes},
};

use super::State;

pub enum ButtonState {
    Neutral,
    Hovered,
    Clicked
}

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

pub struct ButtonStyle {
    pub base_color: Color,
    pub hover_color: Color,
    pub click_color: Color,
    pub corner_radius: f32,
}

pub struct Button {
    pub func: fn(&mut State),
    pub bounds: Bounds,
    pub state: ButtonState,
    pub mouse_buffer: bool,
}

impl Button {
    pub fn new(func: fn(&mut State), bounds: Bounds) -> Self {

        Self {
            func,
            bounds,
            state: ButtonState::Neutral,
            mouse_buffer: false,
        }
    }

    pub fn draw(&self, draw:&mut Draw, style:&ButtonStyle){

        let color = match self.state{
            ButtonState::Neutral => style.base_color,
            ButtonState::Hovered => style.hover_color,
            ButtonState::Clicked => style.click_color,
        };

        draw.rect(self.bounds.pos, self.bounds.size)
            .corner_radius(style.corner_radius)
            .color(color);
    }
}

pub struct ButtonHandler {
    style: ButtonStyle,
    buttons: Vec<Button>,
}

impl ButtonHandler {
    pub fn new(style: ButtonStyle) -> Self{
        let buttons = Vec::new();
        Self {
            style,
            buttons
        }
    }

    pub fn add(&mut self, func: fn(&mut State), bounds: Bounds){
        self.buttons.push(Button::new(func,bounds))
    }

    pub fn update(&mut self, pos: (f32,f32), clicked: bool) -> fn(&mut State){

        fn do_nothing(_: &mut State){}
        let mut func : fn(&mut State);
        func = do_nothing;

        for i in 0..self.buttons.len() {
            // mouse buffer
            if !clicked {
                self.buttons[i].mouse_buffer = false;
            }

            // if mouse outside button
            if !self.buttons[i].bounds.contains(pos){
                self.buttons[i].state = ButtonState::Neutral;
                continue
            }

            // mouse inside button:

            // if mouse not clicked
            if !clicked {
                self.buttons[i].state = ButtonState::Hovered;
                continue
            }

            // mouse clicked and inside button:

            self.buttons[i].state = ButtonState::Clicked;

            if !self.buttons[i].mouse_buffer {
                func = self.buttons[i].func;
                self.buttons[i].mouse_buffer = true;
            }
        }

        func
    }

    pub fn draw(&self,draw:&mut Draw){
        for button in self.buttons.iter() {
            button.draw(draw, &self.style);
        }
    }
}