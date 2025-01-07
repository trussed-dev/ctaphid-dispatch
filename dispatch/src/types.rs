use heapless_bytes::Bytes;

pub use ctaphid_app::Error;

// // 7609 bytes is max message size for ctaphid
// type U6144 = <heapless::consts::U4096 as core::ops::Add<heapless::consts::U2048>>::Output;
// type U7168 = <U6144 as core::ops::Add<heapless::consts::U1024>>::Output;
// pub type U7609 = <U7168 as core::ops::Add<heapless::consts::U441>>::Output;
// pub type U7609 = heapless::consts::U4096;

// TODO: find reasonable size
// pub type Message = heapless::Vec<u8, 3072>;
pub const MESSAGE_SIZE: usize = 7609;

pub type Message = Bytes<MESSAGE_SIZE>;
pub type AppResult = core::result::Result<(), Error>;
pub type ShortMessage = Bytes<1024>;

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

pub use crate::command::Command;

pub type Responder<'pipe> = interchange::Responder<'pipe, (Command, Message), InterchangeResponse>;
pub type Requester<'pipe> = interchange::Requester<'pipe, (Command, Message), InterchangeResponse>;
pub type Channel = interchange::Channel<(Command, Message), InterchangeResponse>;
