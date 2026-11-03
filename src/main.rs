mod maths;
mod maze;
mod rendering;

use std::io;

use rustui::{Engine, KeyCode, Style};

use crate::{maths::Vec2, rendering::Player};

fn main() -> io::Result<()> {
    let map = maze::generate(15).into_map();

    let mut engine = Engine::new()?;
    let mut p = Player {
        pos: Vec2::new(1.3, 1.3),
        dir: Vec2::new(1.0, 0.0).normalise(),
    };

    let a_delta = 5.0f32.to_radians();
    let p_delta = 0.1;

    'main: loop {
        engine.clear_buffer();
        let events = engine.read_inputs()?;
        for event in events {
            match event.code {
                KeyCode::Escape => break 'main,
                KeyCode::Left => {
                    let cos = a_delta.cos();
                    let sin = a_delta.sin();
                    p.dir = Vec2::new(p.dir.x * cos - p.dir.y * sin, p.dir.x * sin + p.dir.y * cos)
                }
                KeyCode::Right => {
                    let cos = (-a_delta).cos();
                    let sin = (-a_delta).sin();
                    p.dir = Vec2::new(p.dir.x * cos - p.dir.y * sin, p.dir.x * sin + p.dir.y * cos)
                }
                KeyCode::Up => {
                    p.pos = p.pos + p.dir * p_delta;
                }
                _ => {}
            }
        }

        rendering::raycast(&p, &mut engine, &map);

        engine.draw()?;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}

fn display_map(map: &Vec<Vec<u8>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 1 {
                print!("\x1b[40m##");
            } else {
                print!("\x1b[107m  ");
            }
        }
        println!("\x1b[0m");
    }
}
