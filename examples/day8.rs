use aoc::Co2;
use itertools::Itertools;
use log::info;
use std::collections::*;

const INPUT: &str = include_str!("inputs/day8.txt");
/*const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;*/

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!(env!("CARGO_BIN_NAME"));
    let lines = INPUT.lines();
    let (w, h) = (
        lines.clone().next().unwrap().chars().count() as isize,
        lines.clone().count() as isize,
    );
    let mut antennas: HashMap<char, Vec<Co2<_>>> = HashMap::new();
    for (c, co2) in lines.clone().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| c != &'.')
            .map(move |(x, c)| (c, Co2(x, y)))
    }) {
        antennas.entry(c).or_insert(Vec::new()).push(co2);
    }

    let in_bounds = |(x, y): (isize, isize)| x >= 0 && y >= 0 && x < w && y < h;

    let mut ans = HashSet::new();
    for (&l, &r) in antennas
        .keys()
        .flat_map(|c| antennas[c].iter().tuple_combinations::<(_, _)>())
    {
        let (dx, dy) = r - l;
        let an0 = l - (dx, dy);
        let an1 = r + (dx, dy);

        if in_bounds(an0) {
            ans.insert(an0);
        }
        if in_bounds(an1) {
            ans.insert(an1);
        }
    }
    println!("p1: {}", ans.len());

    let mut ans = HashSet::new();
    for (&l, &r) in antennas
        .keys()
        .flat_map(|c| antennas[c].iter().tuple_combinations::<(_, _)>())
    {
        let (dx, dy) = r - l;
        ans.insert((r.0 as isize, r.1 as isize));
        ans.insert((l.0 as isize, l.1 as isize));
        let mut an0 = l - (dx, dy);
        let mut an1 = r + (dx, dy);

        while in_bounds(an0) {
            ans.insert(an0);
            an0 = (an0.0 - dx, an0.1 - dy);
        }
        while in_bounds(an1) {
            ans.insert(an1);
            an1 = (an1.0 + dx, an1.1 + dy);
        }
    }
    println!("p2: {}", ans.len());

    for (y, line) in lines.clone().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if ans.contains(&(x as isize, y as isize)) {
                print!("#");
            } else {
                print!("{c}");
            }
        }
        println!();
    }

    Ok(())
}
