use std::io;

fn main() {
    println!("BMI Calculator");

    let user_weight = get_user_input("Enter your weight in kilograms:");
    let user_height = get_user_input("Enter your height in meters:");

    let user_bmi = calculate_user_bmi(user_weight, user_height);
    let bmi_result = bmi_categorization(user_bmi);

    println!("Your BMI is: {user_bmi}");
    println!("{bmi_result}");
}

fn get_user_input(prompt: &str) -> f32 {
    let stdin = io::stdin();

    loop {
        println!("{prompt}");
        let mut buffer = String::new();

        stdin.read_line(&mut buffer).expect("Failed to read line");

        if let Ok(value) = buffer.trim().parse::<f32>() {
            if validate_input(value) {
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

fn calculate_user_bmi(user_weight: f32, user_height: f32) -> f32 {
    user_weight / (user_height * user_height)
}

fn bmi_categorization(bmi: f32) -> &'static str {
    match bmi {
        bmi if bmi < 18.5 => "You are underweight.",
        bmi if bmi < 24.9 => "Your weight is normal.",
        bmi if bmi < 29.9 => "You are overweight.",
        _ => "You are obese.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for the validate_input function
    #[test]
    fn test_validate_input() {
        // Valid values: greater than 0 and less than or equal to 1000
        assert_eq!(validate_input(1.0), true); // Valid: greater than 0 and less than 1000
        assert_eq!(validate_input(1000.0), true); // Valid: equal to 1000
        assert_eq!(validate_input(1001.0), false); // Invalid: exceeds 1000
        assert_eq!(validate_input(0.0), false); // Invalid: value is 0
        assert_eq!(validate_input(-5.0), false); // Invalid: negative value
    }

    // Test for the calculate_user_bmi function
    #[test]
    fn test_calculate_user_bmi() {
        // Calculate BMI: weight / (height^2)
        let bmi1 = calculate_user_bmi(70.0, 1.75);
        let bmi2 = 22.8571;
        assert!((bmi1 - bmi2).abs() < 0.0001); // Allow a small tolerance for floating-point comparisons

        let bmi3 = calculate_user_bmi(90.0, 1.80);
        let bmi4 = 27.7778;
        assert!((bmi3 - bmi4).abs() < 0.0001);

        let bmi5 = calculate_user_bmi(50.0, 1.60);
        let bmi6 = 19.5312;
        assert!((bmi5 - bmi6).abs() < 0.0001);
    }

    // Test for the bmi_categorization function
    #[test]
    fn test_bmi_categorization() {
        // Test different BMI values and their categories
        assert_eq!(bmi_categorization(16.0), "You are underweight.");
        assert_eq!(bmi_categorization(22.0), "Your weight is normal.");
        assert_eq!(bmi_categorization(28.0), "You are overweight.");
        assert_eq!(bmi_categorization(35.0), "You are obese.");
    }
}
