use colored::Colorize;

fn main() -> Result<(), std::io::Error> {
    let mut tries = 0u32;
    let mut buffer = String::new();
    let reader = std::io::stdin();
    let og_word = "happy";

    print!("Type a five letter word: ");

    loop {
        print!("\nType a five letter word: ");
        buffer.clear();
        reader.read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_string();
        tries += 1;

        buffer.chars().enumerate().for_each(|(i, c)| {
            if c == og_word.chars().nth(i).unwrap() {
                print!("{}", c.to_string().green());
            } else if og_word.contains(c) {
                print!("{}", c.to_string().yellow());
            } else {
                print!("{}", c.to_string().red());
            }
        });

        if buffer == og_word {
            break;
        }
    }

    println!("\nIt took you {tries} tries");

    Ok(())
}
