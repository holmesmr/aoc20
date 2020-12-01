use std::str::FromStr;
use std::{fmt, io};

pub fn parse_input_as_value_list<R, T>(input: R) -> impl Iterator<Item = T>
where
    R: io::BufRead,
    T: FromStr,
    T::Err: fmt::Display,
{
    input.lines().enumerate().map(|(i, s)| {
        str::parse::<T>(
            &*s.unwrap_or_else(|e| panic!("IO error while reading input on line {}: {}", i + 1, e)),
        )
        .unwrap_or_else(|e| panic!("Parse error on line {}: {}", i + 1, e))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parses_input_list_of_ints_correctly() {
        let input_str = b"1\n2\n3";
        let reader = Cursor::new(input_str);

        let parsed: Vec<i32> = parse_input_as_value_list(reader).collect();

        assert_eq!(&*parsed, &[1i32, 2i32, 3i32][..]);
    }

    #[test]
    fn allows_trailing_newlines() {
        let input_str = b"1\n2\n3\n";
        let reader = Cursor::new(input_str);

        let parsed: Vec<i32> = parse_input_as_value_list(reader).collect();

        assert_eq!(&*parsed, &[1i32, 2i32, 3i32][..]);
    }

    #[test]
    #[should_panic]
    fn panic_if_not_parseable() {
        let input_str = b"1\nfoo\n3";
        let reader = Cursor::new(input_str);

        let _: Vec<i32> = parse_input_as_value_list(reader).collect();
    }
}
