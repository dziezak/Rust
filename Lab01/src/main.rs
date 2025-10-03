
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::rng().random_range(1..101);

    let resolve:bool = loop{
        println!("Podaj liczbę!");

        let mut guess = String :: new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let mut x: u64;
        match guess.trim().parse::<u64>(){
            Ok(guess) => {
                println!("Your number: {}", guess);
                x = guess;
                if guess == 0 {
                    println!("Your guess is 0. You forfited");
                    //std::process::exit(1);
                    break false;
                }
            }
            Err(_) => {
                x = 0;
                eprintln!("Blad: to nie jest liczba w u64");
                break true;
                //std::process::exit(0);
            }
        }

        let addition = rand::rng().random_range(0..5);
        x = x + addition;
        println!("Your randomly changed number: {}", x);

        let table = tablePowerFunction(x);
        println!("{:?}", table);


    };
}

fn tablePowerFunction(x:u64) -> [u64; 10]{
    let mut table = [x; 10];
    for i in 1..10 { // [a, b)!!!!
        table[i] = table[i-1] * x;
    }
    table
}