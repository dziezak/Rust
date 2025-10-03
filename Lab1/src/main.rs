use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let secret_number = rand::rng().random_range(1..101);

    loop{
        println!("Podaj liczbę!");

        let mut guess = String :: new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<u64>(){
            Ok(guess) => {
                println!("Your guess: {}", guess);
                if guess == 0{
                    println!("Your guess is 0. You forfited");
                    std::process::exit(1);
                }
                match guess.cmp(&secret_number){
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        std::process::exit(0);
                    },
                }
            }
            Err(_) => {
                eprintln!("Blad: to nie jest liczba w u64");
                std::process::exit(0);
            }
        }
    }

}
