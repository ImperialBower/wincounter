#![warn(clippy::pedantic, clippy::unwrap_used, clippy::expect_used)]

pub mod heads_up;
pub mod results;
pub mod util;
pub mod win;
pub mod wins;

/// This crate was born out of my need to calculate win percentages that included ties. One thing I
/// noticed on the `Poker` TV shows that showed winning percentages was that they didn't include
/// ties in the results they displayed on the screen. While that is fine, I want to be able to show
/// both types of results if I want to. This module is designed to allow me to do that if I want.
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
