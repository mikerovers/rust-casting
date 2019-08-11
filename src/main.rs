extern crate image;
extern crate rand;

use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::f32::consts::PI;
use std::path::Path;
use image::GenericImageView;
use rand::prelude::*;

fn pack_color(r: u8, g: u8, b: u8, a: u8) -> u32 {
    return (((a as u32) << 24) + ((b as u32) << 16) + ((g as u32) << 8) + r as u32) as u32;
}

fn unpack_color(color: &u32) -> (u8, u8, u8, u8) {
    (((*color >> 0) & (0xff as u32)) as u8, ((*color >> 8) & (0xff as u32)) as u8, ((*color >> 16) & (0xff as u32)) as u8, ((*color >> 24) & (0xff as u32)) as u8)
}

fn drop_ppm_image(file_name: &String, image: &Vec<u32>, w: usize, h: usize) {
    assert_eq!(image.len(), w * h);
    let mut file = match File::create(file_name) {
        Err(why) => panic!("Couldn't create {}: {}", file_name, why.description()),
        Ok(file) => file,
    };

    file.write(format!("P6\n{} {} \n255\n", w, h).as_bytes());
    for i in 0..h*w {
        let unpacked_color = unpack_color(&image[i]);
        file.write(&[unpacked_color.0]);
        file.write(&[unpacked_color.1]);
        file.write(&[unpacked_color.2]);
    }
}

fn draw_rectangle(image: &mut Vec<u32>, image_width: usize, image_height: usize, x_cor: usize, y_cor: usize, w: usize, h: usize, color: u32) {
    assert_eq!(image.len(), image_width * image_height);
    for x in 0..w {
        for y in 0..h {
            let cx = x_cor + x;
            let cy = y_cor + y;
            if cx >= image_width || cy >= image_height {
                continue;
            }
            image[cx + cy * image_width] = color;
        }
    }
}

fn load_texture(file_name: &String) -> (Vec<u32>, u32, u32) {
    let number_of_channels = -1;

    let image = image::open(Path::new(file_name)).unwrap();

    let width = image.width();
    let height = image.height();
    let texture_cnt = width / height;
    let texture_size = width / texture_cnt;

    if width != height * texture_cnt {
        panic!("Texture file must contain N square texture packed horizontally.")
    }

    let mut texture: Vec<u32> = Vec::with_capacity((width * height) as usize);
    texture.resize(height as usize * width as usize, pack_color(255, 255, 255, 255));
    for y in 0..height {
        for x in 0..width {
            let r = image.get_pixel(x, y).0[0] as u8;
            let g = image.get_pixel(x, y).0[1] as u8;
            let b = image.get_pixel(x, y).0[2] as u8;
            let a = image.get_pixel(x, y).0[3] as u8;
            texture [x as usize + y as usize * width as usize] = pack_color(r, g, b, a);
        }
    }

    (texture, texture_size, texture_cnt)
}

fn texture_column(image: &Vec<u32>, texture_size: usize, number_of_textures: usize, texture_id: usize, texture_coordinate: usize, column_height: usize) -> Vec<u32> {
    let image_width: usize = texture_size * number_of_textures;
    let image_height = texture_size;
    assert!(image.len() == image_width * image_height && texture_coordinate < texture_size && texture_id < number_of_textures);
    let mut column: Vec<u32> = Vec::with_capacity(column_height);
    column.resize(column_height, 0);
    for y in 0..column_height {
        let pix_x: usize = texture_id * texture_size + texture_coordinate;
        let pix_y: usize = (y * texture_size) / column_height;
        column[y] = image[pix_x + pix_y * image_width];
    }

    column
}

