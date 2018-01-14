use std::io;
use futures::{Async, Future, Poll};

use {MessageSend, Message};

/// A Future that sends a `Message`.
pub struct SendMessage<'a, T: 'a> {
    socket: &'a T,
    message: Message,
}

impl<'a, T> SendMessage<'a, T>
where
    T: MessageSend + 'a,
{
    /// Create a new `SendMessage` future.
    pub fn new(socket: &'a T, message: Message) -> SendMessage<'a, T>
    {
        SendMessage { socket, message }
    }
}

impl<'a, T> Future for SendMessage<'a, T>
where
    T: MessageSend + 'a,
{
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.socket.send(&*self.message, 0) {
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    Ok(Async::NotReady)
                } else {
                    Err(e)
                }
            }
            Ok(_) => Ok(Async::Ready(())),
        }
    }
}

/// A Future that sends a multi-part `Message`.
pub struct SendMultipartMessage<'a, T: 'a> {
    socket: &'a T,
    messages: Vec<Vec<u8>>,
}

impl<'a, T>  SendMultipartMessage<'a, T>
where
    T: MessageSend + 'a,
{
    /// Create a new `SendMultipartMessage`.
    pub fn new<I, U>(socket: &'a T, iter: I) -> SendMultipartMessage<'a, T>
    where
        I: IntoIterator<Item = U>,
        U: Into<Vec<u8>>,
    {
        let messages: Vec<Vec<u8>> = iter.into_iter().map(|m| m.into()).collect();
        SendMultipartMessage { socket, messages }
    }
}

impl<'a, T> Future for SendMultipartMessage<'a, T>
where
    T: MessageSend + 'a,
{
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.socket.send_multipart(&self.messages, 0) {
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    Ok(Async::NotReady)
                } else {
                    Err(e)
                }
            }
            Ok(_) => Ok(Async::Ready(())),
        }
    }
}
