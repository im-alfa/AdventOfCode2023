fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                let a: Vec<_> = line.chars().filter(|a| a.is_numeric()).collect();
                format!("{}{}", a.first().unwrap(), a.last().unwrap())
                    .parse::<i32>()
                    .unwrap()
            })
            .sum::<i32>()
    )
}
