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

fn main() {
    let window_width: usize = 512;
    let window_height: usize = 512;

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

    drop_ppm_image(&String::from("./output.ppm"), &framebuffer, window_width, window_height);
    println!("Finished!");
}
