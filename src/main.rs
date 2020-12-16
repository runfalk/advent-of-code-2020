use advent_of_code_2020::{
    day1, day10, day11, day12, day13, day14, day15, day16, day2, day3, day4, day5, day6, day7,
    day8, day9,
};
use anyhow::{anyhow, Result};
use std::path::Path;

fn pad_newlines(answer: String) -> String {
    answer.lines().collect::<Vec<_>>().join("\n   ")
}

fn as_result<A: ToString, B: ToString>(value: (A, Option<B>)) -> (String, Option<String>) {
    (
        value.0.to_string(),
        value.1.map(|answer| answer.to_string()),
    )
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        return Err(anyhow!("Not enough arguments"));
    }

    let path: Option<&Path> = if args.len() == 3 {
        Some(Path::new(&args[2]))
    } else {
        None
    };

    #[allow(overlapping_patterns)]
    let result: (String, Option<String>) = match args[1].parse() {
        Ok(1) => as_result(day1::main(path.unwrap_or(&Path::new("data/day1.txt")))?),
        Ok(2) => as_result(day2::main(path.unwrap_or(&Path::new("data/day2.txt")))?),
        Ok(3) => as_result(day3::main(path.unwrap_or(&Path::new("data/day3.txt")))?),
        Ok(4) => as_result(day4::main(path.unwrap_or(&Path::new("data/day4.txt")))?),
        Ok(5) => as_result(day5::main(path.unwrap_or(&Path::new("data/day5.txt")))?),
        Ok(6) => as_result(day6::main(path.unwrap_or(&Path::new("data/day6.txt")))?),
        Ok(7) => as_result(day7::main(path.unwrap_or(&Path::new("data/day7.txt")))?),
        Ok(8) => as_result(day8::main(path.unwrap_or(&Path::new("data/day8.txt")))?),
        Ok(9) => as_result(day9::main(path.unwrap_or(&Path::new("data/day9.txt")))?),
        Ok(10) => as_result(day10::main(path.unwrap_or(&Path::new("data/day10.txt")))?),
        Ok(11) => as_result(day11::main(path.unwrap_or(&Path::new("data/day11.txt")))?),
        Ok(12) => as_result(day12::main(path.unwrap_or(&Path::new("data/day12.txt")))?),
        Ok(13) => as_result(day13::main(path.unwrap_or(&Path::new("data/day13.txt")))?),
        Ok(14) => as_result(day14::main(path.unwrap_or(&Path::new("data/day14.txt")))?),
        Ok(15) => as_result(day15::main(path.unwrap_or(&Path::new("data/day15.txt")))?),
        Ok(16) => as_result(day16::main(path.unwrap_or(&Path::new("data/day16.txt")))?),
        Ok(1..=25) => return Err(anyhow!("No implementation for this day yet")),
        Ok(day) => return Err(anyhow!("Day {} is not a valid day for advent of code", day)),
        Err(_) => return Err(anyhow!("{:?} is not a valid day", args[1])),
    };

    println!("A: {}", pad_newlines(result.0));
    if let Some(b) = result.1 {
        println!("B: {}", pad_newlines(b));
    }

    Ok(())
}
