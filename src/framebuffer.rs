pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub image: Vec<u32>
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        Framebuffer {width, height, image: Vec::with_capacity(width * height)}
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        assert!(self.image.len() == self.width * self.height && x < self.width && y < self.height);
        self.image[x + y * self.width] = color;
    }

    pub fn get_pixel(&self, i: usize) -> u32 {
        self.image[i]
    }

    pub fn draw_rectangle(&mut self, rect_x: usize, rect_y: usize, rect_w: usize, rect_h: usize, color: u32) {
        assert_eq!(self.image.len(), self.height * self.width);
        for x in 0..rect_w {
            for y in 0..rect_h {
                let cx: usize = rect_x + x;
                let cy: usize = rect_y + y;
                if cx < self.width && cy < self.height {
                    self.set_pixel(cx, cy, color);
                }
            }
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.image.resize(self.width * self.height, color);
    }

    pub fn length(&self) -> usize {
        self.image.len()
    }
}