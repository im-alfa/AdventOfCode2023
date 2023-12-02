fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                let (_game_id, sets) = line.split_once(": ").unwrap();
                //println!("game_id={} sets={}", game_id, sets);

                let mut cubes: [i32; 3] = [0, 0, 0];

                sets.split("; ").for_each(|set| {
                    /*
                        0 -> red
                        1 -> blue
                        2 -> green
                    */

                    set.split(", ").for_each(|cube| {
                        let (count, color) = cube.split_once(" ").unwrap();
                        let count = count.parse::<i32>().unwrap();
                        //println!("count={} color={}", count, color);

                        match color {
                            "red" => {
                                if count > cubes[0] {
                                    cubes[0] = count;
                                }
                            }
                            "blue" => {
                                if count > cubes[1] {
                                    cubes[1] = count;
                                }
                            }
                            "green" => {
                                if count > cubes[2] {
                                    cubes[2] = count;
                                }
                            }
                            _ => unreachable!(),
                        }
                    });
                });

                cubes[0] * cubes[1] * cubes[2]
            })
            .sum::<i32>()
    );
}
