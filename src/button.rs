use notan::{
    prelude::*,
    draw::{Draw, DrawShapes},
};

use crate::theming::Theme;
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

    pub fn draw(&self, draw:&mut Draw, theme:&Theme){

        let color = match self.state{
            ButtonState::Neutral => theme.colors["button"],
            ButtonState::Hovered => theme.colors["button_hover"],
            ButtonState::Clicked => theme.colors["button_click"],
        };

        draw.rect(self.bounds.pos, self.bounds.size)
            .corner_radius(5.0)//BE RID OF THIS PEASENT,SIMPLY TEMPORARY
            .color(color);
    }
}

pub struct ButtonHandler {
    buttons: Vec<Button>,
}

impl ButtonHandler {
    pub fn new() -> Self{
        let buttons = Vec::new();
        Self {
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

    pub fn draw(&self,draw:&mut Draw,theme: &Theme){
        for button in self.buttons.iter() {
            button.draw(draw, theme);
        }
    }
}