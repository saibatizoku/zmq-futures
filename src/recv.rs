//! Futures for receiving messages.
use std::io;
use futures::{Async, Future, Poll};

use {MessageRecv, Message};

/// A Future that receives a multi-part `Message` asynchronously.
/// This is returned by `Socket::recv_multipart`
pub struct ReceiveMultipartMessage<'a, T: 'a> {
    socket: &'a T,
}

impl<'a, T> ReceiveMultipartMessage<'a, T>
where
    T: MessageRecv + 'a,
{
    pub fn new(socket: &'a T) -> ReceiveMultipartMessage<'a, T> {
        ReceiveMultipartMessage { socket }
    }
}

impl<'a, T> Future for ReceiveMultipartMessage<'a, T>
where
    T: MessageRecv + 'a,
{
    type Item = Vec<Message>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.socket.recv_multipart(0) {
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    Ok(Async::NotReady)
                } else {
                    Err(e)
                }
            }
            Ok(msgs) => {
                let m_out = msgs.iter().map(|v| v.into()).collect::<Vec<Message>>();
                Ok(Async::Ready(m_out))
            }
        }
    }
}

/// A Future that receives a `Message` asynchronously. This is returned by `Socket::recv`
pub struct ReceiveMessage<'a, T: 'a> {
    socket: &'a T,
}

impl<'a, T> ReceiveMessage<'a, T>
where
    T: MessageRecv + 'a,
{
    pub fn new(socket: &'a T) -> ReceiveMessage<'a, T> {
        ReceiveMessage { socket }
    }
}

impl<'a, T> Future for ReceiveMessage<'a, T>
where
    T: MessageRecv + 'a,
{
    type Item = Message;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut msg = Message::new();
        match self.socket.recv(&mut msg, 0) {
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    Ok(Async::NotReady)
                } else {
                    Err(e)
                }
            }
            Ok(_) => Ok(Async::Ready(msg)),
        }
    }
}
