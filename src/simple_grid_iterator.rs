use std::ops::Range;

pub struct SimpleGridIterator {
    cells: Range<usize>,
    width: usize,
    height: usize,
}

impl SimpleGridIterator {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: 0..(width * height),
            width,
            height,
        }
    }
}

impl Iterator for SimpleGridIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.cells.next().map(|i| (i % self.width, i / self.width))
    }
}

impl Clone for SimpleGridIterator {
    fn clone(&self) -> Self {
        Self::new(self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut iter = SimpleGridIterator::new(3, 2);

        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next(), Some((2, 0)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), Some((2, 1)));
        assert_eq!(iter.next(), None);
    }
}
