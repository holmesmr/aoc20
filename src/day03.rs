use std::str::FromStr;

use crate::common::transform_input_as_value_list;

use anyhow::{bail, Context};
use std::collections::BTreeSet;

pub struct TreeField {
    row_patterns: Box<[TreeRowPattern]>,
    pattern_width: usize,
}

impl TreeField {
    fn check_for_tree(&self, y: usize, x: usize) -> bool {
        let tree_line = &self.row_patterns[y];
        let mapped_x = (x % self.pattern_width) as u8;

        tree_line.0.contains(&mapped_x)
    }
}

pub struct TreeRowPattern(pub BTreeSet<u8>);

impl FromStr for TreeRowPattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = BTreeSet::new();

        for (i, c) in s.chars().enumerate() {
            match c {
                '.' => {}
                '#' => {
                    output.insert(i as u8);
                }
                other => bail!("Expected one of ('.', '#'), got {:?}", other),
            }
        }

        Ok(TreeRowPattern(output))
    }
}

fn parse_tree_line(i: usize, line: String) -> anyhow::Result<(usize, TreeRowPattern)> {
    Ok((
        line.len(),
        (&*line)
            .parse()
            .with_context(|| format!("Parse error on line {}", i + 1))?,
    ))
}

fn build_tree_field<R: std::io::BufRead>(input: R) -> anyhow::Result<TreeField> {
    let mut tree_lines = transform_input_as_value_list(input, parse_tree_line);
    // Take first line of input to build basic assumptions
    let (expected_length, first_line) = tree_lines
        .next()
        .unwrap_or_else(|| bail!("Expected at least 1 line in tree field"))?;
    let mut row_patterns = vec![first_line];

    for (i, line) in tree_lines.enumerate() {
        let (length, line) = line?;

        if length != expected_length {
            bail!(
                "Line {} was of length {}, but length {} is expected",
                i + 1,
                length,
                expected_length
            );
        }

        row_patterns.push(line);
    }

    Ok(TreeField {
        row_patterns: row_patterns.into(),
        pattern_width: expected_length,
    })
}

fn count_trees_in_path(tree_field: &TreeField, y_step: usize, x_step: usize) -> usize {
    (0..tree_field.row_patterns.len())
        .filter(|&i| i % y_step == 0)
        .enumerate()
        .filter(|&(n, y)| tree_field.check_for_tree(y, n * x_step))
        .count()
}

pub fn solve_part1<R: std::io::BufRead>(input: R) -> anyhow::Result<usize> {
    let tree_field = build_tree_field(input)?;

    Ok(count_trees_in_path(&tree_field, 1, 3))
}

pub fn solve_part2<R: std::io::BufRead>(input: R) -> anyhow::Result<usize> {
    let tree_field = build_tree_field(input)?;

    let r1_d1 = count_trees_in_path(&tree_field, 1, 1);
    let r3_d1 = count_trees_in_path(&tree_field, 1, 3);
    let r5_d1 = count_trees_in_path(&tree_field, 1, 5);
    let r7_d1 = count_trees_in_path(&tree_field, 1, 7);
    let r1_d2 = count_trees_in_path(&tree_field, 2, 1);

    Ok(r1_d1 * r3_d1 * r5_d1 * r7_d1 * r1_d2)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    const SAMPLE_INPUT: &'static [u8] = b"\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    #[test]
    fn test_tree_field_building() {
        use std::iter::FromIterator;

        let input = Cursor::new(SAMPLE_INPUT);
        let tree_field = build_tree_field(input).expect("Invalid tree field");

        let tree_pattern: Vec<Vec<u8>> = vec![
            vec![2, 3],
            vec![0, 4, 8],
            vec![1, 6, 9],
            vec![2, 4, 8, 10],
            vec![1, 5, 6, 9],
            vec![2, 4, 5],
            vec![1, 3, 5, 10],
            vec![1, 10],
            vec![0, 2, 3, 7],
            vec![0, 4, 5, 10],
            vec![1, 4, 8, 10],
        ];

        assert_eq!(tree_pattern.len(), tree_field.row_patterns.len());
        assert_eq!(tree_field.pattern_width, 11);

        for (expected_line, parsed_line) in tree_pattern.iter().zip(tree_field.row_patterns.iter())
        {
            let expected_line = BTreeSet::from_iter(expected_line.iter().copied());

            assert!(expected_line.difference(&parsed_line.0).next().is_none());
        }
    }

    #[test]
    fn part1_sample() {
        let reader = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part1(reader).unwrap(), 7);
    }

    #[test]
    fn part2_sample() {
        let reader = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part2(reader).unwrap(), 336);
    }
}
