const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                let (game_id, sets) = line.split_once(": ").unwrap();

                let a: bool = sets
                    .split("; ")
                    .map(|set| {
                        set.split(", ")
                            .map(|cube| {
                                let (count, color) = cube.split_once(" ").unwrap();
                                let count = count.parse::<i32>().unwrap();
                                match color {
                                    "red" => count <= MAX_RED_CUBES,
                                    "green" => count <= MAX_GREEN_CUBES,
                                    "blue" => count <= MAX_BLUE_CUBES,
                                    _ => unreachable!(),
                                }
                            })
                            .collect::<Vec<bool>>()
                    })
                    .all(|v| v.iter().all(|&b| b));

                match a {
                    true => game_id
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap(),
                    false => 0,
                }
            })
            .sum::<i32>()
    )
}
