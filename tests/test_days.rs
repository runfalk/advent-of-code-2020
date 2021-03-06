use anyhow::Result;
use std::path::Path;

fn run_day<A, B>(day: usize, f: fn(&Path) -> Result<(A, Option<B>)>) -> Result<(A, Option<B>)> {
    f(format!("data/day{}.txt", day).as_ref())
}

#[test]
fn test_day1() -> Result<()> {
    assert_eq!(
        run_day(1, advent_of_code_2020::day1::main)?,
        (55776, Some(223162626))
    );
    Ok(())
}

#[test]
fn test_day2() -> Result<()> {
    assert_eq!(
        run_day(2, advent_of_code_2020::day2::main)?,
        (528, Some(497))
    );
    Ok(())
}

#[test]
fn test_day3() -> Result<()> {
    assert_eq!(
        run_day(3, advent_of_code_2020::day3::main)?,
        (176, Some(5872458240))
    );
    Ok(())
}

#[test]
fn test_day4() -> Result<()> {
    assert_eq!(
        run_day(4, advent_of_code_2020::day4::main)?,
        (213, Some(147))
    );
    Ok(())
}

#[test]
fn test_day5() -> Result<()> {
    assert_eq!(
        run_day(5, advent_of_code_2020::day5::main)?,
        (890, Some(651))
    );
    Ok(())
}

#[test]
fn test_day6() -> Result<()> {
    assert_eq!(
        run_day(6, advent_of_code_2020::day6::main)?,
        (6457, Some(3260))
    );
    Ok(())
}

#[test]
fn test_day7() -> Result<()> {
    assert_eq!(
        run_day(7, advent_of_code_2020::day7::main)?,
        (238, Some(82930))
    );
    Ok(())
}
#[test]
fn test_day8() -> Result<()> {
    assert_eq!(
        run_day(8, advent_of_code_2020::day8::main)?,
        (1563, Some(767))
    );
    Ok(())
}

#[test]
fn test_day9() -> Result<()> {
    assert_eq!(
        run_day(9, advent_of_code_2020::day9::main)?,
        (530627549, Some(77730285))
    );
    Ok(())
}

#[test]
fn test_day10() -> Result<()> {
    assert_eq!(
        run_day(10, advent_of_code_2020::day10::main)?,
        (2368, Some(1727094849536))
    );
    Ok(())
}

#[test]
fn test_day11() -> Result<()> {
    assert_eq!(
        run_day(11, advent_of_code_2020::day11::main)?,
        (2346, Some(2111))
    );
    Ok(())
}

#[test]
fn test_day12() -> Result<()> {
    assert_eq!(
        run_day(12, advent_of_code_2020::day12::main)?,
        (904, Some(18747))
    );
    Ok(())
}

#[test]
fn test_day13() -> Result<()> {
    assert_eq!(
        run_day(13, advent_of_code_2020::day13::main)?,
        (4782, Some(1_118_684_865_113_056))
    );
    Ok(())
}

#[test]
fn test_day14() -> Result<()> {
    assert_eq!(
        run_day(14, advent_of_code_2020::day14::main)?,
        (9_628_746_976_360, Some(4_574_598_714_592))
    );
    Ok(())
}

#[test]
fn test_day15() -> Result<()> {
    assert_eq!(
        run_day(15, advent_of_code_2020::day15::main)?,
        (234, Some(8984))
    );
    Ok(())
}

#[test]
fn test_day16() -> Result<()> {
    assert_eq!(
        run_day(16, advent_of_code_2020::day16::main)?,
        (25984, Some(1265347500049))
    );
    Ok(())
}

#[test]
fn test_day17() -> Result<()> {
    assert_eq!(
        run_day(17, advent_of_code_2020::day17::main)?,
        (391, Some(2264))
    );
    Ok(())
}

#[test]
fn test_day18() -> Result<()> {
    assert_eq!(
        run_day(18, advent_of_code_2020::day18::main)?,
        (3647606140187, Some(323802071857594))
    );
    Ok(())
}

#[test]
fn test_day19() -> Result<()> {
    assert_eq!(
        run_day(19, advent_of_code_2020::day19::main)?,
        (147, Some(263))
    );
    Ok(())
}

#[test]
fn test_day20() -> Result<()> {
    assert_eq!(
        run_day(20, advent_of_code_2020::day20::main)?,
        (18_411_576_553_343, Some(2002))
    );
    Ok(())
}

#[test]
fn test_day21() -> Result<()> {
    assert_eq!(
        run_day(21, advent_of_code_2020::day21::main)?,
        (
            2412,
            Some("mfp,mgvfmvp,nhdjth,hcdchl,dvkbjh,dcvrf,bcjz,mhnrqp".to_owned())
        )
    );
    Ok(())
}

#[test]
fn test_day22() -> Result<()> {
    assert_eq!(
        run_day(22, advent_of_code_2020::day22::main)?,
        (32448, Some(32949))
    );
    Ok(())
}

#[test]
fn test_day23() -> Result<()> {
    assert_eq!(
        run_day(23, advent_of_code_2020::day23::main)?,
        ("95648732".to_owned(), Some(192515314252))
    );
    Ok(())
}

#[test]
fn test_day24() -> Result<()> {
    assert_eq!(
        run_day(24, advent_of_code_2020::day24::main)?,
        (320, Some(3777))
    );
    Ok(())
}

#[test]
fn test_day25() -> Result<()> {
    assert_eq!(
        run_day(25, advent_of_code_2020::day25::main)?,
        (16457981, None)
    );
    Ok(())
}
