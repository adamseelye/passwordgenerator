use std::io::{Result, stdin};
use rand::Rng;

// Main function
pub fn gen_password() -> Result<()> {
    let pass_length = get_user_input("How many characters would you like in the password?")
                        .expect("Failed to parse input");
    let excluded = get_user_input("Which characters should be excluded?")
                    .expect("Failed to parse input");

    let password = generator(pass_length, excluded).expect("Failed to generate password");
    println!("{}", password);

    Ok(())
}

// User input function
fn get_user_input(prompt: &str) -> Result<String> {
    println!("{}", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// Generator function
// User input allows custom charset definition
// Puncuation is already omitted
fn generator(pass_length: String, excluded: String) -> Result<String> {
    // takes about a minute on my machine to generate 100,000,000 characters
    // single thread - on Intel i9 chip
    let mut counter = 0;
    let mut chars: Vec<char> = excluded.chars().collect();

    use std::str::FromStr;
    let pass_var = u32::from_str(&pass_length).expect("Failed to parse input");

    let mut charset: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789)(][}{_=+?/*&^%$#@!~".to_string();

    while counter < excluded.len() {
        counter += 1;
        let excl_pop = chars.pop().expect("Failed to parse array");
        let string_index = charset.find(excl_pop).expect("Error finding character");
        charset.remove(string_index);
    }

    let vec_set: Vec<u8> = charset.as_bytes().to_vec();

    let password_len: u32 = pass_var;
    let mut rng = rand::thread_rng();

    let output: String = (0..password_len)
        .map(|_| {
            let idx = rng.gen_range(0..vec_set.len());
            vec_set[idx] as char
        })
        .collect();

    Ok(output.to_string())
}

