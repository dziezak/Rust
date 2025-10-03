
use std::io;
use rand::Rng;
use std::fs::File;
use std::io::Write;

fn main() {

    let resolve:bool = loop{
        println!("Enter number!");

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
                //x = 0;
                eprintln!("Error: x is not a number u64");
                break true;
                //std::process::exit(0);
            }
        }

        let addition = rand::rng().random_range(0..5);
        x += addition;
        println!("Your randomly changed number: {}", x);

        let table = table_power_function(x);
        println!("{:?}", table);

        println!("Let's check Collatz's Hipotesis");
        let mut res_table = [false; 10];

        for i in 0.. 10 {
            res_table[i] = collaz(table[i]);
        }
        println!("{:?}", res_table);


        //Etap 9
        let mut file = match File::create("xyz.txt"){
            Ok(file) => {file}
            Err(_file) => {
                break true;
            }
        };

        let content = format!("{:?}", res_table);
        //if let Err(_) = file.write_all(content.as_bytes()){
        if file.write_all(content.as_bytes()).is_err(){
            break true;
        }
    };

    if resolve {
        println!("Program has ended due to error");
    } else{
        println!("Program has ended due to user action");
    }
}

fn table_power_function(x:u64) -> [u64; 10]{
    let mut table = [x; 10];
    for i in 1..10 { // [a, b)!!!!
        table[i] = table[i-1] * x;
    }
    table
}

fn collaz(mut number:u64) -> bool {
    for _i in 1.. 100
    {
        if number == 1 {
            return true;
        } else if number.is_multiple_of(2) {
            number /= 2;
        } else {
            number = number * 3 + 1;
        }
    }

    if number == 1{
        return true;
    }
    false
}