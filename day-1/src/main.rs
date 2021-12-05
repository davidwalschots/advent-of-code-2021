fn main() -> std::io::Result<()> {
    println!("{}", number_of_increases(input_values()));

    Ok(())
}

fn number_of_increases(values: Vec<u32>) -> usize {
    values
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn input_values() -> Vec<u32> {
    include_str!("../input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::number_of_increases;

    #[test]
    fn counts_number_of_increases() {
        let input: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let result = number_of_increases(input.to_vec());

        assert_eq!(result, 7);
    }
}
