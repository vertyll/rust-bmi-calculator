use std::io;

struct Person {
    weight: f32,
    height: f32,
}

impl Person {
    fn new() -> Self {
        let weight = Person::get_user_input("Enter your weight in kilograms:");
        let height = Person::get_user_input("Enter your height in meters:");

        Person { weight, height }
    }

    fn get_user_input(prompt: &str) -> f32 {
        let stdin = io::stdin();

        loop {
            println!("{prompt}");
            let mut buffer = String::new();
            stdin.read_line(&mut buffer).expect("Failed to read line");

            let buffer = buffer.trim().replace(",", ".");

            if let Ok(value) = buffer.parse::<f32>() {
                if Person::validate_input(value) {
                    return value;
                } else {
                    println!(
                        "Value must be positive and less than or equal to 1000, please try again."
                    );
                }
            } else {
                println!("Failed to convert input to a number, please try again.");
            }
        }
    }

    fn validate_input(value: f32) -> bool {
        value > 0.0 && value <= 1000.0
    }

    fn calculate_bmi(&self) -> f32 {
        self.weight / (self.height * self.height)
    }

    fn bmi_category(&self, bmi: f32) -> &'static str {
        match bmi {
            bmi if bmi < 18.5 => "You are underweight.",
            bmi if bmi < 24.9 => "Your weight is normal.",
            bmi if bmi < 29.9 => "You are overweight.",
            _ => "You are obese.",
        }
    }
}

fn main() {
    println!("BMI Calculator");

    let person = Person::new();
    let bmi = person.calculate_bmi();
    let category = person.bmi_category(bmi);

    println!("Your BMI is: {:.2}", bmi);
    println!("{}", category);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input() {
        assert_eq!(Person::validate_input(1.0), true);
        assert_eq!(Person::validate_input(1000.0), true);
        assert_eq!(Person::validate_input(1001.0), false);
        assert_eq!(Person::validate_input(0.0), false);
        assert_eq!(Person::validate_input(-5.0), false);
    }

    #[test]
    fn test_calculate_bmi() {
        let person = Person {
            weight: 70.0,
            height: 1.75,
        };
        let bmi = person.calculate_bmi();
        assert!((bmi - 22.8571).abs() < 0.0001);
    }

    #[test]
    fn test_bmi_category() {
        let person = Person {
            weight: 0.0,
            height: 0.0,
        };

        assert_eq!(person.bmi_category(16.0), "You are underweight.");
        assert_eq!(person.bmi_category(22.0), "Your weight is normal.");
        assert_eq!(person.bmi_category(28.0), "You are overweight.");
        assert_eq!(person.bmi_category(35.0), "You are obese.");
    }
}
