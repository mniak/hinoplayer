extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
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
            display_mode.w as u32,
            display_mode.h as u32,
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
    for (index, line) in lines.iter().enumerate() {
        let text_surface = font
            .render(line)
            .blended(Color::RGBA(50, 50, 100, 255))
            .map_err(|e| e.to_string())?;
        let text_rect = text_surface.rect();
        let text_texture = texture_creator
            .create_texture_from_surface(text_surface)
            .map_err(|e| e.to_string())?;

        let center_x = canvas_width / 2;
        let line_center_y = (canvas_height / 2) as f32 + (2.0*(index as f32)-(lines.len() as f32))  * ((font.recommended_line_spacing() as f32) as f32)/2.0 + -0 as f32;


        let target_rect = Rect::new(
            (center_x as i32) - ((text_rect.width() as f32 / 2.0) as i32),
            line_center_y as i32,
            text_rect.width(),
            text_rect.height(),
        );

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