fn main() {
    let window_width: usize = 1024;
    let window_height: usize = 512;
    let map_width: usize = 16;
    let map_height: usize = 16;

    let map: Vec<char> = "00002222222200001              01      11111   01     0        00     0  11100000     3        00   10000      00   3   11100  05   4   0      05   4   1  000000       1      02       1      00       0      00 0000000      00              00002222222200000".chars().collect();

    let mut rng = rand::thread_rng();
    let number_of_colors: usize = 10;
    let mut colors: Vec<u32> = Vec::with_capacity(number_of_colors);
    colors.resize(number_of_colors, 0);
    for i in 0..number_of_colors {
        colors[i] = pack_color(rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255));
    }

    assert_eq!(map.len(), map_width * map_height);
    let mut player_x: f32 = 3.456;
    let mut player_y: f32 = 2.345;
    let mut player_a: f32 = 1.523;
    let mut fov: f32 = PI / 3.0;

    let mut framebuffer: Vec<u32> = Vec::with_capacity(window_width * window_height);

    framebuffer.resize(window_height * window_width, pack_color(255, 255, 255, 255));
    println!("Framebuffer is {} values long", framebuffer.len());

    let (wall_texture, texture_size, texture_count) = load_texture(&String::from("walltext.png"));

    let rect_w: usize = window_width / (map_width * 2);
    let rect_h: usize = window_height / map_height;
    for y in 0..map_height {
        for x in 0..map_width {
            if map[x + y * map_width] == ' ' {
                continue;
            }

            let rect_x = x * rect_w;
            let rect_y = y * rect_h;
            let texture_id = map[x + y * map_width] as usize - '0' as usize;
            assert!((texture_id as u32) < (texture_count as u32));

            draw_rectangle(&mut framebuffer, window_width, window_height, rect_x, rect_y, rect_w, rect_h, wall_texture[texture_id * texture_size as usize]);
        }
    }

    for w in 0..window_width / 2 {
        let angle: f32 = player_a - fov / 2.0 + fov * (w as f32) / (window_width as f32 / 2.0);

        let mut t: f32 = 0.0;
        while t <= 20.0 {
            t += 0.01;
            let cx: f32 = player_x + t * angle.cos();
            let cy: f32 = player_y + t * angle.sin();

            let mut pix_x = (cx * rect_w as f32) as i32;
            let mut pix_y = (cy * rect_h as f32) as i32;
            framebuffer[(pix_x + pix_y * window_width as i32) as usize] = pack_color(160, 160, 160, 255);

            if map[cx as usize + cy as usize * map_width] != ' ' {
                let texture_id = map[cx as usize + cy as usize * map_width] as usize - '0' as usize;
                assert!(texture_id < texture_count as usize);
                let column_height: usize = (window_height as f32 / (t * (angle - player_a).cos())) as usize;

                let hit_x: f32 = cx - (cx + 0.5).floor();
                let hit_y: f32 = cy - (cy + 0.5).floor();

                let mut x_texture_coordinate: i32 = (hit_x * texture_size as f32) as i32;
                if hit_y.abs() > hit_x.abs() {
                    x_texture_coordinate = (hit_y * texture_size as f32) as i32;
                }

                if x_texture_coordinate < 0 {
                    x_texture_coordinate += texture_size as i32;
                }

                assert!(x_texture_coordinate >= 0 && x_texture_coordinate < (texture_size as i32));

                let column: Vec<u32> = texture_column(&wall_texture, texture_size as usize, texture_count as usize, texture_id, x_texture_coordinate as usize, column_height);
                pix_x = (window_width / 2 + w) as i32;
                for j in 0..column_height {
                    pix_y = (j + window_height / 2 - column_height / 2) as i32;
                    if pix_y < 0 || pix_y > window_height as i32 {
                        continue;
                    }

                    framebuffer[(pix_x + pix_y * window_width as i32) as usize] = column[j];
                }

//                draw_rectangle(&mut framebuffer, window_width, window_height, ((window_width as f32) / 2.0 + (w as f32)) as usize, ((window_height as f32) / 2.0 - (column_height as f32) / 2.0) as usize, 1, column_height, wall_texture[texture_id * texture_size as usize]);
                break;
            }
        }
    }

    drop_ppm_image(&String::from("./output.ppm"), &framebuffer, window_width, window_height);
    println!("Finished!");
}
