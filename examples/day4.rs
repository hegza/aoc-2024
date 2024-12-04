use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day4.txt");

fn search(x: isize, y: isize, grid: &[Vec<char>], pat: &[char]) -> u64 {
    let mut found = 0;
    for (dx, dy) in [
        (-1isize, -1isize),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        if (0isize..pat.len() as isize).all(|d| {
            let x = x + dx * d;
            let y = y + dy * d;
            if x < 0 || y < 0 || x >= grid[0].len() as isize || y >= grid.len() as isize {
                return false;
            }
            grid[y as usize][x as usize] == pat[d as usize]
        }) {
            found += 1;
        }
    }
    found
}

fn search_p2(x: isize, y: isize, grid: &[Vec<char>], pat: &[char]) -> u64 {
    let mut found = 0;
    for (dx, dy) in [(-1isize, -1isize), (-1, 1), (1, -1), (1, 1)] {
        if (0isize..pat.len() as isize).all(|d| {
            let x = x + dx * d;
            let y = y + dy * d;
            if x < 0 || y < 0 || x >= grid[0].len() as isize || y >= grid.len() as isize {
                return false;
            }
            grid[y as usize][x as usize] == pat[d as usize]
        }) {
            found += 1;
        }
    }
    found
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();
    let grid = lines
        .map(|line| line.chars().map(|c| c).collect_vec())
        .collect_vec();

    let word = vec!['X', 'M', 'A', 'S'];
    let mut p1 = 0;
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            p1 += search(x as isize, y as isize, &grid, &word);
        }
    }
    println!("{p1}");

    let mut p2 = 0;
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if search_p2(x as isize, y as isize, &grid, &vec!['A', 'M']) == 2
                && search_p2(x as isize, y as isize, &grid, &vec!['A', 'S']) == 2
                && !((grid[y - 1][x - 1] == 'M' && grid[y][x] == 'A' && grid[y + 1][x + 1] == 'M')
                    || (grid[y - 1][x - 1] == 'S'
                        && grid[y][x] == 'A'
                        && grid[y + 1][x + 1] == 'S'))
            {
                p2 += 1;
            }
        }
    }
    println!("{p2}");

    Ok(())
}

// 1928 too high
