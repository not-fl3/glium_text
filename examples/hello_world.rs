extern crate glium;
extern crate glium_text_rusttype as glium_text;
extern crate cgmath;

use glium::Surface;
use glium::glutin;

use glium_text::FontTexture;
use glium_text::TextDisplay;
use glium_text::TextSystem;

use std::thread;
use std::time::Duration;

fn main() {
    use glium::DisplayBuild;

    let display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .build_glium()
        .unwrap();
    let system = TextSystem::new(&display);

    let font = FontTexture::new(
        &display,
        &include_bytes!("font.ttf")[..],
        32,
        FontTexture::ascii_character_list()
    ).unwrap();

    let text = TextDisplay::new(&system, &font, "Hello, world!");
    let text_width = text.get_width();
    println!("Text width: {:?}", text_width);

    let sleep_duration = Duration::from_millis(17);

    'main: loop {
        let (w, h) = display.get_framebuffer_dimensions();

        let matrix = cgmath::Matrix4::new(
            2.0 / text_width, 0.0, 0.0, 0.0,
            0.0, 2.0 * (w as f32) / (h as f32) / text_width, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -1.0, -1.0, 0.0, 1.0f32,
        );

        let color = (1.0, 1.0, 0.0, 1.0);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        glium_text::draw(&text, &system, &mut target, matrix, color)
            .unwrap();
        target.finish().unwrap();

        thread::sleep(sleep_duration);

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }
    }
}
