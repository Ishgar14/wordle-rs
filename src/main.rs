use colored::Colorize;
use rand::seq::SliceRandom;

fn fetch_word() -> Result<String, std::io::Error> {
    let file_name = std::env::current_dir()?.join("src/flwords.txt");
    let buffer = std::fs::read_to_string(file_name)?;
    let buffer = buffer.lines().collect::<Vec<&str>>();

    match buffer.choose(&mut rand::thread_rng()) {
        Some(random_word) => Ok(random_word.to_string()),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not fetch one word from text file",
        )),
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut tries = 0u32;
    let mut user_word = String::new();
    let reader = std::io::stdin();
    let original_word = fetch_word()?;
    let max_tries = 5;

    print!("Type a five letter word: ");

    loop {
        println!();
        user_word.clear();
        reader.read_line(&mut user_word).unwrap();
        user_word = user_word.trim().to_string();

        if user_word.len() != 5 {
            print!("Enter five letter words only");
            continue;
        }

        tries += 1;

        if tries == max_tries {
            println!("You have reached the maximum number of tries");
            println!("The word was {}", original_word);
            return Ok(());
        }

        user_word.chars().enumerate().for_each(|(i, c)| {
            if c == original_word.chars().nth(i).unwrap() {
                print!("{}", c.to_string().green());
            } else if original_word.contains(c) {
                print!("{}", c.to_string().yellow());
            } else {
                print!("{}", c.to_string().red());
            }
        });

        if user_word == original_word {
            break;
        }
    }

    println!("\nIt took you {tries} tries");

    Ok(())
}
