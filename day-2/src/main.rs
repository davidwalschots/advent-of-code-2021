fn main() {
    let input_values = input_values();
    let movement = determine_movement(&input_values[..]);

    println!("multiplied movement is: {}", movement.multiplied());
}

#[derive(Default)]
struct Movement {
    x: u32,
    z: u32,
}

impl Movement {
    fn multiplied(&self) -> u32 {
        self.x * self.z
    }

    fn move_forward(&mut self, amount: u32) {
        self.x += amount;
    }

    fn move_down(&mut self, amount: u32) {
        self.z += amount;
    }

    fn move_up(&mut self, amount: u32) {
        self.z -= amount;
    }
}

fn determine_movement(commands: &[String]) -> Movement {
    commands.iter().map(|item| item.split_whitespace()).fold(
        Movement::default(),
        |mut movement, mut item| {
            let command = item.next().unwrap_or_default();
            let amount: u32 = item.next().unwrap_or("0").parse().unwrap();

            match command {
                "forward" => movement.move_forward(amount),
                "down" => movement.move_down(amount),
                "up" => movement.move_up(amount),
                _ => (),
            }

            movement
        },
    )
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
        let movement = determine_movement(&commands());

        assert_eq!(movement.x, 15);
    }

    #[test]
    fn determines_z_axis_movement() {
        let movement = determine_movement(&commands());

        assert_eq!(movement.z, 10);
    }

    #[test]
    fn determines_multiplied_movement() {
        let movement = determine_movement(&commands());

        assert_eq!(movement.multiplied(), 150);
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
