use crate::simple_grid_iterator::SimpleGridIterator;

pub fn grid_iterator(
    width: usize,
    height: usize,
    jump_x: usize,
    jump_y: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let num_frames = jump_x * jump_y;
    let (frame_width, frame_height) = (width / jump_x, height / jump_y);

    std::iter::repeat(SimpleGridIterator::new(frame_width, frame_height))
        .take(num_frames)
        .enumerate()
        .map(|(i, iter)| std::iter::repeat(i).zip(iter))
        .flatten()
        .map(move |(frame, (x, y))| {
            let (offset_x, offset_y) = (frame % jump_x, frame / jump_x);
            (offset_x + x * jump_x, offset_y + y * jump_y)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_one_small() {
        let mut iter = grid_iterator(2, 2, 1, 1);

        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn jump_two_small() {
        let mut iter = grid_iterator(2, 2, 2, 2);

        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn jump_two_big_grid() {
        let mut iter = grid_iterator(4, 3, 2, 1);

        // Frame 0
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((2, 0)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((2, 1)));
        assert_eq!(iter.next(), Some((0, 2)));
        assert_eq!(iter.next(), Some((2, 2)));

        // Frame 1
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next(), Some((3, 0)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), Some((3, 1)));
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.next(), Some((3, 2)));

        assert_eq!(iter.next(), None);
    }
}
