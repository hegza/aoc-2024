use regex::Regex;

//const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
const INPUT: &str = include_str!("inputs/day3.txt");

fn main() -> anyhow::Result<()> {
    let re: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut results = vec![];
    for (_, [x0, x1]) in re.captures_iter(INPUT).map(|c| c.extract()) {
        results.push((x0.parse::<u64>()?, x1.parse::<u64>()?));
    }
    let p1 = results.iter().fold(0, |acc, (x0, x1)| acc + (x0 * x1));
    println!("{p1}");

    // P2
    {
        let input = {
            let mut input = String::new();
            let mut buf = String::new();
            let mut gather = true;
            for c in INPUT.chars() {
                buf.push(c);
                if !gather && buf.ends_with("do()") {
                    buf = String::new();
                    gather = true;
                }
                if gather && buf.ends_with("don't()") {
                    gather = false;
                    input.push_str(buf.strip_suffix("don't()").unwrap());
                    buf = String::new();
                }
            }
            input
        };

        let mut results = vec![];
        for (_, [x0, x1]) in re.captures_iter(&input).map(|c| c.extract()) {
            results.push((x0.parse::<u64>()?, x1.parse::<u64>()?));
        }
        let p1 = results.iter().fold(0, |acc, (x0, x1)| acc + (x0 * x1));
        println!("{p1}");
    }

    Ok(())
}
