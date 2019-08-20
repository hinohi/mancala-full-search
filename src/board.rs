use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

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

    pub fn triple(&self) -> (usize, usize, bool) {
        (P::to_usize(), S::to_usize(), self.stealing)
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

    pub fn score(&self) -> i8 {
        let (s0, s1) = self.scores();
        if self.side == First {
            s0 as i8 - s1 as i8
        } else {
            s1 as i8 - s0 as i8
        }
    }

    pub fn store_score(&self) -> i8 {
        if self.side == First {
            self.stores[0] as i8 - self.stores[1] as i8
        } else {
            self.stores[1] as i8 - self.stores[0] as i8
        }
    }

    pub fn pit_score(&self) -> i8 {
        let s0 = self.pits[0].iter().sum::<u8>();
        let s1 = self.pits[1].iter().sum::<u8>();
        if self.side == First {
            s0 as i8 - s1 as i8
        } else {
            s1 as i8 - s0 as i8
        }
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
    pub fn list_next(&self) -> HashSet<Board<P, S>> {
        let mut set = HashSet::with_capacity(7 * 4);
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
    type Key = u16;
    fn key(&self) -> Self::Key {
        (u16::from(self.self_pits()[0]) << 8) + u16::from(self.opposite_pits()[0])
    }
}

impl<S: Unsigned + Clone> CompactKey for Board<U2, S> {
    type Key = u32;
    fn key(&self) -> Self::Key {
        let k0 = {
            let p = self.self_pits();
            (u32::from(p[0]) << 8) + u32::from(p[1])
        };
        let k1 = {
            let p = self.opposite_pits();
            (u32::from(p[0]) << 8) + u32::from(p[1])
        };
        (k0 << 16) + k1
    }
}

impl<S: Unsigned + Clone> CompactKey for Board<U3, S> {
    type Key = u64;
    fn key(&self) -> Self::Key {
        macro_rules! shift {
            ($p:ident, $t:ty) => {
                (<$t>::from($p[0]) << 16) + (<$t>::from($p[1]) << 8) + <$t>::from($p[2])
            };
        }
        let k0 = {
            let p = self.self_pits();
            shift!(p, u32)
        };
        let k1 = {
            let p = self.opposite_pits();
            shift!(p, u32)
        };
        (u64::from(k0) << 32) + u64::from(k1)
    }
}

impl<S: Unsigned + Clone> CompactKey for Board<U4, S> {
    type Key = u64;
    fn key(&self) -> Self::Key {
        macro_rules! shift {
            ($p:ident, $t:ty) => {
                (<$t>::from($p[0]) << 24)
                    + (<$t>::from($p[1]) << 16)
                    + (<$t>::from($p[2]) << 8)
                    + <$t>::from($p[3])
            };
        }
        let k0 = {
            let p = self.self_pits();
            shift!(p, u32)
        };
        let k1 = {
            let p = self.opposite_pits();
            shift!(p, u32)
        };
        (u64::from(k0) << 32) + u64::from(k1)
    }
}

impl<S: Unsigned + Clone> CompactKey for Board<U5, S> {
    type Key = u64;
    fn key(&self) -> Self::Key {
        macro_rules! shift {
            ($p:ident, $t:ty) => {
                (<$t>::from($p[0]) << 24)
                    + (<$t>::from($p[1]) << 18)
                    + (<$t>::from($p[2]) << 12)
                    + (<$t>::from($p[3]) << 6)
                    + <$t>::from($p[4])
            };
        }
        let k0 = {
            let p = self.self_pits();
            shift!(p, u32)
        };
        let k1 = {
            let p = self.opposite_pits();
            shift!(p, u32)
        };
        (u64::from(k0) << 32) + u64::from(k1)
    }
}

impl<S: Unsigned + Clone> CompactKey for Board<U6, S> {
    type Key = u64;
    fn key(&self) -> Self::Key {
        macro_rules! shift {
            ($p:ident, $t:ty) => {
                (<$t>::from($p[0]) << 25)
                    + (<$t>::from($p[1]) << 20)
                    + (<$t>::from($p[2]) << 15)
                    + (<$t>::from($p[3]) << 10)
                    + (<$t>::from($p[4]) << 5)
                    + <$t>::from($p[5])
            };
        }
        let k0 = {
            let p = self.self_pits();
            shift!(p, u32)
        };
        let k1 = {
            let p = self.opposite_pits();
            shift!(p, u32)
        };
        (u64::from(k0) << 32) + u64::from(k1)
    }
}

impl<S: Unsigned + Clone> CompactKey for Board<U7, S> {
    type Key = u64;
    fn key(&self) -> Self::Key {
        macro_rules! shift {
            ($p:ident, $t:ty) => {
                (<$t>::from($p[0]) << 24)
                    + (<$t>::from($p[1]) << 20)
                    + (<$t>::from($p[2]) << 16)
                    + (<$t>::from($p[3]) << 12)
                    + (<$t>::from($p[4]) << 8)
                    + (<$t>::from($p[5]) << 4)
                    + <$t>::from($p[6])
            };
        }
        let k0 = {
            let p = self.self_pits();
            shift!(p, u32)
        };
        let k1 = {
            let p = self.opposite_pits();
            shift!(p, u32)
        };
        (u64::from(k0) << 32) + u64::from(k1)
    }
}

impl<S: Unsigned + Clone> CompactKey for Board<U8, S> {
    type Key = u64;
    fn key(&self) -> Self::Key {
        macro_rules! shift {
            ($p:ident, $t:ty) => {
                (<$t>::from($p[0]) << 28)
                    + (<$t>::from($p[1]) << 24)
                    + (<$t>::from($p[2]) << 20)
                    + (<$t>::from($p[3]) << 16)
                    + (<$t>::from($p[4]) << 12)
                    + (<$t>::from($p[5]) << 8)
                    + (<$t>::from($p[6]) << 4)
                    + <$t>::from($p[7])
            };
        }
        let k0 = {
            let p = self.self_pits();
            shift!(p, u32)
        };
        let k1 = {
            let p = self.opposite_pits();
            shift!(p, u32)
        };
        (u64::from(k0) << 32) + u64::from(k1)
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
        let mut board = Board::<U3, U1>::new(true);
        assert!(!board.is_finished());
        assert_eq!(board.list_next().len(), 4);
        assert_eq!(board.scores(), (3, 3));
        let key1 = board.key();
        board.sow(2);
        board.sow(1);
        assert!(!board.is_finished());
        assert_eq!(board.scores(), (4, 2));
        let key2 = board.key();
        assert_ne!(key1, key2);
    }
}
