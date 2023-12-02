fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/02/02.txt")?;

    let lines = input.lines();

    let mut total = 0;
    for line in lines {
        let (game_id_s, cubes_s) = line.split_once(":").unwrap();
        let game_id = extract_game_id(game_id_s);
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for cubes in cubes_s
            .replace(";", ",")
            .split(",")
            .into_iter()
            .map(extract_cubes)
        {
            match &cubes {
                Cubes::Red(cnt) => min_red = std::cmp::max(min_red, *cnt),
                Cubes::Green(cnt) => min_green = std::cmp::max(min_green, *cnt),
                Cubes::Blue(cnt) => min_blue = std::cmp::max(min_blue, *cnt),
            }
        }
        total += min_red * min_green * min_blue;
    }
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    println!("total: {total}");
    Ok(())
}

fn extract_game_id(s: &str) -> u64 {
    s.replace("Game ", "").parse().unwrap()
}

enum Cubes {
    Red(u64),
    Green(u64),
    Blue(u64),
}

impl Cubes {
    fn validate(&self) -> bool {
        match self {
            Cubes::Red(cnt) => *cnt <= 12,
            Cubes::Green(cnt) => *cnt <= 13,
            Cubes::Blue(cnt) => *cnt <= 14,
        }
    }
}

fn extract_cubes(s: &str) -> Cubes {
    let trimmed = s.trim();
    if trimmed.ends_with(" red") {
        Cubes::Red(trimmed.strip_suffix(" red").unwrap().parse().unwrap())
    } else if trimmed.ends_with(" green") {
        Cubes::Green(trimmed.strip_suffix(" green").unwrap().parse().unwrap())
    } else {
        Cubes::Blue(trimmed.strip_suffix(" blue").unwrap().parse().unwrap())
    }
}
