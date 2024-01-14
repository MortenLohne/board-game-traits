//! Traits for abstract game position representations.
//!
//! General game-agnostic tools and engines can be built on this module
//! Represents any 2-player sequential, deterministic, perfect-information game. This includes many popular games such as chess, go, xiangqi, othello, connect four and tic-tac-toe.

use self::Color::*;
use std::fmt;
use std::hash;
use std::ops;

/// Represents a player's color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl ops::Not for Color {
    type Output = Color;

    #[inline]
    fn not(self) -> Self {
        match self {
            White => Black,
            Black => White,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(match *self {
            White => "White",
            Black => "Black",
        })
    }
}

impl Color {
    /// Returns the color's discriminant. 0 for white, 1 for black.
    /// # Examples
    /// ```rust
    /// use board_game_traits::Color;
    /// assert_eq!(Color::White.disc(), 0);
    /// assert_eq!(Color::Black.disc(), 1);
    /// ```
    #[inline]
    pub fn disc(self) -> usize {
        self as u16 as usize
    }

    /// Returns the color's multiplier. -1 for black, 1 for white.
    /// # Examples
    /// ```rust
    /// use board_game_traits::Color;
    /// assert_eq!(Color::White.multiplier(), 1);
    /// assert_eq!(Color::Black.multiplier(), -1);
    /// ```
    #[inline]
    pub fn multiplier(self) -> isize {
        self as u16 as isize * -2 + 1
    }
}

/// The result of a game after it has finished.
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum GameResult {
    WhiteWin = 0,
    BlackWin = 1,
    Draw = 2,
}

impl GameResult {
    /// Returns WhiteWin for white, BlackWin for black
    #[inline]
    pub fn win_by(color: Color) -> Self {
        match color {
            White => Self::WhiteWin,
            Black => Self::BlackWin,
        }
    }
}

impl ops::Not for GameResult {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        match self {
            GameResult::WhiteWin => GameResult::BlackWin,
            GameResult::BlackWin => GameResult::WhiteWin,
            GameResult::Draw => GameResult::Draw,
        }
    }
}

/// The simplest abstract representation of a game position. Together, the provided methods encode all the rules of the game.
pub trait Position: Sized {
    /// The type for moves in the game.
    type Move: Eq + Clone + fmt::Debug;
    /// The type for a reverse move in the game.
    type ReverseMove;
    /// Optional Settings when initializing the position.
    type Settings: Default;

    /// Returns the starting position for the game. This function always produces identical values.
    #[inline]
    fn start_position() -> Self {
        Self::start_position_with_settings(&Self::Settings::default())
    }

    /// Returns the starting position for the game with the given settings.
    fn start_position_with_settings(settings: &Self::Settings) -> Self;

    /// Returns the side to move for the current position.
    fn side_to_move(&self) -> Color;

    /// Generates all legal moves for the side to move, and extends the provided data structure with them.
    fn generate_moves<E: Extend<Self::Move>>(&self, moves: &mut E);

    /// Checks if a move is legal in the current position.
    /// Enables minimax algorithms to use the killer-move heuristic in their search.
    fn move_is_legal(&self, mv: Self::Move) -> bool {
        let mut moves = vec![];
        self.generate_moves(&mut moves);
        moves.contains(&mv)
    }

    /// Plays a move in the position. Also returns an ReverseMove do take the move back.
    ///
    /// Doing and then undoing a move always restores the position to exactly the same state.
    fn do_move(&mut self, mv: Self::Move) -> Self::ReverseMove;

    /// Reverse a move made by `do_move`.
    ///
    /// Doing and then undoing a move always restores the position to exactly the same state.
    fn reverse_move(&mut self, mv: Self::ReverseMove);

    /// Returns the result if the game is decided, otherwise returns None.
    /// If the winning player always plays the last move (as in chess), implementations are allowed
    /// to only return a win when the losing player is to move.
    fn game_result(&self) -> Option<GameResult>;
}

/// A game position that also includes a heuristic static evaluation function.
/// Enables the use of many game-playing algorithms, such as minimax.
pub trait EvalPosition: Position + PartialEq + Clone {
    /// A fast, static evaluation of the current position.
    /// Returns a number between -100 and 100, where 0.0 is a draw, positive number means better for white, and negative number means better for black.
    fn static_eval(&self) -> f32;
}

/// An extended game representation, which includes many additional methods to help game-playing algorithms search more effectively.
pub trait ExtendedPosition: EvalPosition {
    /// The type for a reverse null move
    type ReverseNullMove;

    /// A representation of the position that can be hashed. Can be Self, or unit if no hashing is desired.
    type HashPosition: hash::Hash + Eq;

    fn hash_position(&self) -> Self::HashPosition;

    /// Generates only the "active" moves in a position, and appends them to the provided vector. These are moves that radically change the static evaluation of a position, e.g. captures or promotions in chess.
    /// Search algorithms may recursively search all active moves, so eventually, no moves will be appended.
    /// Required for search algorithms to use quiescence search.
    fn active_moves(&self, moves: &mut Vec<Self::Move>);

    fn null_move_is_available(&self) -> bool;

    /// Does a passing "null move".
    /// A null move is an empty which just transfers the turn to the other player. Enables the null move reduction heuristic.
    fn do_null_move(&mut self) -> Self::ReverseNullMove;

    /// Reverses a passing "null move".
    fn reverse_null_move(&mut self, reverse_move: Self::ReverseNullMove);

    /// Returns an estimate for the average branch factor of the game.
    /// Helps search algorithms guide pruning and time management.
    const BRANCH_FACTOR: u64 = 20;
}
