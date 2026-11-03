mod maths;
mod maze;
mod rendering;

use std::{io, time::Instant};

use rustui::{Engine, KeyCode, Style};

use crate::{maths::Vec2, rendering::Player};

fn main() -> io::Result<()> {
    let map = maze::generate(15).into_map();

    display_map(&map);
    std::thread::sleep(std::time::Duration::from_secs(1));

    let mut engine = Engine::new()?;
    let mut p = Player {
        pos: Vec2::new(1.3, 1.3),
        dir: Vec2::new(1.0, 0.0).normalise(),
    };

    let a_delta = 2.0f32.to_radians();
    let p_delta = 0.2;

    let mut m_pos: (i8, i8) = (0, 0);
    let mut last = Instant::now();

    'main: loop {
        engine.clear_buffer();
        let events = engine.read_inputs()?;
        for event in events {
            match event.code {
                KeyCode::Escape => break 'main,
                KeyCode::Char('a') => {
                    let cos = (a_delta * 2.0).cos();
                    let sin = (a_delta * 2.0).sin();
                    p.dir = Vec2::new(p.dir.x * cos - p.dir.y * sin, p.dir.x * sin + p.dir.y * cos)
                }
                KeyCode::Char('d') => {
                    let cos = (-a_delta * 2.0).cos();
                    let sin = (-a_delta * 2.0).sin();
                    p.dir = Vec2::new(p.dir.x * cos - p.dir.y * sin, p.dir.x * sin + p.dir.y * cos)
                }
                KeyCode::Char('w') => {
                    p.pos = p.pos + p.dir * p_delta;
                }
                KeyCode::Char('s') => {
                    p.pos = p.pos - p.dir * p_delta;
                }
                KeyCode::MouseMove(x, y) => {
                    if (x, y) != m_pos {
                        let dx = m_pos.0 - x;
                        m_pos = (x, y);

                        if dx > 0 {
                            let cos = a_delta.cos();
                            let sin = a_delta.sin();
                            p.dir = Vec2::new(
                                p.dir.x * cos - p.dir.y * sin,
                                p.dir.x * sin + p.dir.y * cos,
                            )
                        } else {
                            let cos = (-a_delta).cos();
                            let sin = (-a_delta).sin();
                            p.dir = Vec2::new(
                                p.dir.x * cos - p.dir.y * sin,
                                p.dir.x * sin + p.dir.y * cos,
                            )
                        }
                    }
                }
                _ => {}
            }
        }

        rendering::raycast(&p, &mut engine, &map);
        engine.put_str(
            0,
            0,
            &format!("dt:{}ms", last.elapsed().as_millis()),
            Style::default(),
        );
        last = Instant::now();

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
            } else if map[y][x] == 2 {
                print!("\x1b[42m##");
            } else {
                print!("\x1b[107m  ");
            }
        }
        println!("\x1b[0m");
    }
}
