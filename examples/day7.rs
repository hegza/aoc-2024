use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day7.txt");

fn recurse(acc: u64, vals: &[u64]) -> Vec<u64> {
    if vals.len() == 0 {
        return vec![acc];
    }
    let x = vals[0];
    recurse(acc + x, &vals[1..vals.len()])
        .into_iter()
        .chain(recurse(acc * x, &vals[1..vals.len()]))
        .collect_vec()
}

fn recurse2(acc: u64, vals: &[u64]) -> Vec<u64> {
    if vals.len() == 0 {
        return vec![acc];
    }
    let x = vals[0];
    recurse2(acc + x, &vals[1..vals.len()])
        .into_iter()
        .chain(recurse2(acc * x, &vals[1..vals.len()]))
        .chain(recurse2(
            (acc.to_string() + &x.to_string()).parse().unwrap(),
            &vals[1..vals.len()],
        ))
        .collect_vec()
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let mut p1 = 0;
    for line in lines.clone() {
        let (test, rest) = line.split_once(": ").unwrap();
        let test = test.parse::<u64>().unwrap();

        let vals = rest
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect_vec();

        let results = recurse(vals[0], &vals[1..]);
        if results.contains(&test) {
            p1 += test;
        }
    }
    println!("{p1}");

    let mut p2 = 0;
    for line in lines {
        let (test, rest) = line.split_once(": ").unwrap();
        let test = test.parse::<u64>().unwrap();

        let vals = rest
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect_vec();

        let results = recurse2(vals[0], &vals[1..]);
        if results.contains(&test) {
            p2 += test;
        }
    }
    println!("{p2}");

    Ok(())
}
// p2 wrong: 20281198396590
// p2 wrong: 74493743489417
