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
            if value > 0.0 {
                return value;
            } else {
                println!("Value must be positive, please try again.");
            }
        } else {
            println!("Failed to convert input to a number, please try again.");
        }
    }
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
