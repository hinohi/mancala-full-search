use std::env;
use std::process;

use getopts::Options;
use mancala_full_search::*;

#[derive(Debug)]
struct Args {
    pit: usize,
    stone: u8,
}

fn print_usage(program: &str, opts: &Options) -> ! {
    let brief = format!("Usage: {}", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

fn parse_args() -> Args {
    let args: Vec<_> = env::args().collect();
    let mut opt = Options::new();
    opt.optflag("h", "help", "print this help menu");
    opt.optopt("p", "pit", "number of pit(required)", "PIT");
    opt.optopt("s", "stone", "number of stone(required)", "STONE");
    let m = opt
        .parse(&args[1..])
        .unwrap_or_else(|f| panic!(f.to_string()));

    if m.opt_present("h") {
        print_usage(&args[0], &opt);
    }
    if !m.free.is_empty() {
        print_usage(&args[0], &opt);
    }
    Args {
        pit: m
            .opt_str("pit")
            .unwrap_or_else(|| print_usage(&args[0], &opt))
            .parse::<usize>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        stone: m
            .opt_str("stone")
            .unwrap_or_else(|| print_usage(&args[0], &opt))
            .parse::<u8>()
            .unwrap_or_else(|f| panic!(f.to_string())),
    }
}

fn main() {
    let args = parse_args();
    let board = Board::new(args.pit, args.stone);
    let db = InMemoryDB::new();
    let mut searcher = Searcher::new(Box::new(db));
    let score = searcher.search(&board);
    println!("score={} num={}", score, searcher.len());
}
