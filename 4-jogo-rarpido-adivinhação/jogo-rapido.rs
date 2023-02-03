use std::io;
fn main() {
    println!("Adivinhe o numero | Guess the number!");

    println!("Insira seu palpite | Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler a linha | Failed to read line");

    println!("Você adivinhou | You guessed: {guess}");
}