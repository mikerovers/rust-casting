extern crate image;
extern crate rand;

use framebuffer::Framebuffer;
use player::Player;
use map::Map;
use texture::Texture;

mod texture;
mod framebuffer;
mod player;
mod map;
mod util;

use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::f32::consts::PI;
use std::path::Path;
use image::{GenericImageView, Frame};
use rand::prelude::*;
use crate::util::{pack_color, drop_ppm_image};

fn wall_x_texture_coordinate(x: f32, y: f32, texture_walls: &mut Texture) -> i32 {
    let hit_x: f32 = x - (x + 0.5).floor();
    let hit_y: f32 = y - (y + 0.5).floor();

    let mut tex: i32 = (hit_x * texture_walls.size as f32) as i32;
    if hit_y.abs() > hit_x.abs() {
        tex = (hit_y * texture_walls.size as f32) as i32;
    }

    if tex < 0 {
        tex += texture_walls.size as i32;
    }

    assert!(tex >= 0 && tex < (texture_walls.size as i32));

    tex
}

fn render(frame_buffer: &mut Framebuffer, map: &mut Map, player: &Player, texture_walls: &mut Texture) {
    frame_buffer.clear(pack_color(255, 255, 255, 255));
    let rect_w = frame_buffer.width / (map.width * 2);
    let rect_h = frame_buffer.height / map.height;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.is_empty(x, y) {
                continue;
            }

            let rect_x = x * rect_w;
            let rect_y = y * rect_h;
            let texture_id = map.get(x, y);
            assert!(texture_id < texture_walls.count);
            frame_buffer.draw_rectangle(rect_x, rect_y, rect_w, rect_h, *texture_walls.get(0, 0, texture_id));
        }
    }

    for w in 0..frame_buffer.width / 2 {
        let angle: f32 = player.angle - player.fov / 2.0 + player.fov * (w as f32) / (frame_buffer.width as f32 / 2.0);

        let mut t: f32 = 0.0;
        while t <= 20.0 {
            t += 0.01;
            let x: f32 = player.x_position + t * angle.cos();
            let y: f32 = player.y_position + t * angle.sin();
            frame_buffer.set_pixel((x * rect_w as f32) as usize, (y * rect_h as f32) as usize, pack_color(160, 160, 160, 255));

            if map.is_empty(x as usize, y as usize) {
                continue;
            }

            let texture_id = map.get(x as usize, y as usize);
            assert!(texture_id < texture_walls.count);
            let column_height: usize = (frame_buffer.height as f32 / (t * (angle - player.angle).cos())) as usize;
            let x_texcoord = wall_x_texture_coordinate(x, y, texture_walls);
            let column: Vec<u32> = texture_walls.get_scaled_column(texture_id, x_texcoord as usize, column_height);
            let pix_x: i32 = (w + frame_buffer.width / 2) as i32;

            for j in 0..column_height {
                let pix_y: i32 = (j + frame_buffer.height / 2 - column_height / 2) as i32;
                if pix_y >= 0 && pix_y < (frame_buffer.height as i32) {
                    frame_buffer.set_pixel(pix_x as usize, pix_y as usize, column[j]);
                }
            }
            break;
        }
    }
}

fn main() {
    let mut frame_buffer = Framebuffer::new(1024, 512);

    let mut player = Player::new(3.456, 2.345, 1.523, PI / 3.0);
    let mut map = Map::new(16, 16, "00002222222200001              01      11111   01     0        00     0  11100000     3        00   10000      00   3   11100  05   4   0      05   4   1  000000       1      02       1      00       0      00 0000000      00              00002222222200000".chars().collect());

    let mut texture_walls = Texture::new(&String::from("walltext.png"));
    if !texture_walls.count == 0 {
        panic!("Failed to load wall texture");
    }

    render(&mut frame_buffer, &mut map, &mut player, &mut texture_walls);
    drop_ppm_image(&String::from("output.ppm"), &frame_buffer, frame_buffer.width, frame_buffer.height);

    println!("Finished!");
}
