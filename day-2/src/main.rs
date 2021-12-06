fn main() {
    let input_values = input_values();
    let movement = determine_movement(basic_movement, &input_values[..]);

    println!("multiplied movement is: {}", movement.multiplied());

    let movement = determine_movement(aimed_movement, &input_values[..]);
    println!("aimed multiplied movement is: {}", movement.multiplied());
}

#[derive(Default)]
struct Position {
    x: u32,
    z: u32,
    aim: u32,
}

impl Position {
    fn multiplied(&self) -> u32 {
        self.x * self.z
    }
}

fn determine_movement(
    execute_movement: fn(position: Position, command: &str, amount: u32) -> Position,
    commands: &[String],
) -> Position {
    commands.iter().map(|item| item.split_whitespace()).fold(
        Position::default(),
        |current_position, mut item| {
            let command = item.next().unwrap_or_default();
            let amount: u32 = item.next().unwrap_or("0").parse().unwrap();

            execute_movement(current_position, command, amount)
        },
    )
}

fn basic_movement(mut current_position: Position, command: &str, amount: u32) -> Position {
    match command {
        "forward" => current_position.x += amount,
        "down" => current_position.z += amount,
        "up" => current_position.z -= amount,
        _ => (),
    }

    current_position
}

fn aimed_movement(mut current_position: Position, command: &str, amount: u32) -> Position {
    match command {
        "forward" => {
            current_position.x += amount;
            current_position.z += current_position.aim * amount;
        }
        "down" => current_position.aim += amount,
        "up" => current_position.aim -= amount,
        _ => (),
    }

    current_position
}

fn input_values() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(|item| item.to_owned())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_x_axis_movement() {
        let movement = determine_movement(basic_movement, &commands());

        assert_eq!(movement.x, 15);
    }

    #[test]
    fn determines_z_axis_movement() {
        let movement = determine_movement(basic_movement, &commands());

        assert_eq!(movement.z, 10);
    }

    #[test]
    fn determines_multiplied_movement() {
        let movement = determine_movement(basic_movement, &commands());

        assert_eq!(movement.multiplied(), 150);
    }

    #[test]
    fn determines_aimed_x_axis_movement() {
        let movement = determine_movement(aimed_movement, &commands());

        assert_eq!(movement.x, 15);
    }

    #[test]
    fn determines_aimed_z_axis_movement() {
        let movement = determine_movement(aimed_movement, &commands());

        assert_eq!(movement.z, 60);
    }

    #[test]
    fn determines_aimed_multiplied_movement() {
        let movement = determine_movement(aimed_movement, &commands());

        assert_eq!(movement.multiplied(), 900);
    }

    fn commands() -> [String; 6] {
        [
            "forward 5".to_owned(),
            "down 5".to_owned(),
            "forward 8".to_owned(),
            "up 3".to_owned(),
            "down 8".to_owned(),
            "forward 2".to_owned(),
        ]
    }
}
