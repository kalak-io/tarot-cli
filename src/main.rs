use std::env;
use std::process;

use tarot_cli::*;

fn main() {
    println!("Let's play Tarot!");

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    run(config);

    println!("\n\nThanks for playing !");
}
