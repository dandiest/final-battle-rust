#[derive(Debug)]
enum Actions {
    // Actions of the character
    Resting,
    Charging(u32),
    Attacking(String),
}
#[derive(Debug)]
struct Character {
    // Character's details
    name: String,
    level: i32,
    power: Powers,
    status: Actions,
}
#[derive(Debug)]
enum Powers {
    // For now, only 2 powers
    Ice,
    Fire,
}

impl Character {
    fn print_details(&self) {
        // Print characther details
        println!(
            "Character name: {}, Character level: {}, Character power: {:?}, Character status: {:?}",
            self.name, self.level, self.power, self.status,
        );

        match &self.status {
            Actions::Resting => {
                println!("{} is resting. Make them sleep!", self.name);
            }
            Actions::Charging(percentage) => {
                println!("{} is charging. Actual charge: {} ", self.name, percentage);
            }
            Actions::Attacking(attack) => {
                println!("{} is attacking! Attack name: {}", self.name, attack);
            }
        }
    }
}

fn main() {
    let mut first_character = Character {
        // Create a character
        name: "Matthew".to_string(),
        level: 10,
        power: Powers::Ice,
        status: Actions::Resting,
    };
    let mut second_character = Character {
        // Create another character
        name: "Luke".to_string(),
        level: 5,
        power: Powers::Fire,
        status: Actions::Resting,
    };
    let mut percent = 10;

    loop {
        first_character.status = Actions::Charging(percent);
        first_character.print_details();

        second_character.status = Actions::Charging(percent);
        second_character.print_details();

        percent += 30;

        if percent > 100 {
            first_character.status = Actions::Attacking(String::from("Ice Attack!"));
            second_character.status = Actions::Attacking(String::from("Fire Attack!"));

            println!("--- Final Battle ---");
            first_character.print_details();
            second_character.print_details();
            break;
        }
    }
}
