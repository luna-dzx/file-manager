mod button;
mod file_list;
mod theming;

use notan::draw::*;
use notan::prelude::*;
use std::ops::Deref;
use std::os::linux::raw::stat;
use button::*;
use file_list::*;
use theming::*;

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default()
        .transparent(true)
        .decorations(true)
        .resizable(true);

    notan::init_with(State::new)
        .add_config(win)
        .add_config(DrawConfig)
        .update(update)// Simple way to add the draw extension
        .draw(draw)
        .build()
}

#[derive(AppState)]
pub struct State {
    time: f32,
    button_handler: ButtonHandler,
    theme: Theme,
    file_list: FileList,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let button_handler = ButtonHandler::new();

        //let font = gfx.create_font(include_bytes!("assets/Ubuntu-B.ttf"));

        let theme = Theme::from_path("theme.toml");
        let file_list = FileList::new();

        let mut state = Self {
            time: 0.0,
            button_handler,
            theme,
            file_list
        };

        state.button_handler.add(
            State::test_state,
            Bounds::new((100.0,100.0),(100.0,50.0))
        );
        state.button_handler.add(
            |_| println!("AAAAAAAAAAA\nwriting cool little closure"),
            Bounds::new((100.0,300.0),(100.0,50.0))
        );

        state
    }

    fn process_buttons(&mut self, app: &mut App){
        let pos = app.mouse.position();
        let clicked = app.mouse.left_is_down();

        let func = self.button_handler.update(pos,clicked);
        (func)(self);
    }

    fn draw_buttons(&mut self, draw: &mut Draw)
    {
        self.button_handler.draw(draw,&self.theme);
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

    draw.clear(state.theme.colors["secondary"]);
    let size = gfx.size();

    state.file_list.draw(&mut draw, size, &state.theme);

    state.draw_buttons(&mut draw);

    gfx.render(&draw);
}
