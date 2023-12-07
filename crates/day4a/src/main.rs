fn main() {
    let a = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (_, part2) = line.split_once(": ").unwrap();
            let (selected, correct_numbers) = part2.split_once(" | ").unwrap();

            let correct_numbers = correct_numbers
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut points = 0;
            selected.split_whitespace().for_each(|number| {
                let number = number.parse::<usize>().unwrap();
                if correct_numbers.contains(&number) {
                    if points == 0 {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                }
            });

            points
        })
        .sum::<i32>();

    println!("{}", a);
}
