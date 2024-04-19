use num_enum::TryFromPrimitive;
use num_integer::div_rem;
use std::convert::TryFrom;
use std::fmt::Display;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::ops::RangeInclusive;
use std::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, RangeToInclusive,
};
use strum_macros::FromRepr;

trait AllowedValues {
    fn range() -> RangeInclusive<u8>;
}

#[derive(Clone, Copy, FromRepr, Debug, PartialEq)]
#[repr(u8)]
pub enum File {
    FileA = 0,
    FileB = 1,
    FileC = 2,
    FileD = 3,
    FileE = 4,
    FileF = 5,
    FileG = 6,
    FileH = 7,
    INVALID = u8::MAX,
}

impl AllowedValues for File {
    fn range() -> RangeInclusive<u8> {
        0..=7
    }
}

impl From<i8> for File {
    fn from(value: i8) -> Self {
        Self::from(value as u8)
    }
}

impl From<u8> for File {
    fn from(value: u8) -> Self {
        match value {
            _ if !Self::range().contains(&value) => Self::INVALID,
            _ => Self::from_repr(value).unwrap(),
        }
    }
}

#[derive(Clone, Copy, FromRepr, Debug, PartialEq)]
#[repr(u8)]
pub enum Rank {
    Rank1 = 0,
    Rank2 = 1,
    Rank3 = 2,
    Rank4 = 3,
    Rank5 = 4,
    Rank6 = 5,
    Rank7 = 6,
    Rank8 = 7,
    INVALID = u8::MAX,
}

impl AllowedValues for Rank {
    fn range() -> RangeInclusive<u8> {
        0..=7
    }
}

impl From<i8> for Rank {
    fn from(value: i8) -> Self {
        Self::from(value as u8)
    }
}

impl From<u8> for Rank {
    fn from(value: u8) -> Self {
        match value {
            _ if !Self::range().contains(&value) => Self::INVALID,
            _ => Self::from_repr(value).unwrap(),
        }
    }
}

pub struct Point {
    file: u8,
    rank: u8,
}

#[rustfmt::skip]
#[derive(Clone, Copy, FromRepr, Debug, PartialEq)]
#[repr(u8)]
pub enum Field {
    A1 =  0, B1 =  1, C1 =  2, D1 =  3, E1 =  4, F1 =  5, G1 =  6, H1 =  7,
    A2 =  8, B2 =  9, C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
    INVALID = u8::MAX,
}

impl AllowedValues for Field {
    fn range() -> RangeInclusive<u8> {
        0..=63
    }
}

impl From<i8> for Field {
    fn from(value: i8) -> Self {
        Self::from(value as u8)
    }
}

impl From<u8> for Field {
    fn from(value: u8) -> Self {
        match value {
            _ if !Self::range().contains(&value) => Self::INVALID,
            _ => Self::from_repr(value).unwrap(),
        }
    }
}

impl From<Field> for Point {
    fn from(value: Field) -> Self {
        let (d, m) = div_rem(value as u8, 8);
        Point { file: d, rank: m }
    }
}

/*
 ------------------------------------------------
 |     NNWW |    NNW |   NN |    NNE |     NNEE |
 ------------------------------------------------
 |      NWW |     NW |    N |     NE |      NEE |
 ------------------------------------------------
 |       WW |      W |    X |      E |       EE |
 ------------------------------------------------
 |      SWW |     SW |    S |     SE |      SEE |
 ------------------------------------------------
 |     SSWW |    SSW |   SS |    SSE |     SSEE |
 ------------------------------------------------

 ------------------------------------------------
 | +8+8-1-1 | +8+8-1 | +8+8 | +8+8+1 | +8+8+1+1 |
 ------------------------------------------------
 |   +8-1-1 |   +8-1 |   +8 |   +8+1 |   +8+1+1 |
 ------------------------------------------------
 |    0-1-1 |    0-1 |    0 |    0+1 |    0+1+1 |
 ------------------------------------------------
 |   -8-1-1 |   -8-1 |   -8 |   -8+1 |   -8+1+1 |
 ------------------------------------------------
 | -8-8-1-1 | -8-8-1 | -8-8 | -8-8+1 | -8-8+1+1 |
 ------------------------------------------------

 ------------------------------------------------
 |      +14 |    +15 |  +16 |    +17 |      +18 |
 ------------------------------------------------
 |       +6 |     +7 |   +8 |    + 9 |      +10 |
 ------------------------------------------------
 |       -2 |     -1 |    0 |     +1 |       +2 |
 ------------------------------------------------
 |      -10 |     -9 |   -8 |     -7 |       -6 |
 ------------------------------------------------
 |      -18 |    -17 |  -16 |    -15 |      -14 |
 ------------------------------------------------

*/
struct MV(i8, i8);

