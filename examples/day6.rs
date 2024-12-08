use aoc::*;
use itertools::Itertools;
use std::cmp;
use std::collections::*;

const INPUT: &str = include_str!("inputs/day6.txt");

pub fn is_within_bounds<T>((x, y): (T, T), left: T, top: T, right: T, bottom: T) -> bool
where
    T: cmp::PartialOrd,
{
    x >= left && y >= top && x <= right && y <= bottom
}

fn rotate((x, y): (i64, i64)) -> (i64, i64) {
    CARDINAL_OFFSETS[(CARDINAL_OFFSETS
        .iter()
        .position(|&(a, b)| (a, b) == (x, y))
        .unwrap()
        + 1)
        % CARDINAL_OFFSETS.len()]
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let (w, h) = (
        lines.clone().next().unwrap().chars().count(),
        lines.clone().count(),
    );

    let mut pos = (0isize, 0isize);
    let map = lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => true,
                    '^' => {
                        pos = (x as isize, y as isize);
                        true
                    }
                    '#' => false,
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let index = |x: usize, y: usize| map[y * w + x];

    let mut visited = HashSet::new();
    let mut pos = pos;
    let mut next = pos;
    let mut dir = (-1i64, 0i64);
    while is_within_bounds(next, 0isize, 0isize, w as isize, h as isize) {
        // Take a step
        {
            pos = next;
            visited.insert(pos);
        }
        // Check wall
        next = (pos.0 + dir.0 as isize, pos.1 + dir.1 as isize);
        while !index(next.0 as usize, next.1 as usize) {
            if !is_within_bounds(next, 0isize, 0isize, w as isize, h as isize) {
                break;
            }
            dir = rotate(dir);
            next = (pos.0 + dir.0 as isize, pos.1 + dir.1 as isize);
        }
    }
    let p1 = visited.len();
    println!("{p1}");

    Ok(())
}

// p2 666 too low
// p2 667 too low
