#![no_std]

use heapless::Vec;
use trussed_core::InterruptFlag;

mod command;

pub use command::{Command, VendorCommand};

/// trait interface for a CTAPHID application.
/// The application chooses which commands to register to, and will be called upon
/// when the commands are received in the CTAPHID layer.  Only one application can be registered to a particular command.
pub trait App<'interrupt, const N: usize> {
    /// Get access to the app interrupter
    fn interrupt(&self) -> Option<&'interrupt InterruptFlag> {
        None
    }

    /// Define which CTAPHID commands to register to.
    fn commands(&self) -> &'static [Command];

    /// Application is called here when one of it's register commands occurs.
    /// Application must put response in @message, or decide to return an error.
    ///
    /// The response is pre-cleared.
    fn call(
        &mut self,
        command: Command,
        request: &[u8],
        response: &mut Vec<u8, N>,
    ) -> Result<(), Error>;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    NoResponse,
    InvalidCommand,
    InvalidLength,
}
