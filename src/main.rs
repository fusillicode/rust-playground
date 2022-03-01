fn main() {
    println!("Hello, world!");
}

fn capped(overflowing_value: i32, range: &std::ops::Range<i32>) -> (i32, i32) {
    let max = range.clone().max().unwrap_or(0) + 1;
    let (mut cycling_range, positive_overflowing_value): (
        Box<dyn std::iter::Iterator<Item = i32>>,
        i32,
    ) = if overflowing_value < 0 {
        (
            Box::new(range.clone().rev().cycle()),
            (overflowing_value + 1).abs(),
        )
    } else {
        (Box::new(range.clone().cycle()), overflowing_value.abs())
    };
    let capped_value =
        (0..=positive_overflowing_value).fold(0, |_, _| cycling_range.next().unwrap_or(0));
    (capped_value, positive_overflowing_value / max)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ciccio() {
        assert_eq!((59, 0), capped(59, &(0..60)));
        assert_eq!((0, 0), capped(0, &(0..60)));
        assert_eq!((1, 1), capped(61, &(0..60)));
        assert_eq!((0, 2), capped(120, &(0..60)));
        assert_eq!((10, 2), capped(130, &(0..60)));

        assert_eq!((0, 0), capped(0, &(0..24)));
        assert_eq!((1, 1), capped(25, &(0..24)));
        assert_eq!((18, 1), capped(42, &(0..24)));
        assert_eq!((4, 3), capped(76, &(0..24)));

        assert_eq!((21, 0), capped(-3, &(0..24)));
        assert_eq!((0, 0), capped(-24, &(0..24)));
        assert_eq!((23, 1), capped(-25, &(0..24)));

        assert_eq!((0, 59), capped(59, &(0..0)));
        assert_eq!((0, 59), capped(59, &(1..1)));
    }
}
