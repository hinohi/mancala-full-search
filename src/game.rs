use std::collections::HashSet;
use std::fmt::{self, Write};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Board {
    pit: usize,
    stone: u8,
    side: usize,
    pits: [Vec<u8>; 2],
    score: [u8; 2],
}

impl fmt::Display for Board {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        if self.side == 1 {
            s += "* |";
        } else {
            s += "  |";
        }
        write!(s, "{:2}", self.score[1]).unwrap();
        write!(
            s,
            "|{}|  |",
            self.pits[1]
                .iter()
                .rev()
                .map(|p| format!("{:2}", *p))
                .collect::<Vec<String>>()
                .join("|")
        )
        .unwrap();
        if self.side == 0 {
            s += "\n* |  ";
        } else {
            s += "\n  |  ";
        }
        write!(
            s,
            "|{}|",
            self.pits[0]
                .iter()
                .map(|p| format!("{:2}", *p))
                .collect::<Vec<String>>()
                .join("|")
        )
        .unwrap();
        write!(s, "{:2}|", self.score[0]).unwrap();
        write!(dest, "{}", s)
    }
}

impl Board {
    pub fn new(pit: usize, stone: u8) -> Board {
        assert!(pit as i32 * stone as i32 * 2 <= 127);
        Board {
            pit,
            stone,
            side: 0,
            pits: [vec![stone; pit], vec![stone; pit]],
            score: [0, 0],
        }
    }

    pub fn get_key(&self) -> Vec<u8> {
        let mut key = Vec::with_capacity(self.pit * 2);
        key.extend(&self.pits[self.side]);
        key.extend(&self.pits[1 - self.side]);
        key
    }

    pub fn is_finished(&self) -> bool {
        self.pits[0].iter().sum::<u8>() == 0 || self.pits[1].iter().sum::<u8>() == 0
    }

    pub fn get_score(&self) -> i8 {
        if self.side == 0 {
            self.score[0] as i8 - self.score[1] as i8
        } else {
            self.score[1] as i8 - self.score[0] as i8
        }
    }

    fn move_stone(&mut self, side: usize, pos: usize, num: usize) -> (usize, usize) {
        if pos + num <= self.pit {
            for i in pos..pos + num {
                self.pits[side][i] += 1;
            }
            return (side, pos + num - 1);
        }
        for i in pos..self.pit {
            self.pits[side][i] += 1;
        }
        if self.side == side {
            self.score[side] += 1;
            if pos + num == self.pit + 1 {
                return (side, self.pit);
            }
            self.move_stone(1 - side, 0, pos + num - self.pit - 1)
        } else {
            self.move_stone(1 - side, 0, pos + num - self.pit)
        }
    }

    pub fn can_move(&self, pos: usize) -> bool {
        if pos >= self.pit {
            false;
        }
        self.pits[self.side][pos] != 0
    }

    pub fn move_one(&mut self, pos: usize) {
        debug_assert!(self.can_move(pos));
        debug_assert!(!self.is_finished());
        let num = self.pits[self.side][pos];
        self.pits[self.side][pos] = 0;
        let (side, end_pos) = self.move_stone(self.side, pos + 1, num as usize);
        if side == self.side {
            if end_pos == self.pit {
                if !self.is_finished() {
                    self.side = 1 - self.side;
                }
            } else if self.pits[side][end_pos] == 1 {
                let opposite_pos = self.pit - 1 - end_pos;
                let opposite_num = self.pits[1 - side][opposite_pos];
                self.pits[1 - side][opposite_pos] = 0;
                self.score[side] += opposite_num;
            }
        }
        self.side = 1 - self.side;
    }

    pub fn list_next(&self) -> HashSet<Board> {
        let mut set = HashSet::new();
        if self.is_finished() {
            return set;
        }
        let mut stack = vec![self.clone()];
        while !stack.is_empty() {
            let board = stack.pop().unwrap();
            for pos in 0..self.pit {
                if !board.can_move(pos) {
                    continue;
                }
                let mut copied = board.clone();
                copied.move_one(pos);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_11() {
        let mut b = Board::new(1, 1);
        assert!(!b.is_finished());
        assert!(b.can_move(0));
        assert_eq!(b.get_score(), 0);
        b.move_one(0);
        assert!(b.is_finished());
        assert_eq!(b.side, 1);
        assert_eq!(b.get_score(), -1);
    }

    #[test]
    fn list_11() {
        let b = Board::new(1, 1);
        let list = b.list_next();
        assert_eq!(list.len(), 1);
        let b = list.iter().next().unwrap();
        assert_eq!(b.side, 1);
        assert_eq!(b.pits[0], vec![0]);
        assert_eq!(b.pits[1], vec![1]);
        assert_eq!(b.score, [1, 0]);
    }
}
