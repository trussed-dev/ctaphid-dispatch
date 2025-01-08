use ctaphid_app::{Command, Error};
use heapless_bytes::Bytes;

pub const MESSAGE_SIZE: usize = 7609;

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
