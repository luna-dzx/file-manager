use notan::draw::*;
use notan::prelude::*;
use std::ops::Deref;

mod button;
use button::*;

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default()
        .transparent(true)
        .decorations(false)
        .resizable(true);

    notan::init_with(State::new)
        .add_config(win)
        .add_config(DrawConfig)
        .update(update)// Simple way to add the draw extension
        .draw(draw)
        .build()
}

#[derive(AppState, Clone)]
pub struct State {
    time: f32,
    buttons: Vec<Button>,
}

fn hello_world(){
    println!("hello world")
}

impl State {
    fn new(_gfx: &mut Graphics) -> Self {
        let bounds = Bounds::new((100.0,100.0),(100.0,50.0));
        let buttons = Vec::new();

        let mut state = Self {
            time: 0.0,
            buttons,
        };

        state.buttons.push(Button::new(State::test_state,bounds));
        state.buttons.push(Button::new(|_| println!("AAAAAAAAAAA\nwriting cool little closure"),
                                       Bounds::new((100.0,300.0),(100.0,50.0))));

        state
    }

    fn process_buttons(&mut self, app: &mut App){
        let pos = app.mouse.position();
        let clicked = app.mouse.left_is_down();

        for i in 0..self.buttons.len() {
            // mouse buffer
            if !clicked {
                self.buttons[i].mouse_buffer = false;
            }

            // if mouse outside button
            if !self.buttons[i].bounds.contains(pos){
                self.buttons[i].color = Color::new(0.5,0.5,0.5,1.0);
                continue
            }

            // mouse inside button:

            // if mouse not clicked
            if !clicked {
                self.buttons[i].color = Color::new(0.8,0.8,0.8,1.0);
                continue
            }

            // mouse clicked and inside button:

            self.buttons[i].color = Color::new(1.0,1.0,1.0,1.0);

            if !self.buttons[i].mouse_buffer {
                (self.buttons[i].func)(self);
                self.buttons[i].mouse_buffer = true;
            }
        }
    }

    fn draw_buttons(&mut self, draw: &mut Draw)
    {
        for button in self.buttons.iter_mut() {
            button.draw(draw);
        }
    }

    pub fn test_state(state: &mut State){
        println!("woah wahst aaaaa {}",state.time);
    }
}


fn update(app: &mut App, state: &mut State) {
    state.time += app.timer.delta_f32();
    state.process_buttons(app);
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.clear(Color::TRANSPARENT);
    let size = gfx.size();
    draw.triangle((0.5, 0.0), (0.0, 1.0), (1.0, 1.0))
        .scale(size.0 as f32,size.1 as f32)
        .color(Color::MAGENTA);

    state.draw_buttons(&mut draw);

    gfx.render(&draw);
}
