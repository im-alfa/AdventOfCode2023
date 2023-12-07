fn main() {
    let cards = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (_, part2) = line.split_once(": ").unwrap();
            let (selected, correct_numbers) = part2.split_once(" | ").unwrap();

            let correct_numbers = correct_numbers
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            selected.split_whitespace().map(|number| {
                let number = number.parse::<usize>().unwrap();
                if correct_numbers.contains(&number) {
                    return 1
                }

                0
            }).sum::<i32>()
        })
        .collect::<Vec<i32>>();

    let mut solution: Vec<u32> = vec![1; cards.len()];

    for (i, count) in cards.iter().enumerate() {
        (1..=*count).for_each(|j| {
            solution[i + j as usize] += solution[i];
        });
    }

    println!("{}", solution.iter().sum::<u32>())

}
