use std::str::FromStr;

use crate::common::transform_input_as_value_list;

use anyhow::{bail, Context};
use itertools::Itertools;

// A parsed policy line.
#[derive(Debug)]
struct PolicyLine {
    required_char: char,
    param1: u8,
    param2: u8,
}

// Represents a password policy where between min_count and max_count occurrences of
// required_char are present, with min_count being param1 and max_count being param2
#[derive(Debug)]
struct SledRentalPasswordPolicy {
    required_char: char,
    min_count: u8,
    max_count: u8,
}

// Represents a password policy where a password must contain required_char at the position
// param1 *XOR* param2.
#[derive(Debug)]
struct TobogganRentalPasswordPolicy {
    required_char: char,
    pos1: u8,
    pos2: u8,
}

impl SledRentalPasswordPolicy {
    pub fn new(line: PolicyLine) -> anyhow::Result<Self> {
        if line.param1 > line.param2 {
            bail!("The minimum count cannot exceed the maximum count");
        }

        Ok(Self {
            required_char: line.required_char,
            min_count: line.param1,
            max_count: line.param2,
        })
    }

    pub fn check_password(&self, password: &str) -> bool {
        let char_count = password
            .chars()
            .filter(|&c| c == self.required_char)
            .count() as u8;

        char_count >= self.min_count && char_count <= self.max_count
    }
}

impl TobogganRentalPasswordPolicy {
    pub fn new(line: PolicyLine) -> anyhow::Result<Self> {
        Ok(Self {
            required_char: line.required_char,
            pos1: line.param1,
            pos2: line.param2,
        })
    }

    pub fn check_password(&self, password: &str) -> bool {
        let i1 = self.pos1 as usize - 1;
        let i2 = self.pos2 as usize - 1;

        password
            .chars()
            .enumerate()
            .filter(|&(i, c)| (i == i1 || i == i2) && c == self.required_char)
            .exactly_one()
            .is_ok()
    }
}

const CHAR_DELIM: char = ' ';
const PARAM_DELIM: char = '-';

impl FromStr for PolicyLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_delim_pos = if let Some(i) = s.find(CHAR_DELIM) {
            i
        } else {
            bail!("Unable to find char delimiter ({:?})", CHAR_DELIM);
        };

        let (range, chr) = s.split_at(char_delim_pos);
        // chr currently contains the delimiter as well
        let chr = &chr[(CHAR_DELIM.len_utf8())..];

        let range_delim_pos = if let Some(i) = range.find(PARAM_DELIM) {
            i
        } else {
            bail!("Unable to find param delimiter ({:?})", PARAM_DELIM);
        };

        let (min_count_str, max_count_str) = range.split_at(range_delim_pos);
        // again, max_count currently contains the delimiter as well
        let max_count_str = &max_count_str[(PARAM_DELIM.len_utf8())..];

        Ok(Self {
            required_char: chr
                .chars()
                .next()
                .context("No char provided in policy line")?,
            param1: min_count_str.parse().context("Error parsing param1")?,
            param2: max_count_str.parse().context("Error parsing param2")?,
        })
    }
}

impl FromStr for SledRentalPasswordPolicy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SledRentalPasswordPolicy::new(s.parse()?)
    }
}

impl FromStr for TobogganRentalPasswordPolicy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TobogganRentalPasswordPolicy::new(s.parse()?)
    }
}

const PASSWORD_DELIM: &str = ": ";

pub fn solve_part1<R: std::io::BufRead>(input: R) -> anyhow::Result<usize> {
    transform_input_as_value_list(input, |_i, line| {
        let pw_delim_pos = if let Some(i) = line.find(PASSWORD_DELIM) {
            i
        } else {
            bail!("Unable to find password delimiter ({:?})", PASSWORD_DELIM);
        };

        let (policy_str, password) = (&*line).split_at(pw_delim_pos);
        let policy: SledRentalPasswordPolicy = policy_str.parse()?;
        let password = &password[(PASSWORD_DELIM.len())..];

        Ok(policy.check_password(password))
    })
    .fold_results(0, |acc, valid| if valid { acc + 1 } else { acc })
}

pub fn solve_part2<R: std::io::BufRead>(input: R) -> anyhow::Result<usize> {
    transform_input_as_value_list(input, |_i, line| {
        let pw_delim_pos = if let Some(i) = line.find(PASSWORD_DELIM) {
            i
        } else {
            bail!("Unable to find password delimiter ({:?})", PASSWORD_DELIM);
        };

        let (policy_str, password) = (&*line).split_at(pw_delim_pos);
        let policy: TobogganRentalPasswordPolicy = policy_str.parse()?;
        let password = &password[(PASSWORD_DELIM.len())..];

        Ok(policy.check_password(password))
    })
    .fold_results(0, |acc, valid| if valid { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    const SAMPLE_INPUT: &'static [u8] = b"\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

    #[test]
    fn test_policy_parse() {
        let policy_str = "1-3 a";

        let policy: PolicyLine = policy_str.parse().unwrap();

        assert_eq!(policy.param1, 1);
        assert_eq!(policy.param2, 3);
        assert_eq!(policy.required_char, 'a');
    }

    #[test]
    fn sled_rental_invalid_ranges_prevented() {
        let policy_str = "3-1 a";

        assert!(policy_str.parse::<SledRentalPasswordPolicy>().is_err());
    }

    #[test]
    fn part1_sample() {
        let reader = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part1(reader).unwrap(), 2);
    }

    #[test]
    fn part2_sample() {
        let reader = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part2(reader).unwrap(), 1);
    }
}
