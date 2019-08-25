use std::collections::HashSet;

use generic_array::ArrayLength;
use mancala_full_search::{Board, CompactKey};
use typenum::*;

fn calc_key<P, S>(board: &Board<P, S>) -> Vec<u8>
where
    P: ArrayLength<u8> + Clone,
    S: Unsigned + Clone,
{
    let mut key = Vec::with_capacity(P::to_usize() * 2);
    for s in board.self_pits() {
        key.push(*s);
    }
    for s in board.opposite_pits() {
        key.push(*s);
    }
    key
}

fn search<P, S>(
    u_db: &mut HashSet<<Board<P, S> as CompactKey>::Key>,
    v_db: &mut HashSet<Vec<u8>>,
    board: Board<P, S>,
) where
    P: ArrayLength<u8> + Clone,
    S: Unsigned + Clone,
    Board<P, S>: CompactKey,
{
    let u_key = board.key();
    let v_key = calc_key(&board);
    if u_db.contains(&u_key) {
        assert!(v_db.contains(&v_key));
        return;
    }
    if !board.is_finished() {
        for next in board.list_next() {
            search(u_db, v_db, next);
        }
    }
    u_db.insert(u_key);
    v_db.insert(v_key);
    assert_eq!(u_db.len(), v_db.len());
}

#[test]
fn true_1_1() {
    search(
        &mut HashSet::new(),
        &mut HashSet::new(),
        Board::<U1, U1>::new(true),
    );
}

#[test]
fn true_2_1() {
    search(
        &mut HashSet::new(),
        &mut HashSet::new(),
        Board::<U2, U1>::new(true),
    );
}

#[test]
fn true_3_1() {
    search(
        &mut HashSet::new(),
        &mut HashSet::new(),
        Board::<U3, U1>::new(true),
    );
}

#[test]
fn true_4_1() {
    search(
        &mut HashSet::new(),
        &mut HashSet::new(),
        Board::<U4, U1>::new(true),
    );
}

#[test]
fn true_5_1() {
    search(
        &mut HashSet::new(),
        &mut HashSet::new(),
        Board::<U5, U1>::new(true),
    );
}

#[test]
fn true_6_1() {
    search(
        &mut HashSet::new(),
        &mut HashSet::new(),
        Board::<U6, U1>::new(true),
    );
}

#[test]
fn false_5_2() {
    search(
        &mut HashSet::new(),
        &mut HashSet::new(),
        Board::<U5, U1>::new(false),
    );
}
