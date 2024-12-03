use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day2.txt");

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let reports = lines.map(|line| {
        line.split_whitespace()
            .map(|ns| ns.parse::<u32>().unwrap())
            .collect_vec()
    });

    let p1 = reports
        .clone()
        .filter(|r| {
            let w = || r.iter().tuple_windows();
            (w().all(|(a, b)| a > b) || w().all(|(a, b)| a < b))
                && w().all(|(a, b)| a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3)
        })
        .count();
    println!("{p1}");

    let p2 = reports
        .clone()
        .filter(|r| {
            let w = || r.iter().tuple_windows();
            ((w().all(|(a, b)| a > b) || w().all(|(a, b)| a < b))
                && w().all(|(a, b)| a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3))
                || (0..r.len())
                    .map(|skip_idx| {
                        move || {
                            r[0..skip_idx]
                                .iter()
                                .chain(r[skip_idx + 1..].iter())
                                .tuple_windows()
                        }
                    })
                    .any(|w| {
                        (w().all(|(a, b)| a > b) || w().all(|(a, b)| a < b))
                            && w().all(|(a, b)| a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3)
                    })
        })
        .count();
    println!("{p2}");

    Ok(())
}
