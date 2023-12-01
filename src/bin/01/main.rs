fn main() -> anyhow::Result<()> {
    let words = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let input = std::fs::read_to_string("input/01/01.txt")?;
    let mut replaced_input = input.clone();
    for (word, digit) in &words {
        replaced_input =
            replaced_input.replace(word, format!("{word}{digit}{word}{digit}{word}").as_str());
    }
    let lines = input.lines();
    let mut result: u32 = 0;
    for line in lines {
        let first_d: u8 = line
            .as_bytes()
            .iter()
            .find(|&&c| c >= b'0' && c <= b'9')
            .unwrap()
            - b'0';
        let last_d: u8 = line
            .as_bytes()
            .iter()
            .rev()
            .find(|&&c| c >= b'0' && c <= b'9')
            .unwrap()
            - b'0';
        result += first_d as u32 * 10 + last_d as u32
    }
    println!("Result: {result}");
    Ok(())
}
