use std::collections::BTreeMap;

use crate::common::parse_input_as_value_list;

// The way this solution works:
// * Look up the map with the current number
// * If exists, return the product of the current number and the lookup value and finish
// * Else insert the current number with the key (2020 - value).
// * Fail if no solutions found
pub fn solve_part1<R: std::io::BufRead>(input: R) -> Option<i32> {
    let sum_target = 2020;
    let mut target_mapping = BTreeMap::<i32, i32>::new();

    for i in parse_input_as_value_list(input) {
        if let Some(other) = target_mapping.get(&i) {
            return Some(i * other);
        }

        target_mapping.insert(sum_target - i, i);
    }

    None
}

// Similar strategy but less elegant:
// * Work out (target - value) for entire list of numbers (T)
// * Do solution in part 1 for every entry in the new Vec, but with target being the entry,
//   which provides B and C
// * Recover A as (2020 - T)
// * Return A * B * C
pub fn solve_part2<R: std::io::BufRead>(input: R) -> Option<i64> {
    let sum_target = 2020i64;
    let mut target_mapping = BTreeMap::<i64, i64>::new();
    let ints: Vec<i64> = parse_input_as_value_list(input).collect();

    for target in ints.iter().map(|i| sum_target - i) {
        for i in &ints {
            if let Some(other) = target_mapping.get(&i) {
                return Some(i * other * (2020 - target));
            }

            target_mapping.insert(target - i, *i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SAMPLE_INPUT: &'static [u8] = b"1721\n979\n366\n299\n675\n1456";

    #[test]
    fn part1_sample() {
        let reader = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part1(reader), Some(514579));
    }

    #[test]
    fn part2_sample() {
        let reader = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part2(reader), Some(241861950));
    }
}
