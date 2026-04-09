#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

pub mod cli;
pub mod duration;
pub mod entry;

pub use entry::Entry;
