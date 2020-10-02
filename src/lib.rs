// list here all files/modules
mod batman;
mod joker;
mod riddler;
mod robin;

// list here all functions that form the
// interface of the library
pub use crate::batman::batman_quote;
pub use crate::joker::joker_warning;
pub use crate::robin::robin_exclamation;
