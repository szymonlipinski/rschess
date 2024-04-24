//! This crate is a Rust chess engine.
//!
//! Basically it's my playground for learning Rust.
//!

use num_enum::TryFromPrimitive;
use num_integer::div_rem;
use std::convert::TryFrom;
use std::fmt::Display;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, RangeInclusive,
};
use strum_macros::FromRepr;

/// Trait for storing allowed values for a type.
///
/// Values outside the range should be converted to either [Option::None][std::Option::None] or invalid value
/// represented by the type.
///
trait AllowedValues {
    /// Returns range of allowed values.
    fn allowed_values() -> RangeInclusive<u8>;

    /// Checks if the given value is allowed to be converted into a proper type value.
    fn is_valid(value: u8) -> bool {
        Self::allowed_values().contains(&value)
    }
}

/// Chess File, also known as column.
/// These are named a..h.
///
/// The special `INVALID` invariant is used e.g. when converting from value > 7.
/// This way we can return proper object instead of using Option<File>, which is not allowed
/// for some of the operators.
///
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
    fn allowed_values() -> RangeInclusive<u8> {
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
            _ if !Self::is_valid(value) => Self::INVALID,
            _ => Self::from_repr(value).unwrap(),
        }
    }
}

/// Chess Ranks, also known as rows.
/// These are named 1..8
///
/// The special `INVALID` invariant is used e.g. when converting from value > 7.
/// This way we can return proper object instead of using Option<File>, which is not allowed
/// for some of the operators.
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
    fn allowed_values() -> RangeInclusive<u8> {
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
            _ if !Self::is_valid(value) => Self::INVALID,
            _ => Self::from_repr(value).unwrap(),
        }
    }
}

/// Field coordinates as numbers.
pub struct Point {
    file: u8,
    rank: u8,
}

