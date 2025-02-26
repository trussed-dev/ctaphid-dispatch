use ctaphid_app::{Command, Error};
use heapless_bytes::Bytes;

// TODO: update ML-DSA to minimum necessary sizes
pub const MESSAGE_SIZE: usize = if cfg!(feature = "mldsa87") {
    20000
} else if cfg!(feature = "mldsa65") {
    15000
} else if cfg!(feature = "mldsa44") {
    10000
} else {
    7609
};

pub type Message = Bytes<MESSAGE_SIZE>;

/// Wrapper struct that implements [`Default`][] to be able to use [`response_mut`](interchange::Responder::response_mut)
pub struct InterchangeResponse(pub Result<Message, Error>);

impl Default for InterchangeResponse {
    fn default() -> Self {
        InterchangeResponse(Ok(Message::new()))
    }
}

impl From<Result<Message, Error>> for InterchangeResponse {
    fn from(value: Result<Message, Error>) -> Self {
        Self(value)
    }
}

impl From<InterchangeResponse> for Result<Message, Error> {
    fn from(value: InterchangeResponse) -> Self {
        value.0
    }
}

pub type Responder<'pipe> = interchange::Responder<'pipe, (Command, Message), InterchangeResponse>;
pub type Requester<'pipe> = interchange::Requester<'pipe, (Command, Message), InterchangeResponse>;
pub type Channel = interchange::Channel<(Command, Message), InterchangeResponse>;
