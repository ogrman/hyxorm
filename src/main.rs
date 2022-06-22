extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

mod model;

use model::world::World;
use model::world::CellContent;
use model::snake::Snake;
use model::snake::Direction;

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
    let mut score = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut world = World::new(16, 16);
    world.spawn_nugget();
    let mut snake = Snake::new(1, 5, Direction::Right, 5);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Left => snake.turn(Direction::Left),
                        Keycode::Right => snake.turn(Direction::Right),
                        Keycode::Up => snake.turn(Direction::Up),
                        Keycode::Down => snake.turn(Direction::Down),
                        _ => {},
                    }
                }
                _ => {},
            }
        }

        if tick % 10 == 0 {
            let np = snake.next_head_pos();

            match world.get_cell(np.x, np.y) {
                CellContent::Nothing => {
                    if snake.contains(&np) {
                        break 'running;
                    } else {
                        if world.has_nugget(np.x, np.y) {
                            world.consume_nugget(&np);
                            score += 1;
                            println!("Score: {score}");
                            snake.grow();
                            world.spawn_nugget();
                        }
                        snake.move_fwd();
                    }
                },
                CellContent::Wall => break 'running,
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let wall_color = Color::RGB(255, 0, 0);
        let nugget_color = Color::RGB(0, 0, 255);

        for y in 0..world.height {
            for x in 0..world.width {
                let x_pxl = x as i32 * 32;
                let y_pxl = y as i32 * 32;
                match world.get_cell(x, y) {
                    CellContent::Wall => {
                        canvas.set_draw_color(wall_color);
                        canvas.fill_rect(Rect::new(x_pxl, y_pxl, 32, 32)).ok();
                    },
                    _ => {
                        if world.has_nugget(x, y) {
                            canvas.set_draw_color(nugget_color);
                            canvas.fill_rect(Rect::new(x_pxl, y_pxl, 32, 32)).ok();
                        }
                    }
                }
            }
        }

        canvas.set_draw_color(Color::RGB(0, 255, 0));

        for segment in snake.segments.iter() {
            let x_pxl = segment.x as i32 * 32;
            let y_pxl = segment.y as i32 * 32;
            canvas.fill_rect(Rect::new(x_pxl, y_pxl, 32, 32)).ok();
        }

        canvas.present();

        tick += 1;
    }
}
