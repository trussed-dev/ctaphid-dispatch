use ctaphid_app::{Command, Error};
use heapless_bytes::Bytes;

pub const DEFAULT_MESSAGE_SIZE: usize = 7609;

/// Wrapper struct that implements [`Default`][] to be able to use [`response_mut`](interchange::Responder::response_mut)
pub struct InterchangeResponse<const N: usize>(pub Result<Bytes<N>, Error>);

impl<const N: usize> Default for InterchangeResponse<N> {
    fn default() -> Self {
        InterchangeResponse(Ok(Default::default()))
    }
}

impl<const N: usize> From<Result<Bytes<N>, Error>> for InterchangeResponse<N> {
    fn from(value: Result<Bytes<N>, Error>) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<InterchangeResponse<N>> for Result<Bytes<N>, Error> {
    fn from(value: InterchangeResponse<N>) -> Self {
        value.0
    }
}

pub type Responder<'pipe, const N: usize> =
    interchange::Responder<'pipe, (Command, Bytes<N>), InterchangeResponse<N>>;
pub type Requester<'pipe, const N: usize> =
    interchange::Requester<'pipe, (Command, Bytes<N>), InterchangeResponse<N>>;
pub type Channel<const N: usize> =
    interchange::Channel<(Command, Bytes<N>), InterchangeResponse<N>>;
