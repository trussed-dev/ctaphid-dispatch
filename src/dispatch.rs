use crate::app::App;
use crate::types::{Command, Error, InterchangeResponse, Message, Responder};

pub struct Dispatch {
    responder: Responder<'static>,
}

impl Dispatch {
    pub fn new(responder: Responder<'static>) -> Dispatch {
        Dispatch { responder }
    }

    fn find_app<'a, 'b>(
        command: Command,
        apps: &'a mut [&'b mut dyn App],
    ) -> Option<&'a mut &'b mut dyn App> {
        apps.iter_mut()
            .find(|app| app.commands().contains(&command))
    }

    // // Using helper here to take potentially large stack burden off of call chain to application.
    // #[inline(never)]
    // fn reply_with_request_buffer(&mut self){
    //     let (_command, message) = self.responder.take_request().unwrap();
    //     let message = message.clone();
    //     self.responder.respond(&Ok(message)).expect("responder failed");
    // }

    // Using helper here to take potentially large stack burden off of call chain to application.
    #[inline(never)]
    fn reply_with_error(&mut self, error: Error) {
        self.reply_or_cancel(InterchangeResponse(Err(error)))
    }

    fn reply_or_cancel(&mut self, response: InterchangeResponse) {
        if self.responder.respond(response).is_ok() {
            return;
        }

        if self.responder.acknowledge_cancel().is_err() {
            panic!("Unexpected state: {:?}", self.responder.state());
        }
    }
    fn send_reply_or_cancel(&mut self) {
        if self.responder.send_response().is_ok() {
            return;
        }

        if self.responder.acknowledge_cancel().is_err() {
            panic!("Unexpected state: {:?}", self.responder.state());
        }
    }

    #[inline(never)]
    fn call_app(&mut self, app: &mut dyn App, command: Command, request: &Message) {
        let response_buffer = self.responder.response_mut().unwrap().0.as_mut().unwrap();

        if let Err(error) = app.call(command, request, response_buffer) {
            self.reply_with_error(error);
        } else {
            self.send_reply_or_cancel()
        }
    }

    #[inline(never)]
    pub fn poll<'a>(&mut self, apps: &mut [&'a mut dyn App]) -> bool {
        info!("ctaphid sees state: {:?}", self.responder.state());
        let maybe_request = self.responder.take_request();
        if let Some((command, message)) = maybe_request {
            // info_now!("cmd: {}", u8::from(command));
            // info_now!("cmd: {:?}", command);

            if let Some(app) = Self::find_app(command, apps) {
                // match app.call(command, self.responder.response_mut().unwrap()) {
                self.call_app(*app, command, &message);
            } else {
                self.reply_with_error(Error::InvalidCommand);
            }
            self.responder.state() == interchange::State::Responded
        } else {
            match self.responder.state() {
                interchange::State::Canceled => self.responder.acknowledge_cancel().is_ok(),
                interchange::State::Responded => true,
                _ => false,
            }
        }
    }
}
