use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

use generic_array::ArrayLength;
use mancala_full_search::{search_clean, Board, CompactKey, Settlement};
use typenum::*;

fn get_input<P, S>(board: &Board<P, S>) -> usize
where
    P: ArrayLength<u8> + Clone,
    S: Unsigned + Clone,
{
    println!(
        "{:?}",
        board.opposite_pits().iter().rev().collect::<Vec<_>>()
    );
    println!("{:?}", board.self_pits());
    loop {
        print!("Side {:?}: ", board.side);
        stdout().flush().unwrap();
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        match buf.trim().parse() {
            Ok(i) => match board.can_sow(i) {
                Ok(_) => {
                    return i;
                }
                Err(e) => eprintln!("{}", e),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn print_suggest<P, S>(
    db: &HashMap<<Board<P, S> as CompactKey>::Key, Settlement>,
    board: &Board<P, S>,
) where
    P: ArrayLength<u8> + Clone,
    S: Unsigned + Clone,
    Board<P, S>: CompactKey,
{
    let mut next_list = board.list_next_with_pos().drain().collect::<Vec<_>>();
    next_list.sort_by_key(|(b, _)| db.get(&b.key()).unwrap());
    println!("#########################");
    for (b, v) in next_list.iter().take(3) {
        println!("{:?}", b.self_pits().iter().rev().collect::<Vec<_>>());
        println!("{:?}", b.opposite_pits());
        println!("pos={:?}", v);
        println!("score={:?}", -*db.get(&b.key()).unwrap());
        println!();
    }
    println!("------------------------");
}

fn main() {
    let origin_board = Board::<U5, U3>::new(false);
    let board = origin_board.clone();
    let key = board.key();
    let (pits, seeds, stealing) = board.triple();
    let db: HashMap<_, _> = search_clean(board.clone(), 4, 1024).into();
    println!("{} pits={} seeds={}", stealing, pits, seeds);
    println!("len={} score={}", db.len(), db.get(&key).unwrap());

    loop {
        println!("******************");
        println!("start battle");
        println!("******************");
        let mut board = origin_board.clone();
        while !board.is_finished() {
            print_suggest(&db, &board);
            let pos = get_input(&board);
            board.sow(pos);
        }
    }
}
