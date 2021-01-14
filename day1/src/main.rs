use itertools::Itertools;

fn main() -> anyhow::Result<()> {

    // include file at compile-time
    // xxd target/debug/day1 | grep "1623" -A 5
    // xxd hexdump tool
    let (a, b, c) = include_str!("input.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == 2020)
        .expect("no tuple of length 3 had a sum of 2020");

    dbg!(a, b, c);
    dbg!(a + b + c);
    dbg!(a * b * c);

    Ok(())
}
