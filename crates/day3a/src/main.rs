struct FoundNumber {
    number: i32,
    boundaries: ((usize, usize), (usize, usize)),
}

fn main() {
    let mut symbols: Vec<(usize, usize)> = Vec::new();
    let mut found_numbers: Vec<FoundNumber> = Vec::new();

    include_str!("../input2.txt")
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            let characters = line.chars();
            let line_lenght = line.len();
            let mut current_boundaries = ((row, 0), (row, 0));
            let mut found_number: Vec<char> = Vec::new();
            characters.enumerate().for_each(|(col, c)| {
                if !c.is_digit(10) && c != '.' {
                    symbols.push((row, col));
                }

                if c.is_numeric() {
                    if found_number.is_empty() {
                        current_boundaries.0 = (row, col);
                    }
                    found_number.push(c);
                }

                if !found_number.is_empty() && (!c.is_numeric() || col == line_lenght - 1) {
                    current_boundaries.1 = (row, col - 1);

                    let number = found_number
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                    found_numbers.push(FoundNumber {
                        number,
                        boundaries: current_boundaries,
                    });
                    found_number.clear();
                }
            });
        });

    println!(
        "{:?}",
        found_numbers
            .iter()
            .map(|number| {
                let (x1, y1) = number.boundaries.0;
                let (x2, y2) = number.boundaries.1;

                let bounds: Vec<(usize, usize)> = (x1.saturating_sub(1)..=x2 + 1)
                    .flat_map(move |i| {
                        (y1.saturating_sub(1)..=y2 + 1).filter_map(move |j| {
                            if !(i >= x1 && i <= x2 && j >= y1 && j <= y2) {
                                Some((i, j))
                            } else {
                                None
                            }
                        })
                    })
                    .collect();

                for (x, y) in bounds.clone() {
                    if symbols.contains(&(x, y)) {
                        return number.number;
                    }
                }

                0
            })
            .sum::<i32>()
    );
}
