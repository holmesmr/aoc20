use aoc20::common::{emit_duration, get_start_time};

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();

    let start_time = get_start_time();

    println!("{}", aoc20::day03::solve_part2(stdin.lock())?);

    emit_duration(start_time);

    Ok(())
}
