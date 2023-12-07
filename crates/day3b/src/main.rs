struct FoundNumber {
    number: i32,
    boundaries: ((usize, usize), (usize, usize)),
}

struct Symbol {
    position: (usize, usize),
    hit_numbers: Option<[i32; 2]>,
}

fn main() {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut found_numbers: Vec<FoundNumber> = Vec::new();

    include_str!("../input.txt")
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            let characters = line.chars();
            let line_lenght = line.len();
            let mut current_boundaries = ((row, 0), (row, 0));
            let mut found_number: Vec<char> = Vec::new();
            characters.enumerate().for_each(|(col, c)| {
                if c == '*' {
                    symbols.push(Symbol {
                        position: (row, col),
                        hit_numbers: None,
                    });
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

    found_numbers.iter().for_each(|number| {
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
            symbols.iter_mut().for_each(|symbol| {
                if symbol.position == (x, y) {
                    if symbol.hit_numbers.is_none() {
                        symbol.hit_numbers = Some([number.number, 0]);
                    } else {
                        symbol.hit_numbers.as_mut().unwrap()[1] = number.number;
                    }
                }
            });
        }
    });

    println!(
        "{}",
        symbols
            .iter()
            .filter_map(|symbol| symbol.hit_numbers)
            .map(|numbers| numbers[0] * numbers[1])
            .sum::<i32>()
    );
}
