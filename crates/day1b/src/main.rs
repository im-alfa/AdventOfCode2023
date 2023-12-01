// filthy hack, but i cba :joy:
const REPLACE: [(&str, &str); 9] = [
    ("one", "one1one"),
    ("two", "two2two"),
    ("three", "three3three"),
    ("four", "four4four"),
    ("five", "five5five"),
    ("six", "six6six"),
    ("seven", "seven7seven"),
    ("eight", "eight8eight"),
    ("nine", "nine9nine"),
];

fn main() {
    println!(
        "{:?}",
       include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut replaced_line = line.to_string();
            for (word, number) in REPLACE {
                replaced_line = replaced_line.replace(word, number);
            }

            let a: Vec<_> = replaced_line.chars().filter(|a| a.is_numeric()).collect();
            format!("{}{}", a.first().unwrap(), a.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>()
    );
}
