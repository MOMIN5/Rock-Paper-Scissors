use std::io;
use rand::Rng;
use std::process;
use colored::Colorize;

fn main() {
    println!("{}","                Rock Papers Scissors".bright_blue());
    println!("{} {}","Your Turn!! Please Choose Rock Paper or Scissors".bright_blue()," \n 1.Rock \n 2.Paper \n 3.Scissors");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Not a valid Option!");

    let input_int:i32 = input.replace("\n","").parse().unwrap();
    let random_int = rand::thread_rng().gen_range(1..3);
    if input_int > 3 || input_int < 1 {
        println!("Not a valid Option!");
        process::exit(0);
    }

    let mut result = "";
    if input_int == 1 && random_int == 3 || input_int == 2 && random_int == 1 || input_int == 3 && random_int == 2 {
        result = "Congrats! You won!";
    } else if input_int == random_int {
        result = "Thats a draw!";
    } else {
        result = "Aww, you lost! Better luck next time";
    }
     println!("You Choose: {}\nOpponent Chooses: {}", get_name(input_int),get_name(random_int));
    println!("{}",result.red());

}

fn get_name(x: i32) -> String {

    if x == 1 {
        return "Rock".to_string();
    }else if x == 2 {
        return "Paper".to_string();
    }else if x == 3 {
        return "Scissors".to_string();
    }
        return "null".to_string();
}


