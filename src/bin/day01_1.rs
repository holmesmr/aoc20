use aoc20::common::{emit_duration, get_start_time};

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();

    let start_time = get_start_time();

    if let Some(solution) = aoc20::day01::solve_part1(stdin.lock())? {
        println!("{}", solution);
    } else {
        eprintln!("No solutions found");
        std::process::exit(1);
    }

    emit_duration(start_time);

    Ok(())
}
