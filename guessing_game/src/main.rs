use std::io;

fn main() {
    println!("Guess a number!");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line :(");

    println!("you guessed {}", guess);
}
