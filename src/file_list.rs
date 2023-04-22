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


        if size.0 > 700
        {
            draw.rect((self.padding + 1.0,self.padding + 1.0),(-2.0*self.padding - 1.0 + size.0 as f32 * 0.2, size.1 as f32 - self.padding * 2.0 - 1.0))
                .corner_radius(10.0)
                .color(theme.primary_color);
            draw.rect((self.padding + size.0 as f32 * 0.2,self.padding + 1.0),(-2.0*self.padding + size.0 as f32 * 0.3, size.1 as f32 - self.padding * 2.0 - 1.0))
                .corner_radius(10.0)
                .color(theme.primary_color);
            draw.rect((self.padding + size.0 as f32 * 0.5,self.padding + 1.0),(-2.0*self.padding - 1.0 + size.0 as f32 * 0.5, size.1 as f32 - self.padding * 2.0 - 1.0))
                .corner_radius(10.0)
                .color(theme.primary_color);
        }
        else{
            draw.rect((self.padding,self.padding + 1.0),(-2.0*self.padding + size.0 as f32 * 0.3, size.1 as f32 - self.padding * 2.0 - 1.0))
                .corner_radius(10.0)
                .color(theme.primary_color);
            draw.rect((self.padding + size.0 as f32 * 0.3,self.padding + 1.0),(-2.0*self.padding - 1.0 + size.0 as f32 * 0.7, size.1 as f32 - self.padding * 2.0 - 1.0))
                .corner_radius(10.0)
                .color(theme.primary_color);
        }


        draw.text(&theme.font,"poopoo");


    }
}