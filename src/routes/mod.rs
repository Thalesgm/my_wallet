//! src/routes/mod.rs
mod register;
mod deposit;
mod transfer;
mod manifest;

pub use register::*;
pub use deposit::*;
pub use transfer::*;
pub use manifest::*;