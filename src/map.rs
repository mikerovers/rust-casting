pub struct Map {
    pub width: usize,
    pub height: usize,
    pub map: Vec<char>
}

impl Map {
    pub fn new(width: usize, height: usize, map: Vec<char>) -> Map {
        Map{
            width,
            height,
            map
        }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width && y < self.height && self.map.len() == self.width * self.height);
        self.map[x + y * self.width] as usize - '0' as usize
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        assert!(x < self.width && y < self.height && self.map.len() == self.width * self.height);
        self.map[x + y * self.width] == ' '
    }
}