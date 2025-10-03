use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let secret_number = rand::rng().random_range(1..101);

    loop{
        println!("Podaj liczbę!");

        let mut line = String :: new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match line.trim().parse::<u64>(){
            Ok(guess_x) => {
                println!("Podano liczbe: {}", guess_x);
            }
            Err(_) => {
                eprintln!("Blad: to nie jest liczba w u64");
                std::process::exit(0);
            }
        }
        let guess_x: u64 = line.trim()
            .parse()
            .expect("Please type a number!");

        println!("Your guess: {}", guess_x);
        if(guess_x == 0){
            println!("Your guess is 0. You forfited");
            std::process::exit(1);
        }
        match guess_x.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                std::process::exit(0);
            },
        }
    }

}
