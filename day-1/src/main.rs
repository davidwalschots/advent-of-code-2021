fn main() {
    let input_values = input_values();
    println!("increases: {}", number_of_increases(&input_values));
    println!(
        "sliding increases: {}",
        number_of_sliding_window_increases(&input_values)
    );
}

fn number_of_increases(values: &Vec<u32>) -> usize {
    values
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn number_of_sliding_window_increases(values: &Vec<u32>) -> usize {
    let sliding_window_values: Vec<u32> = values
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect();

    number_of_increases(&sliding_window_values)
}

fn input_values() -> Vec<u32> {
    include_str!("../input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_number_of_increases() {
        let result = number_of_increases(&input().to_vec());

        assert_eq!(result, 7);
    }

    #[test]
    fn counts_number_of_increases_in_sliding_window() {
        let result = number_of_sliding_window_increases(&input().to_vec());

        assert_eq!(result, 5);
    }

    fn input() -> [u32; 10] {
        [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }
}
