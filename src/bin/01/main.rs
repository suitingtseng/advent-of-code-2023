fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/01/01.txt")?;
    let lines = input.lines();
    let mut result: u32 = 0;
    for line in lines {
        let first_d: u8 = line
            .as_bytes()
            .iter()
            .find(|&&c| c >= b'0' && c <= b'9')
            .unwrap()
            .clone()
            - b'0';
        let last_d: u8 = line
            .as_bytes()
            .iter()
            .rev()
            .find(|&&c| c >= b'0' && c <= b'9')
            .unwrap()
            .clone()
            - b'0';
        result += first_d as u32 * 10 + last_d as u32
    }
    println!("Result: {result}");
    Ok(())
}