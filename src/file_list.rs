use super::theming::*;
use notan::prelude::*;
use notan::draw::*;

pub struct FileList {
    padding: f32,
    radius: f32,
}

impl FileList {
    pub fn new() -> Self {
        Self {padding: 2.0, radius: 10.0}
    }


    pub fn draw(&self, draw:&mut Draw, size: (i32,i32), theme: &Theme) {

        let top_bar: f32 = 50.0 / (size.1 as f32);

        let mut draw_rect = |min: (f32, f32), max: (f32, f32)| {

            let mut pos = (self.padding + min.0 * size.0 as f32, self.padding + min.1 * size.1 as f32);
            let mut rect_size = (- pos.0 - self.padding + max.0 * size.0 as f32, - pos.1 - self.padding + max.1 * size.1 as f32);
            if min.0 <= 0.0 { pos = (pos.0 + 1.0, pos.1); }
            if min.1 <= 0.0 { pos = (pos.0, pos.1 + 1.0); }
            if max.0 >= 1.0 { rect_size = (rect_size.0 - 2.0, rect_size.1); }
            if max.1 >= 1.0 { rect_size = (rect_size.0, rect_size.1 - 2.0); }


            draw.rect(pos, rect_size)
                .corner_radius(10.0)
                .color(theme.colors["primary"]);

        };

        draw_rect((0.0,0.0),(1.0, top_bar));

        if size.0 > 700
        {
            draw_rect((0.0,top_bar),(0.2, 1.0));
            draw_rect((0.2,top_bar),(0.5, 1.0));
            draw_rect((0.5,top_bar),(1.0, 1.0));
        }
        else{
            draw_rect((0.0,top_bar),(0.3, 1.0));
            draw_rect((0.3,top_bar),(1.0, 1.0));
        }

        let paths = std::fs::read_dir("./").unwrap();

        let mut y = top_bar+10.0;

        for path in paths {
            let path = path.unwrap().path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let color: Color;
            color = if path.is_dir() {
                Color::WHITE
            }
            else{
                theme.colors["secondary"]
            };

            /*TRIES TO GET FONT FROM THEME
            BANISHED EXILED AND SCORNED FOR NOW
            draw.text(&theme.font,file_name).color(color)
                .position(5.0,y);*/

            y+=20.0;
        }


    }
}