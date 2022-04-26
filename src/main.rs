fn main() {
    println!("Hello, world!");
}

// fn capped(overflowing_value: i32, range: &std::ops::Range<i32>) -> (i32, i32) {
//     let max = range.clone().max().unwrap_or(0) + 1;
//     let (mut cycling_range, positive_overflowing_value): (
//         Box<dyn std::iter::Iterator<Item = i32>>,
//         i32,
//     ) = if overflowing_value < 0 {
//         (
//             Box::new(range.clone().rev().cycle()),
//             (overflowing_value + 1).abs(),
//         )
//     } else {
//         (Box::new(range.clone().cycle()), overflowing_value.abs())
//     };
//     let capped_value =
//         (0..=positive_overflowing_value).fold(0, |_, _| cycling_range.next().unwrap_or(0));
//     (capped_value, positive_overflowing_value / max)
// }

// pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
//     let magazone: Vec<&str> = magazine.to_vec();
//     let a: HashSet<&str> = magazine.iter().copied().collect();
//     let b: HashSet<&str> = note.iter().copied().collect();
//     a.difference(&b).count() > 0
// }

// #[derive(Debug)]
// pub enum CalculatorInput {
//     Add,
//     Subtract,
//     Multiply,
//     Divide,
//     Value(i32),
// }

// pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
//     let mut stack: Vec<i32> = Vec::new();
//     for i in inputs {
//         dbg!(&stack);
//         match i {
//             CalculatorInput::Add => {
//                 stack
//                     .pop()
//                     .and_then(|a| stack.pop().map(|b| stack.push(a + b)));
//             }
//             CalculatorInput::Subtract => {
//                 stack
//                     .pop()
//                     .and_then(|a| stack.pop().map(|b| stack.push(a - b)));
//             }
//             CalculatorInput::Multiply => {
//                 stack
//                     .pop()
//                     .and_then(|a| stack.pop().map(|b| stack.push(a * b)));
//             }
//             CalculatorInput::Divide => {
//                 stack
//                     .pop()
//                     .and_then(|a| stack.pop().map(|b| stack.push(a / b)));
//             }
//             CalculatorInput::Value(n) => stack.push(*n),
//         }
//     }
//     dbg!(&stack);
//     if stack.len() != 1 {
//         return None;
//     }
//     stack.pop()
// }

// pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
//     let mut word_chars = word.chars().collect::<Vec<char>>();
//     word_chars.sort_unstable();

//     for a in possible_anagrams {
//         for c in a.chars().map(|c| c.to_string()) {
//             let normalized_c = if c.is_uppercase() { c.to_lowercase().to_string() } else { c }
//         }
//     }

//     todo!()
// }

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    if first_list.is_empty() {
        return Comparison::Sublist;
    }

    if second_list.is_empty() {
        return Comparison::Superlist;
    }

    let (first_list_len, second_list_len) = (first_list.len(), second_list.len());

    let (shortest, longest, shortest_len) = if first_list_len > second_list_len {
        (second_list, first_list, second_list_len)
    } else {
        (first_list, second_list, first_list_len)
    };

    for win in longest.windows(shortest_len) {
        if win == shortest {
            if first_list_len > second_list_len {
                return Comparison::Superlist;
            } else {
                return Comparison::Sublist;
            }
        }
    }

    Comparison::Unequal
}

macro_rules! orbital_period {
    ($i:ty, $x:literal) => {
        impl Planet for $i {
            fn years_during(d: &Duration) -> f64 {
                d.0 as f64 / (Self::EARTH_YEAR_SECONDS * $x)
            }
        }
    };
}

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    const EARTH_YEAR_SECONDS: f64 = 31557600.0;

    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

orbital_period!(Mercury, 0.2408467);
orbital_period!(Venus, 0.61519726);
orbital_period!(Earth, 1.0);
orbital_period!(Mars, 1.8808158);
orbital_period!(Jupiter, 11.862615);
orbital_period!(Saturn, 29.447498);
orbital_period!(Uranus, 84.016846);
orbital_period!(Neptune, 164.79132);

