use std::fs::File;
use std::error::Error;
use std::io::Write;

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
            assert!(cx < image_width && cy < image_height);
            image[cx + cy * image_width] = color;
        }
    }
}

fn main() {
    let window_width: usize = 512;
    let window_height: usize = 512;
    let map_width: usize = 16;
    let map_height: usize = 16;

    let map: Vec<char> = "00002222222200001              01      11111   01     0        00     0  11100000     3        00   10000      00   0   11100  00   0   0      00   0   1  000000       1      02       1      00       0      00 0000000      00              00002222222200000".chars().collect();

    assert_eq!(map.len(), map_width * map_height);
    let player_x: f32 = 3.456;
    let player_y: f32 = 2.345;

    let mut framebuffer: Vec<u32> = Vec::with_capacity(window_width * window_height);

    framebuffer.resize(window_height * window_width, 255);
    println!("Framebuffer is {} values long", framebuffer.len());

    for y in 0..window_height {
        for x in 0..window_width {
            let r: u8 = (255 * y / window_height) as u8;
            let g: u8 = (255 * x / window_width) as u8;
            let b: u8 = 0;
            framebuffer[x + y * window_width] = pack_color(r, g, b, 255);
        }
    }

    let rect_w: usize = window_width / map_width;
    let rect_h: usize = window_height / map_height;
    for y in 0..map_height {
        for x in 0..map_width {
            if map[x + y * map_width] == ' ' {
                continue;
            }

            let rect_x = x * rect_w;
            let rect_y = y * rect_h;
            draw_rectangle(&mut framebuffer, window_width, window_height, rect_x, rect_y, rect_w, rect_h, pack_color(0, 255, 255, 255));
        }
    }

    draw_rectangle(&mut framebuffer, window_width, window_height, (player_x * rect_w as f32) as usize, (player_y * rect_h as f32) as usize, 5, 5, pack_color(255, 255, 255, 255));

    drop_ppm_image(&String::from("./output.ppm"), &framebuffer, window_width, window_height);
    println!("Finished!");
}
