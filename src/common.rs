use std::str::FromStr;

use anyhow::Context;

pub fn transform_input_as_value_list<F, R, T>(
    input: R,
    transformer: F,
) -> impl Iterator<Item = anyhow::Result<T>>
where
    F: Fn(usize, String) -> anyhow::Result<T>,
    R: std::io::BufRead,
{
    input.lines().enumerate().map(move |(i, s)| {
        transformer(
            i,
            s.with_context(|| format!("IO error while reading input on line {}", i + 1))?,
        )
    })
}

pub fn parse_input_as_value_list<R, T>(input: R) -> impl Iterator<Item = anyhow::Result<T>>
where
    R: std::io::BufRead,
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    transform_input_as_value_list(input, |i, s| {
        str::parse::<T>(&*s).with_context(|| format!("Parse error on line {}", i + 1))
    })
}

#[cfg(not(feature = "emit_times"))]
pub fn get_start_time() -> Option<std::time::Instant> {
    None
}

#[cfg(not(feature = "emit_times"))]
pub fn emit_duration(_start_time: Option<std::time::Instant>) {}

#[cfg(feature = "emit_times")]
pub fn get_start_time() -> Option<std::time::Instant> {
    Some(std::time::Instant::now())
}

#[cfg(feature = "emit_times")]
pub fn emit_duration(start_time: Option<std::time::Instant>) {
    let duration = std::time::Instant::now() - start_time.unwrap();

    eprintln!(
        "Wallclock duration: {} seconds ({} µs)",
        duration.as_secs_f32(),
        duration.as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parses_input_list_of_ints_correctly() {
        let input_str = b"1\n2\n3";
        let reader = Cursor::new(input_str);

        let parsed = parse_input_as_value_list(reader)
            .collect::<anyhow::Result<Vec<i32>>>()
            .unwrap();

        assert_eq!(&*parsed, &[1i32, 2i32, 3i32][..]);
    }

    #[test]
    fn allows_trailing_newlines() {
        let input_str = b"1\n2\n3\n";
        let reader = Cursor::new(input_str);

        let parsed = parse_input_as_value_list(reader)
            .collect::<anyhow::Result<Vec<i32>>>()
            .unwrap();

        assert_eq!(&*parsed, &[1i32, 2i32, 3i32][..]);
    }

    #[test]
    fn error_if_not_parseable() {
        let input_str = b"1\nfoo\n3";
        let reader = Cursor::new(input_str);

        assert!(parse_input_as_value_list(reader)
            .collect::<anyhow::Result<Vec<i32>>>()
            .is_err());
    }
}
