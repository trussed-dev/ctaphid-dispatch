use core::sync::atomic::Ordering;

use crate::types::{InterchangeResponse, Message, Responder};

use ctaphid_app::{App, Command, Error};
use ref_swap::OptionRefSwap;
use trussed_core::InterruptFlag;

pub struct Dispatch<'pipe, 'interrupt> {
    responder: Responder<'pipe>,
    interrupt: Option<&'interrupt OptionRefSwap<'interrupt, InterruptFlag>>,
}

impl<'pipe> Dispatch<'pipe, '_> {
    pub fn new(responder: Responder<'pipe>) -> Self {
        Dispatch {
            responder,
            interrupt: None,
        }
    }
}

impl<'pipe, 'interrupt> Dispatch<'pipe, 'interrupt> {
    pub fn with_interrupt(
        responder: Responder<'pipe>,
        interrupt: Option<&'interrupt OptionRefSwap<'interrupt, InterruptFlag>>,
    ) -> Self {
        Dispatch {
            responder,
            interrupt,
        }
    }

    fn find_app<'a, 'b>(
        command: Command,
        apps: &'a mut [&'b mut dyn App<'interrupt>],
    ) -> Option<&'a mut &'b mut dyn App<'interrupt>> {
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
    fn call_app(&mut self, app: &mut dyn App<'interrupt>, command: Command, request: &Message) {
        let response_buffer = self
            .responder
            .response_mut()
            .expect("App calls should only happen when a respose can be constructed")
            .0
            .as_mut()
            .unwrap();

        // Cancellation is best-effort, and not relevant for actual synchronisation, so relaxed is used
        let res =
            if let (Some(app_interrupt), Some(interrupt_ptr)) = (app.interrupt(), self.interrupt) {
                app_interrupt.set_working();
                interrupt_ptr.store(Some(app_interrupt), Ordering::Relaxed);
                let res = app.call(command, request, response_buffer);
                app_interrupt.set_idle();
                interrupt_ptr.store(None, Ordering::Relaxed);
                res
            } else {
                app.call(command, request, response_buffer)
            };

        info_now!("Got res: {:?}", res);
        if let Err(error) = res {
            self.reply_with_error(error)
        } else {
            self.send_reply_or_cancel()
        }
    }

    #[inline(never)]
    pub fn poll(&mut self, apps: &mut [&mut dyn App<'interrupt>]) -> bool {
        // We could call take_request directly, but for some reason this doubles stack usage.
        let mut message_buffer = Message::new();
        if let Ok((command, message)) = self.responder.request() {
            // info_now!("cmd: {}", u8::from(command));
            // info_now!("cmd: {:?}", command);

            message_buffer.extend_from_slice(message).unwrap();

            if let Some(app) = Self::find_app(*command, apps) {
                // match app.call(command, self.responder.response_mut().unwrap()) {
                self.call_app(*app, *command, &message_buffer);
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