// impl Planet for Mercury {
//     fn years_during(d: &Duration) -> f64 {
//         d.0 as f64 / (Self::EARTH_YEAR_SECONDS * 0.2408467)
//     }
// }
// impl Planet for Venus {
//     fn years_during(d: &Duration) -> f64 {
//         d.0 as f64 / (Self::EARTH_YEAR_SECONDS * 0.61519726)
//     }
// }
// impl Planet for Earth {
//     fn years_during(d: &Duration) -> f64 {
//         d.0 as f64 / (Self::EARTH_YEAR_SECONDS * 1.0)
//     }
// }
// impl Planet for Mars {
//     fn years_during(d: &Duration) -> f64 {
//         d.0 as f64 / (Self::EARTH_YEAR_SECONDS * 1.8808158)
//     }
// }
// impl Planet for Jupiter {
//     fn years_during(d: &Duration) -> f64 {
//         d.0 as f64 / (Self::EARTH_YEAR_SECONDS * 11.862615)
//     }
// }
// impl Planet for Saturn {
//     fn years_during(d: &Duration) -> f64 {
//         d.0 as f64 / (Self::EARTH_YEAR_SECONDS * 29.447498)
//     }
// }
// impl Planet for Uranus {
//     fn years_during(d: &Duration) -> f64 {
//         d.0 as f64 / (Self::EARTH_YEAR_SECONDS * 84.016846)
//     }
// }
// impl Planet for Neptune {
//     fn years_during(d: &Duration) -> f64 {
//         d.0 as f64 / (Self::EARTH_YEAR_SECONDS * 164.79132)
//     }
// }

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mine = Some("*".as_bytes().get(0).unwrap());
    let mut output = vec![];

    for (y, row) in minefield.iter().enumerate() {
        let mut output_row = String::new();
        let cols = row.as_bytes();

        for (x, cell) in cols.iter().enumerate() {
            if Some(cell) == mine {
                output_row.push(*cell as char);
                continue;
            }

            let bottom_row = minefield.get(y + 1);
            let top_row = if y == 0 { None } else { minefield.get(y - 1) };

            let bottom = minefield.get(y + 1).and_then(|z| z.as_bytes().get(x));
            let top = if y == 0 {
                None
            } else {
                minefield.get(y - 1).and_then(|z| z.as_bytes().get(x))
            };
            let right = cols.get(x + 1);
            let left = if x == 0 { None } else { cols.get(x - 1) };
            let bottom_left = if x == 0 {
                None
            } else {
                bottom_row.and_then(|z| z.as_bytes().get(x - 1))
            };
            let bottom_right = bottom_row.and_then(|z| z.as_bytes().get(x + 1));
            let top_left = if x == 0 {
                None
            } else {
                top_row.and_then(|z| z.as_bytes().get(x - 1))
            };
            let top_right = top_row.and_then(|z| z.as_bytes().get(x + 1));

            let mines_count = [
                bottom == mine,
                top == mine,
                right == mine,
                left == mine,
                bottom_left == mine,
                bottom_right == mine,
                top_left == mine,
                top_right == mine,
            ]
            .iter()
            .filter(|x| **x)
            .count();

            let output_char = if mines_count == 0 {
                ' '
            } else {
                char::from_digit(mines_count as u32, 10).unwrap()
            };

            output_row.push(output_char);
        }

        output.push(output_row);
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    fn remove_annotations(board: &[&str]) -> Vec<String> {
        board.iter().map(|r| remove_annotations_in_row(r)).collect()
    }

    fn remove_annotations_in_row(row: &str) -> String {
        row.chars()
            .map(|ch| match ch {
                '*' => '*',

                _ => ' ',
            })
            .collect()
    }

    fn run_test(test_case: &[&str]) {
        let cleaned = remove_annotations(test_case);

        let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();

        let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();

        assert_eq!(expected, annotate(&cleaned_strs));
    }

    #[test]

    fn no_rows() {
        #[rustfmt::skip]
        run_test(&[
        ]);
    }

    #[test]

    fn no_columns() {
        #[rustfmt::skip]
        run_test(&[
            "",
        ]);
    }

    #[test]

    fn no_mines() {
        #[rustfmt::skip]
        run_test(&[
            "   ",
            "   ",
            "   ",
        ]);
    }

    #[test]

    fn board_with_only_mines() {
        #[rustfmt::skip]
        run_test(&[
            "***",
            "***",
            "***",
        ]);
    }

    #[test]

    fn mine_surrounded_by_spaces() {
        #[rustfmt::skip]
        run_test(&[
            "111",
            "1*1",
            "111",
        ]);
    }

    #[test]

    fn space_surrounded_by_mines() {
        #[rustfmt::skip]
        run_test(&[
            "***",
            "*8*",
            "***",
        ]);
    }

    #[test]

    fn horizontal_line() {
        #[rustfmt::skip]
        run_test(&[
            "1*2*1",
        ]);
    }

    #[test]

    fn horizontal_line_mines_at_edges() {
        #[rustfmt::skip]
        run_test(&[
            "*1 1*",
        ]);
    }

    #[test]

    fn vertical_line() {
        #[rustfmt::skip]
        run_test(&[
            "1",
            "*",
            "2",
            "*",
            "1",
        ]);
    }

    #[test]

    fn vertical_line_mines_at_edges() {
        #[rustfmt::skip]
        run_test(&[
            "*",
            "1",
            " ",
            "1",
            "*",
        ]);
    }

    #[test]

    fn cross() {
        #[rustfmt::skip]    run_test(&[
            " 2*2 ",
            "25*52",
            "*****",
            "25*52",
            " 2*2 ",
        ]);
    }

    #[test]

    fn large_board() {
        #[rustfmt::skip]
        run_test(&[
            "1*22*1",
            "12*322",
            " 123*2",
            "112*4*",
            "1*22*2",
            "111111",
        ]);
    }

    // fn assert_in_delta(expected: f64, actual: f64) {
    //     let diff: f64 = (expected - actual).abs();

    //     let delta: f64 = 0.01;

    //     if diff > delta {
    //         panic!(
    //             "Your result of {} should be within {} of the expected result {}",
    //             actual, delta, expected
    //         )
    //     }
    // }

    // #[test]

    // fn earth_age() {
    //     let duration = Duration::from(1_000_000_000);

    //     assert_in_delta(31.69, Earth::years_during(&duration));
    // }

    // #[test]

    // fn mercury_age() {
    //     let duration = Duration::from(2_134_835_688);

    //     assert_in_delta(280.88, Mercury::years_during(&duration));
    // }

    // #[test]

    // fn venus_age() {
    //     let duration = Duration::from(189_839_836);

    //     assert_in_delta(9.78, Venus::years_during(&duration));
    // }

    // #[test]

    // fn mars_age() {
    //     let duration = Duration::from(2_129_871_239);

    //     assert_in_delta(35.88, Mars::years_during(&duration));
    // }

    // #[test]

    // fn jupiter_age() {
    //     let duration = Duration::from(901_876_382);

    //     assert_in_delta(2.41, Jupiter::years_during(&duration));
    // }

    // #[test]

    // fn saturn_age() {
    //     let duration = Duration::from(2_000_000_000);

    //     assert_in_delta(2.15, Saturn::years_during(&duration));
    // }

    // #[test]

    // fn uranus_age() {
    //     let duration = Duration::from(1_210_123_456);

    //     assert_in_delta(0.46, Uranus::years_during(&duration));
    // }

    // #[test]

    // fn neptune_age() {
    //     let duration = Duration::from(1_821_023_456);

    //     assert_in_delta(0.35, Neptune::years_during(&duration));
    // }

    // #[test]
    // fn test_sublist() {
    //     assert_eq!(Comparison::Sublist, sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
    //     assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]));
    //     assert_eq!(Comparison::Sublist, sublist(&[3, 4], &[1, 2, 3, 4, 5]));
    //     assert_eq!(Comparison::Equal, sublist(&[1, 2, 3], &[1, 2, 3]));
    //     assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[2, 3, 4]));
    //     assert_eq!(Comparison::Unequal, sublist(&[1, 2, 4], &[1, 2, 3, 4, 5]));
    //     assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
    //     assert_eq!(Comparison::Superlist, sublist(&['a', 's', 'd', 'f'], &[]));
    //     assert_eq!(Comparison::Equal, sublist::<i32>(&[], &[]));
    // }

    // #[test]
    // fn rust-playground() {
    //     assert_eq!((59, 0), capped(59, &(0..60)));
    //     assert_eq!((0, 0), capped(0, &(0..60)));
    //     assert_eq!((1, 1), capped(61, &(0..60)));
    //     assert_eq!((0, 2), capped(120, &(0..60)));
    //     assert_eq!((10, 2), capped(130, &(0..60)));

    //     assert_eq!((0, 0), capped(0, &(0..24)));
    //     assert_eq!((1, 1), capped(25, &(0..24)));
    //     assert_eq!((18, 1), capped(42, &(0..24)));
    //     assert_eq!((4, 3), capped(76, &(0..24)));

    //     assert_eq!((21, 0), capped(-3, &(0..24)));
    //     assert_eq!((0, 0), capped(-24, &(0..24)));
    //     assert_eq!((23, 1), capped(-25, &(0..24)));

    //     assert_eq!((0, 59), capped(59, &(0..0)));
    //     assert_eq!((0, 59), capped(59, &(1..1)));
    // }

    // #[test]
    // fn ciccia() {
    //     let magazine = "Astronomer Amy Mainzer spent hours chatting with Leonardo DiCaprio for Netflix's 'Don't Look Up'".split_whitespace().collect::<Vec<&str>>();
    //     let note = "Amy Mainzer chatting with Leonardo DiCaprio."
    //         .split_whitespace()
    //         .collect::<Vec<&str>>();
    //     assert!(can_construct_note(&magazine, &note));
    // }

    // #[test]
    // fn cicci() {
    //     assert_eq!(
    //         None,
    //         evaluate(&[
    //             CalculatorInput::Value(2),
    //             CalculatorInput::Subtract,
    //             CalculatorInput::Value(2),
    //             CalculatorInput::Subtract,
    //             CalculatorInput::Value(2),
    //             CalculatorInput::Value(2),
    //             CalculatorInput::Subtract,
    //         ])
    //     );
    // }

    // #[test]
    // fn cicciu() {
    //     assert_eq!(
    //         None,
    //         evaluate(&[
    //             CalculatorInput::Value(2),
    //             CalculatorInput::Subtract,
    //             CalculatorInput::Value(2),
    //             CalculatorInput::Subtract,
    //             CalculatorInput::Value(2),
    //             CalculatorInput::Value(2),
    //             CalculatorInput::Subtract,
    //         ])
    //     );
    // }
}
