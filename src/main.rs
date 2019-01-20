#![feature(drain_filter)]
extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;

mod font;
mod model;

use font::fivebyfive;
use model::snake::Direction;
use model::snake::Position;
use model::snake::Snake;
use model::world::CellContent;
use model::world::World;

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

    let mut world = World::new(16, 16);
    let clone_this_snake = Snake::new(Position { x: 1, y: 5 }, Direction::Right, 5);
    let mut snake = clone_this_snake.clone();
    world.spawn_nugget(&snake, None);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => snake.turn(Direction::Left),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => snake.turn(Direction::Right),
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => snake.turn(Direction::Up),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => snake.turn(Direction::Down),
                _ => {}
            }
        }

        if tick % 10 == 0 {
            let np = snake.next_head_pos();

            match world.get_cell(&np) {
                CellContent::Nothing => {
                    if snake.is_here(&np) {
                        snake = clone_this_snake.clone();
                    } else {
                        snake.move_fwd()
                    }
                }
                CellContent::Nugget => {
                    world.consume_nugget(&np);
                    snake.grow();
                    world.spawn_nugget(&snake, Some(&np));
                    snake.move_fwd();
                    if world.score > 0 && world.score % 10 == 0 {
                        world.spawn_nugget(&snake, Some(&np));
                    }
                }
                CellContent::Wall => {
                    snake = clone_this_snake.clone();
                }
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let wall_color = Color::RGB(255, 0, 0);
        let nugget_color = Color::RGB(0, 0, 255);
        let score_color = Color::RGB(0, 0, 255);
        let snake_color = Color::RGB(0, 255, 0);

        let pixel_size_u32: u32 = 32;
        let pixel_size_usize: usize = pixel_size_u32 as usize;
        let pixel_size_i32: i32 = pixel_size_u32 as i32;

        for y in 0..world.height {
            for x in 0..world.width {
                let x_pxl = x as i32 * pixel_size_i32;
                let y_pxl = y as i32 * pixel_size_i32;
                match world.get_cell(&Position { x, y }) {
                    CellContent::Nugget => {
                        canvas.set_draw_color(nugget_color);
                        fill_square(&mut canvas, x_pxl, y_pxl, pixel_size_u32);
                    }
                    CellContent::Wall => {
                        canvas.set_draw_color(wall_color);
                        fill_square(&mut canvas, x_pxl, y_pxl, pixel_size_u32);
                    }
                    _ => (),
                }
            }
        }

        canvas.set_draw_color(snake_color);

        for segment in snake.segments.iter() {
            let x_pxl = segment.pos.x as i32 * pixel_size_i32;
            let y_pxl = segment.pos.y as i32 * pixel_size_i32;
            fill_square(&mut canvas, x_pxl, y_pxl, pixel_size_u32);
        }

        let start_x: u32 = ((world.width + 1) * pixel_size_usize) as u32;
        let start_y: u32 = pixel_size_u32;
        let score_digits = world.score.to_string();
        let font_pixel_size: u32 = 4;
        let digit_padding = 5;
        let digit_width = 20;
        canvas.set_draw_color(score_color);
        for (digit_count, cd) in score_digits.chars().enumerate().map(|(dc, x)| (dc as u32, x)) {
            let digit = cd.to_digit(10).unwrap();
            let char_vec = fivebyfive::from_digit(digit as usize);
            let digit_offset = digit_count * (digit_width + digit_padding);
            for (digit_pixel, x) in char_vec.iter().enumerate().map(|(dp, x)| (dp as u32, x)) {
                let digit_x = digit_pixel % 5;
                let digit_y = digit_pixel / 5;
                if *x == 1 {
                    fill_square(
                        &mut canvas,
                        (start_x + digit_offset + digit_x * font_pixel_size) as i32,
                        (start_y + digit_y * font_pixel_size) as i32,
                        font_pixel_size,
                    );
                }
            }
        }

        canvas.present();

        tick += 1;
    }
}

fn fill_square<RT: RenderTarget>(canvas: &mut Canvas<RT>, x: i32, y: i32, size: u32) {
    canvas
        .fill_rect(Rect::new(
            x,
            y,
            size,
            size,
        )).ok();
}
