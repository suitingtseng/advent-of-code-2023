use std::collections::HashMap;
fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/03/03.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let m = lines.len();
    let n = lines[0].len();
    let mut total: i64 = 0;
    let mut gears: HashMap<(i64, i64), Vec<i64>> = HashMap::new();
    for i in 0..m {
        let mut j = 0;
        while j < n {
            let line = lines[i];
            let c: char = line.as_bytes()[j].into();
            if !c.is_digit(10) {
                j += 1;
                continue;
            } else {
                let start = j;
                while j < n && char::from(line.as_bytes()[j]).is_digit(10) {
                    j += 1;
                }
                let end = j;
                let mut gear_neighbors = vec![i as i64]
                    .into_iter()
                    .map(|i| vec![i - 1, i, i + 1])
                    .flatten()
                    .map(|i| ((start as i64 - 1)..(end as i64 + 1)).map(move |j| (i, j)))
                    .flatten()
                    .filter(|tup| {
                        tup.0 >= 0
                            && tup.0 < m as i64
                            && tup.1 >= 0
                            && tup.1 < n as i64
                            && !(tup.0 == i as i64 && tup.1 >= start as i64 && tup.1 < end as i64)
                    })
                    .filter(|tup| {
                        char::from(lines[tup.0 as usize].as_bytes()[tup.1 as usize]) == '*'
                    });
                let number = line[start..end].parse::<i64>().unwrap();
                for tup in gear_neighbors {
                    gears
                        .entry(tup)
                        .and_modify(|v| v.push(number))
                        .or_insert(vec![number]);
                }
            }
        }
    }

    // 12 red cubes, 13 green cubes, and 14 blue cubes

    for v in gears.values() {
        if v.len() == 2 {
            total += v[0] * v[1];
        }
    }
    println!("total: {total}");
    Ok(())
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}
