use std::io;

fn main() {
    println!("Welcome to Kalah!");
    loop {
        println!("To begin, enter the number where you would like to start on this board.");

        let mut first_move = String::new();
        
        io::stdin().read_line(&mut first_move)
            .expect("Failed to read the line.");

        let first_move: u32 = match first_move.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error processing your input. Is it a number?");
                continue;
            },
        };
  
        println!("Your first_move: {}", first_move);
    }
}