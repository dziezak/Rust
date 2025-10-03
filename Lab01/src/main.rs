
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

    let name:String = String::from("Kornel");
    let my_favourite_numbers: (u64, u64) = (7, 7);

    println!("Usage of Tuple before merging it with string");
    println!("name: {}, Tuple: {}, {}", name, my_favourite_numbers.0, my_favourite_numbers.1);

    let my_great_tuple = double_for_break(name);

    println!("Usage of Tuple after having fun with it");
    println!("Tuple: {}, {}, {}", my_great_tuple.0, my_great_tuple.1, my_great_tuple.2);


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

fn double_for_break(name:String) -> (String, u64, u64) {
    let mut ii = 1;
    let mut jj = 1;
    'higher_one: for i in 6..100{
        'lower_one: for j in 6..100{
            if i == 6 && j == 9 {
                println!(" Omg what is going on {}{}", i, j);
                ii = i;
                jj = j;
                break 'lower_one;
            }
            if i == 21 && j == 37 {
                println!(" Omg what is going on {}{}", i, j);
                ii = i;
                jj = j;
                break 'higher_one;
            }
        }
    }
    (name+" Dzieza", ii, jj)
}

