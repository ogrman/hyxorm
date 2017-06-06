extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

mod model;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Hyxorm the Bitener", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut tick = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    let world = model::World::new(16, 16);
    let mut segment = model::SnakeSegment::new(1, 5, model::Direction::Right);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    break 'running,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } =>
                    segment.turn(model::Direction::Left),
                Event::KeyDown { keycode: Some(Keycode::Right), .. } =>
                    segment.turn(model::Direction::Right),
                Event::KeyDown { keycode: Some(Keycode::Up), .. } =>
                    segment.turn(model::Direction::Up),
                Event::KeyDown { keycode: Some(Keycode::Down), .. } =>
                    segment.turn(model::Direction::Down),
                _ => {}
            }
        }

        if tick % 10 == 0 {
            segment.move_fwd();
        }

        {
            // Update the window title.
            let mut window = canvas.window_mut();

            let position = window.position();
            let size = window.size();
            let title = format!("Hyxorm the Bitener - pos({}x{}), size({}x{}): {}",
                                position.0,
                                position.1,
                                size.0,
                                size.1,
                                tick);
            window.set_title(&title).unwrap();

            tick += 1;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 0, 0));

        for y in 0..world.height {
            for x in 0..world.width {
                if world.get_cell(x, y) == 1 {
                    let x_pxl = x as i32 * 32;
                    let y_pxl = y as i32 * 32;
                    canvas.fill_rect(Rect::new(x_pxl, y_pxl, 32, 32)).ok();
                }
            }
        }

        canvas.set_draw_color(Color::RGB(0, 255, 0));

        let x_pxl = segment.x as i32 * 32;
        let y_pxl = segment.y as i32 * 32;
        canvas.fill_rect(Rect::new(x_pxl, y_pxl, 32, 32)).ok();

        canvas.present();
    }
}
