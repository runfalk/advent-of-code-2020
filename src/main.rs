use advent_of_code_2020::day1;
use anyhow::{anyhow, Result};

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

    #[allow(overlapping_patterns)]
    let result: (String, Option<String>) = match args[1].parse()? {
        1 => as_result(day1::main(&args[2..])?),
        1..=25 => return Err(anyhow!("No implementation for this day yet")),
        day => return Err(anyhow!("Day {} is not a valid day for advent of code", day)),
    };

    println!("A: {}", pad_newlines(result.0));
    if let Some(b) = result.1 {
        println!("B: {}", pad_newlines(b));
    }

    Ok(())
}
