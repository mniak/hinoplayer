extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::printf;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let sdl_ttf = ttf::init().map_err(|e| e.to_string())?;

    let video = sdl.video()?;
    let display_mode = video.current_display_mode(0)?;
    let font = sdl_ttf.load_font("./assets/din-condensed-bold.ttf", 96)?;

    let window = video
        .window(
            "HinoPlayer: Video",
            (display_mode.w as f32 * 0.8) as u32,
            (display_mode.h as f32 * 0.8) as u32,
        )
        // .fullscreen()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(10, 10, 10));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.fill_rect(Rect::new(1, 1, 100, 100))?;

    let lines = ["First line", "And the other line"];
    let (canvas_width, canvas_height) = canvas.output_size()?;
    let total_text_height = (font.recommended_line_spacing() * lines.len() as i32) as f32;
    let offset = font.recommended_line_spacing() as f32 * 4.0 / 7.0;

    let rendered_lines: Result<Vec<(Texture<'_>, Rect, Rect)>, String> = lines
        .iter()
        .enumerate()
        .map(
            |(index, line)| -> Result<(Texture<'_>, Rect, Rect), String> {
                let text_surface = font
                    .render(line)
                    .blended(Color::RGBA(50, 50, 100, 255))
                    .map_err(|e| e.to_string())?;
                let text_rect = text_surface.rect();
                let text_texture = texture_creator
                    .create_texture_from_surface(text_surface)
                    .map_err(|e| e.to_string())?;

                let center_x = canvas_width as f32 / 2.0;
                let center_y = canvas_height as f32 / 2.0;
                let line_center_y = center_y - text_rect.h as f32 / 2.0
                    + (font.recommended_line_spacing() * index as i32) as f32
                    - total_text_height / 2.0
                    + offset;

                let target_rect = Rect::new(
                    (center_x - (text_rect.width() as f32) / 2.0) as i32,
                    line_center_y as i32,
                    text_rect.width(),
                    text_rect.height(),
                );

                Ok((text_texture, text_rect, target_rect))
            },
        )
        .collect();
    
    for line in rendered_lines? {
        let (text_texture, text_rect, target_rect) = line;
        canvas
            .copy(&text_texture, text_rect, target_rect)
            .map_err(|e| e.to_string())?;
    }
    

    canvas.present();

    let mut events = sdl.event_pump()?;

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // canvas.clear();
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
