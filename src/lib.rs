//! Rust futures, streams, and sinks for ØMQ sockets.
extern crate futures;
extern crate zmq;

mod recv;
mod send;

use std::io;

use zmq::{Message, Sendable};

pub mod future {
    //! Futures for ØMQ sockets.
    pub use recv::*;
    pub use send::*;
}


/// Receives simple and multipart `Message`s.
pub trait MessageRecv {
    fn recv(&self, &mut Message, i32) -> io::Result<()>;
    fn recv_multipart(&self, i32) -> io::Result<Vec<Vec<u8>>>;
}

/// Sends simple and multipart `Message`s.
pub trait MessageSend {
    fn send<T>(&self, T, i32) -> io::Result<()>
        where
            T: Sendable;
    fn send_multipart<I, T>(&self, I, i32) -> io::Result<()>
        where
            I: IntoIterator<Item = T>,
            T: Into<Message>;
}
