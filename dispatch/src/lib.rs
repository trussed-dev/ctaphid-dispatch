//! # ctaphid-dispatch
//!
//! This library defines a concept of CTAPHID apps, which declare
//! CTAPHID commands, which are then dispatched to them.
//!
//! The intention is for non-FIDO authenticator apps to be able
//! to extend the CTAPHID interface with additional functionality.
//!
//! For instance, the Solo 2 management app.
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate delog;
generate_macros!();

pub mod dispatch;
pub mod types;

pub mod app {
    pub use crate::types::AppResult;
    pub use ctaphid_app::{App, Command, Error, Message};
}

pub mod command {
    pub use ctaphid_app::{Command, VendorCommand};
}
