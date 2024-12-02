const INPUT: &str = include_str!("inputs/day1.txt");

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let (mut a, mut b): (Vec<_>, Vec<_>) = lines
        .map(|line| {
            let mut toks = line.split_whitespace();
            {
                (
                    toks.next().unwrap().parse::<u32>().unwrap(),
                    toks.next().unwrap().parse::<u32>().unwrap(),
                )
            }
        })
        .unzip();

    a.sort();
    b.sort();

    let p1 = a
        .iter()
        .zip(b.iter())
        .fold(0, |acc, (a, b)| acc + a.abs_diff(*b));

    println!("{p1}");

    let mut p2 = 0;
    for ax in a {
        let mut count = 0;
        for bx in &b {
            if *bx == ax {
                count += 1;
            }
        }
        p2 += ax * count;
    }
    println!("{p2}");

    Ok(())
}
