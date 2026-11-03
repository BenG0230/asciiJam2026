use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use rustui::{Colour, Engine, Modifiers, Style};

use crate::maths::Vec2;

pub const MAP: [[u8; 20]; 20] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

pub struct Player {
    pub pos: Vec2,
    pub dir: Vec2,
}

pub fn raycast(p: &Player, e: &mut Engine, map: &Vec<Vec<u8>>) {
    let map_pos = Vec2::new(p.pos.x.floor(), p.pos.y.floor());
    let rem = p.pos - map_pos;

    let w = e.term.width;
    let fov = 90.0f32.to_radians();
    let plane = Vec2::new(p.dir.y, -p.dir.x);

    for x in 0..w {
        let camx = 2.0 * x as f32 / w as f32 - 1.0;
        let ray_dir = p.dir + plane * camx;

        let mut map_x = map_pos.x as i32;
        let mut map_y = map_pos.y as i32;

        let step_x = if ray_dir.x < 0.0 { -1 } else { 1 };
        let step_y = if ray_dir.y < 0.0 { -1 } else { 1 };

        let delta_dist_x = if ray_dir.x == 0.0 {
            f32::INFINITY
        } else {
            (1.0 / ray_dir.x).abs()
        };
        let delta_dist_y = if ray_dir.y == 0.0 {
            f32::INFINITY
        } else {
            (1.0 / ray_dir.y).abs()
        };

        let mut side_dist_x = if ray_dir.x < 0.0 {
            rem.x * delta_dist_x
        } else {
            (1.0 - rem.x) * delta_dist_x
        };

        let mut side_dist_y = if ray_dir.y < 0.0 {
            rem.y * delta_dist_y
        } else {
            (1.0 - rem.y) * delta_dist_y
        };

        let mut hit = false;
        let mut side = 0;

        while !hit {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }

            if map[map_y as usize][map_x as usize] > 0 {
                hit = true;
            }
        }

        let perp_wall_dist = if side == 0 {
            side_dist_x - delta_dist_x
        } else {
            side_dist_y - delta_dist_y
        };

        let line_height = (e.term.height as f32 / perp_wall_dist) as i32;

        let mut draw_start = -line_height / 2 + e.term.height as i32 / 2;
        if draw_start < 0 {
            draw_start = 0;
        }
        let mut draw_end = line_height / 2 + e.term.height as i32 / 2;
        if draw_end >= e.term.height as i32 {
            draw_end = e.term.height as i32 - 1;
        }

        let brightness = ((255.0 / (1.0 + perp_wall_dist / 3.0)).clamp(0.0, 255.0) / 1.0) as u8;
        let col = if side == 1 {
            Colour::Rgb(brightness + 10, brightness + 10, brightness + 10)
        } else {
            Colour::Rgb(brightness, brightness, brightness)
        };

        let style = Style {
            fg: col,
            bg: Colour::C16(30),
            modifiers: Modifiers(0),
        };

        let ceil_style = Style {
            fg: Colour::C16(30),
            bg: Colour::C16(30),
            modifiers: Modifiers(0),
        };

        let star_style = Style {
            fg: Colour::C16(37),
            bg: Colour::C16(30),
            modifiers: Modifiers(0),
        };

        let mut rng =
            StdRng::seed_from_u64((x + ((p.dir.x.cos() + p.dir.y.sin()) * 150.0) as u16) as u64);
        for y in 0..draw_start as usize {
            if rng.random_bool(0.02) {
                if rand::random_bool(0.5) {
                    e.put_char(x as usize, y, '.', star_style);
                } else {
                    e.put_char(x as usize, y, '.', ceil_style);
                }
            } else {
                e.put_char(x as usize, y, '.', ceil_style);
            }
        }

        e.draw_line(
            x as usize,
            draw_start as usize,
            x as usize,
            draw_end as usize,
            '#',
            style,
        );

        e.draw_line(
            x as usize,
            draw_end as usize + 1,
            x as usize,
            e.term.height as usize,
            '.',
            ceil_style,
        );
    }
}
