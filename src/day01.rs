use std::collections::BTreeMap;

use crate::common::parse_input_as_value_list;

// The way this solution works:
// * Look up the map with the current number
// * If exists, return the product of the current number and the lookup value and finish
// * Else insert the current number with the key (2020 - value).
// * Fail if no solutions found
pub fn solve_part1<R: std::io::BufRead>(input: R) -> anyhow::Result<Option<i32>> {
    let sum_target = 2020;
    let mut target_mapping = BTreeMap::<i32, i32>::new();

    for i in parse_input_as_value_list(input) {
        let i = i?;
        if let Some(other) = target_mapping.get(&i) {
            return Ok(Some(i * other));
        }

        target_mapping.insert(sum_target - i, i);
    }

    Ok(None)
}

// Similar strategy but less elegant:
// * Work out (target - value) for entire list of numbers (T)
// * Do solution in part 1 for every entry in the new Vec, but with target being the entry,
//   which provides B and C
// * Recover A as (2020 - T)
// * Return A * B * C
pub fn solve_part2<R: std::io::BufRead>(input: R) -> anyhow::Result<Option<i64>> {
    let sum_target = 2020i64;
    let mut target_mapping = BTreeMap::<i64, i64>::new();
    let ints = parse_input_as_value_list(input).collect::<anyhow::Result<Vec<i64>>>()?;

    for target in ints.iter().map(|i| sum_target - i) {
        for i in &ints {
            if let Some(other) = target_mapping.get(&i) {
                return Ok(Some(i * other * (2020 - target)));
            }

            target_mapping.insert(target - i, *i);
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SAMPLE_INPUT: &'static [u8] = b"\
1721
979
366
299
675
1456
";

    #[test]
    fn part1_sample() {
        let reader = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part1(reader).unwrap(), Some(514579));
    }

    #[test]
    fn part2_sample() {
        let reader = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part2(reader).unwrap(), Some(241861950));
    }
}
