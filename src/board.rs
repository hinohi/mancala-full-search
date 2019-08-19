use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use fnv::FnvHashSet;
use generic_array::{ArrayLength, GenericArray};
use typenum::{
    consts::{U1, U2, U3, U4, U5, U6, U7, U8},
    Unsigned,
};

#[derive(Debug, Clone)]
pub struct Board<P, S>
where
    P: ArrayLength<u8>,
{
    stealing: bool,
    side: Side,
    pits: [GenericArray<u8, P>; 2],
    stores: [u8; 2],
    _game: PhantomData<S>,
}

impl<P, S> Hash for Board<P, S>
where
    P: ArrayLength<u8>,
    S: Unsigned,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.side.hash(state);
        self.pits[0].hash(state);
        self.pits[1].hash(state);
        self.stores.hash(state);
    }
}

impl<P, S> PartialEq for Board<P, S>
where
    P: ArrayLength<u8>,
    S: Unsigned,
{
    fn eq(&self, other: &Self) -> bool {
        self.side == other.side && self.pits == other.pits && self.stores == other.stores
    }
}

impl<P, S> Eq for Board<P, S>
where
    P: ArrayLength<u8>,
    S: Unsigned,
{
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Side {
    First,
    Second,
}

use Side::*;

impl Side {
    #[inline]
    pub fn as_usize(self) -> usize {
        match self {
            First => 0,
            Second => 1,
        }
    }

    #[inline]
    pub fn turned(self) -> Side {
        match self {
            First => Second,
            Second => First,
        }
    }
}

impl<P, S> Board<P, S>
where
    P: ArrayLength<u8> + Clone,
    S: Unsigned + Clone,
{
    pub fn new(stealing: bool) -> Board<P, S> {
        let pits = vec![S::to_u8(); P::to_usize()];
        Board {
            stealing,
            side: First,
            pits: [
                GenericArray::clone_from_slice(&pits),
                GenericArray::clone_from_slice(&pits),
            ],
            stores: [0, 0],
            _game: PhantomData,
        }
    }

    /// 終了判定
    pub fn is_finished(&self) -> bool {
        self.pits[0].iter().all(|s| *s == 0) || self.pits[1].iter().all(|s| *s == 0)
    }

    pub fn scores(&self) -> (u8, u8) {
        let s0 = self.stores[0] + self.pits[0].iter().sum::<u8>();
        let s1 = self.stores[1] + self.pits[1].iter().sum::<u8>();
        (s0, s1)
    }

    pub fn self_pits(&self) -> &GenericArray<u8, P> {
        &self.pits[self.side.as_usize()]
    }

    pub fn opposite_pits(&self) -> &GenericArray<u8, P> {
        &self.pits[self.side.turned().as_usize()]
    }

    fn move_seed(&mut self, side: Side, pos: usize, num: usize) -> (Side, usize) {
        if pos + num <= P::to_usize() {
            for i in pos..pos + num {
                self.pits[side.as_usize()][i] += 1;
            }
            return (side, pos + num - 1);
        }
        for i in pos..P::to_usize() {
            self.pits[side.as_usize()][i] += 1;
        }
        if self.side == side {
            self.stores[side.as_usize()] += 1;
            if pos + num == P::to_usize() + 1 {
                return (side, P::to_usize());
            }
            self.move_seed(side.turned(), 0, pos + num - P::to_usize() - 1)
        } else {
            self.move_seed(side.turned(), 0, pos + num - P::to_usize())
        }
    }

    pub fn sow(&mut self, pos: usize) {
        let num = self.pits[self.side.as_usize()][pos];
        self.pits[self.side.as_usize()][pos] = 0;
        let (side, end_pos) = self.move_seed(self.side, pos + 1, num as usize);
        if side == self.side {
            if end_pos == P::to_usize() {
                if !self.is_finished() {
                    return;
                }
            } else if self.stealing && self.pits[side.as_usize()][end_pos] == 1 {
                let opposite_pos = P::to_usize() - 1 - end_pos;
                let opposite_num = self.pits[side.turned().as_usize()][opposite_pos];
                if opposite_num > 0 {
                    self.pits[side.as_usize()][end_pos] = 0;
                    self.pits[side.turned().as_usize()][opposite_pos] = 0;
                    self.stores[side.as_usize()] += opposite_num + 1;
                }
            }
        }
        self.side = self.side.turned();
    }

    /// 今の手番での手を全列挙してそれを行った場合のユニークな盤面のセットを返す
    pub fn list_next(&self) -> FnvHashSet<Board<P, S>> {
        let mut set = FnvHashSet::with_capacity_and_hasher(32, Default::default());
        if self.is_finished() {
            return set;
        }
        let mut stack = Vec::with_capacity(4);
        stack.push(self.clone());
        while let Some(board) = stack.pop() {
            for (pos, &s) in board.self_pits().iter().enumerate() {
                if s == 0 {
                    continue;
                }
                let mut copied = board.clone();
                copied.sow(pos);
                if copied.side == self.side {
                    stack.push(copied);
                } else {
                    set.insert(copied);
                }
            }
        }
        set
    }
}

pub trait CompactKey {
    type Key: Hash + Eq;
    fn key(&self) -> Self::Key;
}

impl<S: Unsigned + Clone> CompactKey for Board<U1, S> {
    type Key = u8;
    fn key(&self) -> Self::Key {
        (self.self_pits()[0] << 4) + self.opposite_pits()[0]
    }
}

impl<S: Unsigned + Clone> CompactKey for Board<U2, S> {
    type Key = u16;
    fn key(&self) -> Self::Key {
        let k0 = (self.self_pits()[0] << 4) + self.self_pits()[1];
        let k1 = (self.opposite_pits()[0] << 4) + self.opposite_pits()[1];
        (u16::from(k0) << 8) + u16::from(k1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_1_2() {
        let mut board = Board::<U1, U2>::new(true);
        assert!(!board.is_finished());
        assert_eq!(board.list_next().len(), 1);
        assert_eq!(board.scores(), (2, 2));
        let key1 = board.key();
        board.sow(0);
        assert!(board.is_finished());
        assert_eq!(board.scores(), (1, 3));
        let key2 = board.key();
        assert_ne!(key1, key2);
    }

    #[test]
    fn smoke_3_1() {
        let mut board = Board::<U1, U2>::new(true);
        assert!(!board.is_finished());
        assert_eq!(board.list_next().len(), 1);
        assert_eq!(board.scores(), (2, 2));
        let key1 = board.key();
        board.sow(0);
        assert!(board.is_finished());
        assert_eq!(board.scores(), (1, 3));
        let key2 = board.key();
        assert_ne!(key1, key2);
    }
}
