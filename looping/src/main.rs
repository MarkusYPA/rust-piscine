use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?".to_owned();
    let solution = "the letter e".to_owned();
    let mut tries = 0;

    loop {
        println!("{}", riddle);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        tries += 1;
        if guess.trim().to_lowercase() == solution {
            println!("Number of trials: {}", tries);
            break;
        }
    }
}