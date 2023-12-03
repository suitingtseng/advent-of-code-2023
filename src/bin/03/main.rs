use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    row: i64,
    col: i64,
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/03/03.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let m = lines.len();
    let n = lines[0].len();
    let mut total: i64 = 0;
    let mut gears: HashMap<Point, Vec<i64>> = HashMap::new();
    for i in 0..m {
        let mut j = 0;
        while j < n {
            let line = lines[i];
            let c: char = line.as_bytes()[j].into();
            if !c.is_digit(10) {
                j += 1;
            } else {
                let start = j;
                while j < n && line.as_bytes()[j].is_ascii_digit() {
                    j += 1;
                }
                let end = j;
                let gear_neighbors = vec![i as i64]
                    .into_iter()
                    .flat_map(|row| vec![row - 1, row, row + 1])
                    .flat_map(|row| {
                        ((start as i64 - 1)..(end as i64 + 1)).map(move |col| Point { row, col })
                    })
                    .filter(|p| {
                        p.row >= 0
                            && p.row < m as i64
                            && p.col >= 0
                            && p.col < n as i64
                            && !(p.row == i as i64 && p.col >= start as i64 && p.col < end as i64)
                            && lines[p.row as usize].as_bytes()[p.col as usize] == b'*'
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

    for v in gears.values() {
        if v.len() == 2 {
            total += v[0] * v[1];
        }
    }
    println!("total: {total}");
    Ok(())
}
