use anyhow::Result;
use std::path::Path;

fn run_day<A, B>(day: usize, f: fn(&Path) -> Result<(A, Option<B>)>) -> Result<(A, Option<B>)> {
    f(format!("data/day{}.txt", day).as_ref())
}

#[test]
fn test_day1() -> Result<()> {
    assert_eq!(
        run_day(1, advent_of_code_2020::day1::main).unwrap(),
        (55776, Some(223162626))
    );
    Ok(())
}

#[test]
fn test_day2() -> Result<()> {
    assert_eq!(
        run_day(2, advent_of_code_2020::day2::main).unwrap(),
        (528, Some(497))
    );
    Ok(())
}

#[test]
fn test_day3() -> Result<()> {
    assert_eq!(
        run_day(3, advent_of_code_2020::day3::main).unwrap(),
        (176, Some(5872458240))
    );
    Ok(())
}

#[test]
fn test_day4() -> Result<()> {
    assert_eq!(
        run_day(4, advent_of_code_2020::day4::main).unwrap(),
        (213, Some(147))
    );
    Ok(())
}

#[test]
fn test_day5() -> Result<()> {
    assert_eq!(
        run_day(5, advent_of_code_2020::day5::main).unwrap(),
        (890, Some(651))
    );
    Ok(())
}

#[test]
fn test_day6() -> Result<()> {
    assert_eq!(
        run_day(6, advent_of_code_2020::day6::main).unwrap(),
        (6457, Some(3260))
    );
    Ok(())
}

#[test]
fn test_day7() -> Result<()> {
    assert_eq!(
        run_day(7, advent_of_code_2020::day7::main).unwrap(),
        (238, Some(82930))
    );
    Ok(())
}
#[test]
fn test_day8() -> Result<()> {
    assert_eq!(
        run_day(8, advent_of_code_2020::day8::main).unwrap(),
        (1563, Some(767))
    );
    Ok(())
}

#[test]
fn test_day9() -> Result<()> {
    assert_eq!(
        run_day(9, advent_of_code_2020::day9::main).unwrap(),
        (530627549, Some(77730285))
    );
    Ok(())
}

#[test]
fn test_day10() -> Result<()> {
    assert_eq!(
        run_day(10, advent_of_code_2020::day10::main).unwrap(),
        (2368, Some(1727094849536))
    );
    Ok(())
}