impl Add for MV {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        MV(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[rustfmt::skip]
#[derive(Debug, Copy, Clone)]
pub enum Direction {
   NNWW,  NNW,  NN,  NNE,  NNEE, 
    NWW,   NW,   N,   NE,   NEE,
     WW,    W,   H,    E,    EE,
    SWW,   SW,   S,   SE,   SEE,
   SSWW,  SSW,  SS,  SSE,  SSEE,
}

impl Direction {
    #[rustfmt::skip]
    #[inline(always)]
    fn mv(self) -> MV {
        match self {
            Self::H => MV( 0,  0),
            Self::N => MV( 0,  1),
            Self::E => MV( 1,  0),
            Self::S => MV( 0, -1),
            Self::W => MV(-1,  0),
            Self::EE => Self::E.mv() + Self::E.mv(),
            Self::WW => Self::W.mv() + Self::W.mv(),
            Self::NN => Self::N.mv() + Self::N.mv(),
            Self::NE => Self::N.mv() + Self::E.mv(),
            Self::NW => Self::N.mv() + Self::W.mv(),
            Self::SS => Self::S.mv() + Self::S.mv(),
            Self::SE => Self::S.mv() + Self::E.mv(),
            Self::SW => Self::S.mv() + Self::W.mv(),
            Self::NNW => Self::NN.mv() + Self::W.mv(),
            Self::NNE => Self::NN.mv() + Self::E.mv(),
            Self::NWW => Self::N.mv() + Self::WW.mv(),
            Self::NEE => Self::N.mv() + Self::EE.mv(),
            Self::SWW => Self::S.mv() + Self::WW.mv(),
            Self::SEE => Self::S.mv() + Self::EE.mv(),
            Self::SSW => Self::SS.mv() + Self::W.mv(),
            Self::SSE => Self::SS.mv() + Self::E.mv(),
            Self::NNWW => Self::NN.mv() + Self::WW.mv(),
            Self::NNEE => Self::NN.mv() + Self::EE.mv(),
            Self::SSWW => Self::SS.mv() + Self::WW.mv(),
            Self::SSEE => Self::SS.mv() + Self::EE.mv(),
        }
    }
}

impl Add<Direction> for Field {
    type Output = Field;

    fn add(self, rhs: Direction) -> Self::Output {
        let mv = rhs.mv();
        let file = self.file() as i8 + mv.0;
        let rank = self.rank() as i8 + mv.1;

        Self::new(File::from(file), Rank::from(rank))
    }
}

impl Field {
    fn new(file: File, rank: Rank) -> Self {
        if file == File::INVALID {
            return Self::INVALID;
        }
        if rank == Rank::INVALID {
            return Self::INVALID;
        }

        match Field::from_repr(file as u8 + rank as u8) {
            Some(x) => x,
            None => Field::INVALID,
        }
    }
    fn rank(self) -> Rank {
        match Rank::from_repr(self as u8 / 8) {
            Some(x) => x,
            None => Rank::INVALID,
        }
    }
    fn file(self) -> File {
        match File::from_repr(self as u8 / 8) {
            Some(x) => x,
            None => File::INVALID,
        }
    }
    fn mv(self, direction: Direction) -> Field {
        self + direction
    }
}

#[derive(Default)]
struct Bitboard {
    board: u64,
    _phantom: PhantomData<usize>,
}

impl Bitboard {
    fn new(value: u64) -> Self {
        Self {
            board: value,
            _phantom: PhantomData,
        }
    }
    fn make_mask(index: u8) -> u64 {
        1u64 << index
    }

    fn set(&mut self, field: Field) {
        self.board &= Self::make_mask(field as u8)
    }

    fn unset(&mut self, field: Field) {
        self.board &= !Self::make_mask(field as u8)
    }

    fn is_set(&self, field: Field) -> bool {
        self.get(field)
    }

    fn get(&self, field: Field) -> bool {
        0 != (self.board & Self::make_mask(field as u8))
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.board | rhs.board)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.board & rhs.board)
    }
}
impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.board ^ rhs.board)
    }
}
impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.board |= rhs.board
    }
}
impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.board &= rhs.board
    }
}
impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.board ^= rhs.board
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::Output::new(!self.board)
    }
}

impl Bitboard {
    //fn iter_all(&self) -> std::Iterator {}
}