/// Board fields.
///
/// The special `INVALID` invariant is used e.g. when converting from value > 7.
/// This way we can return proper object instead of using Option<File>, which is not allowed
/// for some of the operators.
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
    fn allowed_values() -> RangeInclusive<u8> {
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
            _ if !Self::is_valid(value) => Self::INVALID,
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

/// Type for storing move vector from one field to another.
struct MoveVector(i8, i8);

impl Add for MoveVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        MoveVector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

/// Direction to move from the current field.
///
/// The codes are taken from the geographical ones: `North`, `South`, `East`, `West`
/// and also: `North East`, and `North North West`.
///
/// Except for H, which means HERE.
///
/// 
/// **Example codes for a field:**
///
/// |          |        |      |        |          |
/// |:--------:|:------:|:----:|:------:|:--------:|
/// |     NNWW |    NNW |   NN |    NNE |     NNEE |
/// |      NWW |     NW |    N |     NE |      NEE |
/// |       WW |      W |    H |      E |       EE |
/// |      SWW |     SW |    S |     SE |      SEE |
/// |     SSWW |    SSW |   SS |    SSE |     SSEE |
///
/// **Calculations for the field position:**
///
/// |          |        |      |        |          |
/// |:--------:|:------:|:----:|:------:|:--------:|
/// | +8+8-1-1 | +8+8-1 | +8+8 | +8+8+1 | +8+8+1+1 |
/// |   +8-1-1 |   +8-1 |   +8 |   +8+1 |   +8+1+1 |
/// |    0-1-1 |    0-1 |    0 |    0+1 |    0+1+1 |
/// |   -8-1-1 |   -8-1 |   -8 |   -8+1 |   -8+1+1 |
/// | -8-8-1-1 | -8-8-1 | -8-8 | -8-8+1 | -8-8+1+1 |
///
/// **Calculations in shortened version:**
///
/// |          |        |      |        |          |
/// |:--------:|:------:|:----:|:------:|:--------:|
/// |      +14 |    +15 |  +16 |    +17 |      +18 |
/// |       +6 |     +7 |   +8 |    + 9 |      +10 |
/// |       -2 |     -1 |    0 |     +1 |       +2 |
/// |      -10 |     -9 |   -8 |     -7 |       -6 |
/// |      -18 |    -17 |  -16 |    -15 |      -14 |
#[rustfmt::skip]
#[derive(Debug, Copy, Clone)]
pub enum Direction {
   NNWW,  NNW,  NN,  NNE,  NNEE, 
    NWW,   NW,   N,   NE,   NEE,
     WW,    W,   H,    E,    EE,
    SWW,   SW,   S,   SE,   SEE,
   SSWW,  SSW,  SS,  SSE,  SSEE,
}

impl From<Direction> for MoveVector {
    #[rustfmt::skip]
    #[inline(always)]
    fn from(value: Direction) -> Self {
        match value {
            Direction::H => Self( 0,  0),
            Direction::N => Self( 0,  1),
            Direction::E => Self( 1,  0),
            Direction::S => Self( 0, -1),
            Direction::W => Self(-1,  0),
            Direction::EE => Self::from(Direction::E) + Self::from(Direction::E),
            Direction::WW => Self::from(Direction::W) + Self::from(Direction::W),
            Direction::NN => Self::from(Direction::N) + Self::from(Direction::N),
            Direction::NE => Self::from(Direction::N) + Self::from(Direction::E),
            Direction::NW => Self::from(Direction::N) + Self::from(Direction::W),
            Direction::SS => Self::from(Direction::S) + Self::from(Direction::S),
            Direction::SE => Self::from(Direction::S) + Self::from(Direction::E),
            Direction::SW => Self::from(Direction::S) + Self::from(Direction::W),
            Direction::NNW => Self::from(Direction::NN) + Self::from(Direction::W),
            Direction::NNE => Self::from(Direction::NN) + Self::from(Direction::E),
            Direction::NWW => Self::from(Direction::N) + Self::from(Direction::WW),
            Direction::NEE => Self::from(Direction::N) + Self::from(Direction::EE),
            Direction::SWW => Self::from(Direction::S) + Self::from(Direction::WW),
            Direction::SEE => Self::from(Direction::S) + Self::from(Direction::EE),
            Direction::SSW => Self::from(Direction::SS) + Self::from(Direction::W),
            Direction::SSE => Self::from(Direction::SS) + Self::from(Direction::E),
            Direction::NNWW => Self::from(Direction::NN) + Self::from(Direction::WW),
            Direction::NNEE => Self::from(Direction::NN) + Self::from(Direction::EE),
            Direction::SSWW => Self::from(Direction::SS) + Self::from(Direction::WW),
            Direction::SSEE => Self::from(Direction::SS) + Self::from(Direction::EE),
        }
    }
}

impl Add<Direction> for Field {
    type Output = Field;

    fn add(self, rhs: Direction) -> Self::Output {
        let mv = MoveVector::from(rhs);
        let file = self.file() as i8 + mv.0;
        let rank = self.rank() as i8 + mv.1;

        Self::new(File::from(file), Rank::from(rank))
    }
}

impl Field {
    /// Creates new Field from the arguments.
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
    /// Calculates the Rank for the field.
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
    /// Finds a new Field moving in the given `direction` from the current field.
    fn mv(self, direction: Direction) -> Field {
        self + direction
    }
}

/// Bitboard.
#[derive(Default)]
pub struct Bitboard {
    board: u64,
    _phantom: PhantomData<usize>,
}

impl Bitboard {
    /// Creates a new Bitboard copying the `value` as the bits representation.
    pub fn new(value: u64) -> Self {
        Self {
            board: value,
            _phantom: PhantomData,
        }
    }
    /// Creates bit mask with just one bit set (specified as the `index`).
    pub fn make_mask(index: u8) -> u64 {
        1u64 << index
    }
    /// Sets the bit for the given field.
    pub fn set(&mut self, field: Field) {
        self.board &= Self::make_mask(field as u8)
    }

    /// Clears the bit for the given field.
    pub fn unset(&mut self, field: Field) {
        self.board &= !Self::make_mask(field as u8)
    }

    /// Checks if the bitboard has set the `field`.
    pub fn is_set(&self, field: Field) -> bool {
        self.get(field)
    }

    /// Returns value for the `field`.
    pub fn get(&self, field: Field) -> bool {
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

/// Private struct used for implementing iterator for the set fields.
struct SetFields<'a> {
    board: &'a Bitboard,
    current: u8,
}

/// Private struct used for implementing iterator for the not set fields.
struct UnsetFields<'a> {
    board: &'a Bitboard,
    current: u8,
}

impl<'a> SetFields<'a> {
    fn new(board: &Bitboard) -> SetFields {
        SetFields { board, current: 0 }
    }
}

impl<'a> Iterator for SetFields<'a> {
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current = self.current + 1;
            if !File::is_valid(self.current) {
                return Option::None;
            }
            let field = Field::from(self.current);
            match self.board.get(field) == true {
                false => continue,
                true => return Option::Some(field),
            }
        }
    }
}

impl<'a> UnsetFields<'a> {
    fn new(board: &Bitboard) -> UnsetFields {
        UnsetFields { board, current: 0 }
    }
}

impl<'a> Iterator for UnsetFields<'a> {
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current = self.current + 1;
            if !File::is_valid(self.current) {
                return Option::None;
            }
            let field = Field::from(self.current);
            match self.board.get(field) == false {
                false => continue,
                true => return Option::Some(field),
            }
        }
    }
}

impl Bitboard {
    fn set_fields_iter(&self) -> SetFields {
        SetFields::new(self)
    }
    fn unset_fields_iter(&self) -> UnsetFields {
        UnsetFields::new(self)
    }
}

// -------------------------------------------------
struct FormatterFlags {
    show_board: bool,
    show_files: bool,
    show_ranks: bool,
    files: [char; 8],
    ranks: [char; 8],
    empty_field: char,
    not_empty_field: char,
}

impl Default for FormatterFlags {
    fn default() -> Self {
        Self {
            show_board: true,
            show_files: true,
            show_ranks: true,
            files: ['1', '2', '3', '4', '5', '6', '7', '8'],
            ranks: ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'],
            empty_field: ' ',
            not_empty_field: 'x',
        }
    }
}

impl Bitboard {
    fn as_string(&self, ff: FormatterFlags) -> String {
        " ".to_string()
    }
}

macro_rules! assert_eq {
    ($one:tt, $two:tt) => {
        if $one != $two {
            let diff = $one ^ $two;
            print!(diff.to_string());
        }
    };
}
