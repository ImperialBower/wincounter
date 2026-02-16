//! # Win Counter
//!
//! A library for calculating and reporting win percentages in games where multiple players
//! can tie for first place.
//!
//! ## Overview
//!
//! The `wincounter` crate addresses a specific problem found when reporting results in
//! competitive games: **How to clearly store and report win percentages when you have x number
//! of players, and any number of them can tie with any number of other players?** In many
//! poker TV shows, the displayed winning percentages don't include ties in their calculations.
//!
//! ## Core Concepts
//!
//! The library uses bit flags ([`PlayerFlag`]) to represent winners and ties efficiently:
//! - Each player is represented by a specific bit position (1st player = bit 0, 2nd player = bit 1, etc.)
//! - When multiple bits are set, it indicates a tie between those players
//! - Supports up to 16 players simultaneously
//!
//! ## Main Components
//!
//! - **[`win`]** - Defines player bit flags (`FIRST`, `SECOND`, etc.) and utilities for
//!   converting between player indices and flags
//! - **[`wins`]** - The [`Wins`](wins::Wins) collection type for accumulating game outcomes
//! - **[`heads_up`]** - Specialized handling for two-player games via [`HeadsUp`](heads_up::HeadsUp)
//! - **[`results`]** - The [`Results`](results::Results) type for calculating percentages from accumulated wins
//! - **[`util`]** - Utility functions for percentage calculations and other helpers
//!
//! ## Quick Start
//!
//! ### Example: Heads-Up Play
//!
//! For one-on-one contests (like poker heads-up), use the [`HeadsUp`](heads_up::HeadsUp) struct:
//!
//! ```rust
//! use wincounter::heads_up::HeadsUp;
//!
//! // In "The Hand" between Gus Hansen and Daniel Negreanu:
//! // - Daniel wins 1,365,284 times (79.73%)
//! // - Gus wins 314,904 times (18.39%)
//! // - They tie 32,116 times (1.88%)
//! let the_hand = HeadsUp::new(1_365_284, 314_904, 32_116);
//!
//! assert_eq!(79.73, (the_hand.percentage_first() * 100.0).round() / 100.0);
//! assert_eq!(18.39, (the_hand.percentage_second() * 100.0).round() / 100.0);
//! assert_eq!(1.88, (the_hand.percentage_ties() * 100.0).round() / 100.0);
//!
//! println!("{}", the_hand);
//! // Output: "79.73% (1365284), 18.39% (314904), 1.88% (32116)"
//! ```
//!
//! ### Example: Multi-Player Games
//!
//! For games with multiple players, use [`Wins`](wins::Wins) to accumulate results:
//!
//! ```rust
//! use wincounter::wins::Wins;
//! use wincounter::win::Win;
//!
//! let mut wins = Wins::default();
//!
//! // Record individual game outcomes
//! wins.add(Win::FIRST);           // Player 1 wins alone
//! wins.add(Win::SECOND);          // Player 2 wins alone
//! wins.add(Win::FIRST | Win::SECOND);  // Players 1 and 2 tie
//!
//! // Query results for a specific player
//! let (total_wins, ties) = wins.wins_for(Win::FIRST);
//! assert_eq!(total_wins, 2);  // Won alone once + tied once
//! assert_eq!(ties, 1);        // Tied once
//! ```
//!
//! ### Example: Calculating Percentages
//!
//! Use [`Results`](results::Results) to compute win percentages from accumulated data:
//!
//! ```rust
//! use wincounter::wins::Wins;
//! use wincounter::win::Win;
//! use wincounter::results::Results;
//!
//! let mut wins = Wins::default();
//! wins.add_x(Win::FIRST, 80);
//! wins.add_x(Win::SECOND, 15);
//! wins.add_x(Win::FIRST | Win::SECOND, 5);
//!
//! let results = Results::from_wins(&wins, 2);
//! let (win_pct, tie_pct) = results.wins_and_ties_percentages(0);
//!
//! // Player 1 wins alone 80% of the time, ties 5% of the time
//! assert_eq!(80.0, win_pct);
//! assert_eq!(5.0, tie_pct);
//! ```
//!
//! ## Use Cases
//!
//! - **Poker simulations** - Calculate exact odds for any number of players pre-flop, post-flop, etc.
//! - **Game theory analysis** - Model outcomes for games where ties are possible
//! - **Tournament software** - Track and display accurate win/tie statistics
//! - **Monte Carlo simulations** - Accumulate and analyze results from many game iterations
//!
//! ## Features
//!
//! - **Efficient storage** - Uses bit flags to represent winners compactly
//! - **Tie support** - First-class support for ties between any number of players
//! - **Flexible reporting** - Calculate percentages with or without ties
//! - **Up to 16 players** - Support for games with many participants
//! - **Serde support** - Serialize and deserialize results
//!
//! ## Technical Notes
//!
//! The [`PlayerFlag`] type is currently a type alias for `u16`. There are plans to refactor
//! this into a newtype wrapper (e.g., `struct PlayerFlag(u16)`) for better type safety and
//! to enable implementing methods directly on the type.

#![warn(clippy::pedantic, clippy::unwrap_used, clippy::expect_used)]

pub mod heads_up;
pub mod results;
pub mod util;
pub mod win;
pub mod wins;
///
/// TODO RF: Refactor this as a `struct PlayerFlag(u16)`.
///
/// NOTE: In retrospect, I should never do these fracking type aliases. I always
/// regret it. Just wrap it.
pub type PlayerFlag = u16;

pub trait Result {
    #[must_use]
    fn is_tie(&self) -> bool;

    #[must_use]
    fn win_for(&self, count: PlayerFlag) -> bool;
}

impl Result for PlayerFlag {
    fn is_tie(&self) -> bool {
        self.count_ones() > 1
    }

    fn win_for(&self, count: PlayerFlag) -> bool {
        self & count == count
    }
}
