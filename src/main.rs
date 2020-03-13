use std::env;
use std::fs;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("No input file provided");
        std::process::exit(1);
    }

    let file = &args[1];
    let content = fs::read_to_string(file).expect("Could not read input file");

    let games: Vec<lib::Game> = content.lines().map(lib::parse_game).collect();
    let scores = lib::get_rankings(games.as_slice());
    println!("{:?}", scores);
}
