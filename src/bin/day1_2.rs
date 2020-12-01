fn main() {
    let stdin = std::io::stdin();

    if let Some(solution) = aoc20::day1::solve_part2(stdin.lock()) {
        println!("{}", solution);
    } else {
        panic!("No solutions found!");
    }
}
