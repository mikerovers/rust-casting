use std::path::Path;
use crate::util::pack_color;
use image::GenericImageView;

pub struct Texture {
    pub image_width: usize,
    pub image_height: usize,
    pub count: usize,
    pub size: usize,
    image: Vec<u32>
}

impl Texture {
    pub fn new(file_name: &String) -> Texture {
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

        Texture {
            image_width: width as usize,
            image_height: height as usize,
            count: texture_cnt as usize,
            size: texture_size as usize,
            image: texture
        }
    }

    pub fn get(&self, x: usize, y: usize, idx: usize) -> &u32 {
        assert!(x < self.size && y < self.size && idx < self.count);
        &self.image[x + idx * self.size + y * self.image_width]
    }

    pub fn get_scaled_column(&self, texture_id: usize, texture_coordinate: usize, column_height: usize) -> Vec<u32> {
        assert!(texture_coordinate < self.size && texture_id < self.count);
        let mut column: Vec<u32> = Vec::with_capacity(column_height);
        column.resize(column_height, 0);
        for y in 0..column_height {
            column[y] = *self.get(texture_coordinate, (y * self.size) / column_height, texture_id);
        }

        column
    }
}