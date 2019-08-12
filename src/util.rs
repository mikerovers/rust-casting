use crate::framebuffer::Framebuffer;
use std::fs::File;
use std::io::Write;
use std::error::Error;

pub fn pack_color(r: u8, g: u8, b: u8, a: u8) -> u32 {
    return (((a as u32) << 24) + ((b as u32) << 16) + ((g as u32) << 8) + r as u32) as u32;
}

pub fn unpack_color(color: &u32) -> (u8, u8, u8, u8) {
    (((*color >> 0) & (0xff as u32)) as u8, ((*color >> 8) & (0xff as u32)) as u8, ((*color >> 16) & (0xff as u32)) as u8, ((*color >> 24) & (0xff as u32)) as u8)
}

pub fn drop_ppm_image(file_name: &String, image: &Framebuffer, w: usize, h: usize) {
    assert_eq!(image.length(), w * h);
    let mut file = match File::create(file_name) {
        Err(why) => panic!("Couldn't create {}: {}", file_name, why.description()),
        Ok(file) => file,
    };

    file.write(format!("P6\n{} {} \n255\n", w, h).as_bytes());
    for i in 0..h*w {
        let unpacked_color = unpack_color(&image.get_pixel(i));
        file.write(&[unpacked_color.0]);
        file.write(&[unpacked_color.1]);
        file.write(&[unpacked_color.2]);
    }
}