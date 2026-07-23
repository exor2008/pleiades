use ledlab::buffer::Buffer;
use smart_leds::RGB8;

pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct RGB8Buffer<const L: usize, const N: usize> {
    pub data: [RGB8; N],
}

impl<const L: usize, const N: usize> Default for RGB8Buffer<L, N> {
    fn default() -> Self {
        Self {
            data: [RGB8::default(); N],
        }
    }
}

impl<const L: usize, const N: usize> RGB8Buffer<L, N> {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        match x.is_multiple_of(2) {
            true => x * L + y,
            false => x * L + (L - y) - 1,
        }
    }
}

impl<const L: usize, const N: usize> Buffer<RGB8, Point> for RGB8Buffer<L, N> {
    fn write(&mut self, coord: Point, color: RGB8) {
        let index = self.index(coord.x, coord.y);
        self.data[index] = color;
    }

    fn write_straight(&mut self, index: usize, color: RGB8) {
        self.data[index] = color;
    }

    fn clear(&mut self) {
        self.data = [RGB8::default(); N];
    }

    fn bg(&mut self, bg: RGB8) {
        self.data = [bg; N];
    }

    fn read(&self, coord: Point) -> RGB8 {
        let index = self.index(coord.x, coord.y);
        self.data[index]
    }
}
