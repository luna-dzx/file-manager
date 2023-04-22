use std::fs;
use std::io::ErrorKind;
use notan::draw::*;
use notan::prelude::*;
use toml::de::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub highlight_focused: Color,
    pub highlight_unfocused: Color,

    pub font: Font,
}

#[derive(Serialize,Deserialize)]
pub struct RawTheme {
    pub primary_color: [u8;4],
    pub secondary_color: [u8;4],
    pub highlight_focused: [u8;4],
    pub highlight_unfocused: [u8;4],
}


impl Theme{
    pub fn defaults(gfx:&mut Graphics) -> Theme {

        let font = gfx.create_font(include_bytes!("assets/Ubuntu-B.ttf")).unwrap();

        Theme{
            primary_color:       Color::from(0x24232eff),
            secondary_color:     Color::from(0x34324aff),
            highlight_focused:   Color::YELLOW,
            highlight_unfocused: Color::BLACK,
            font
        }
    }

    fn from_raw(gfx:&mut Graphics, raw: RawTheme) -> Self {

        let font = gfx.create_font(include_bytes!("assets/Ubuntu-B.ttf")).unwrap();

        Theme{
            primary_color:       Color::from(raw.primary_color),
            secondary_color:     Color::from(raw.secondary_color),
            highlight_focused:   Color::from(raw.highlight_focused),
            highlight_unfocused: Color::from(raw.highlight_unfocused),
            font
        }
    }

    fn to_raw(&self) -> RawTheme {
        RawTheme {
            primary_color: self.primary_color.rgba_u8(),
            secondary_color: self.secondary_color.rgba_u8(),
            highlight_focused: self.highlight_focused.rgba_u8(),
            highlight_unfocused: self.highlight_unfocused.rgba_u8(),
        }
    }


    pub fn from_path(gfx:&mut Graphics, path:&str)->Theme{
        // attempt to load from file
        if let Ok(config_content) = fs::read_to_string(path) {
            if let Ok(raw) = toml::from_str::<RawTheme>(&config_content){
                return Theme::from_raw(gfx, raw);
            }
        }

        println!("default thingy");
        // load default
        let theme = Theme::defaults(gfx);
        theme.save_to_path(path);
        theme
    }

    pub fn save_to_path(&self, path:&str){
        let toml_string = toml::to_string_pretty(&self.to_raw()).unwrap();

        fs::File::create(path).unwrap();
        fs::write(path,toml_string).unwrap();
    }
}