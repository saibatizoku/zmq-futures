//! Rust futures, streams, and sinks for ØMQ sockets.
extern crate futures;
#[cfg(test)] extern crate futures_cpupool;
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

pub mod stream {
    //! Streams for ØMQ sockets.
    pub use listen::*;
}


/// Receives simple and multipart `Message`s.
pub trait MessageRecv {
    /// Return true if there are more frames of a multipart message to receive.
    fn get_rcvmore(&self) -> io::Result<bool>;

    /// Receive a message into a `Message`. The length passed to `zmq_msg_recv` is the length
    /// of the buffer.
    fn recv(&self, &mut Message, i32) -> io::Result<()>;

    /// Receive bytes into a slice. The length passed to `zmq_recv` is the length of the slice. The
    /// return value is the number of bytes in the message, which may be larger than the length of
    /// the slice, indicating truncation.
    fn recv_into(&self, &mut [u8], i32) -> io::Result<usize>;

    /// Receive a message into a fresh `Message`.
    fn recv_msg(&self, i32) -> io::Result<Message>;

    /// Receive a message as a byte vector.
    fn recv_bytes(&self, i32) -> io::Result<Vec<u8>>;

    /// Receive a `String` from the socket.
    ///
    /// If the received message is not valid UTF-8, it is returned as the original `Vec` in the `Err`
    /// part of the inner result.
    fn recv_string(&self, i32) -> io::Result<Result<String, Vec<u8>>>;

    /// Receive a multipart message from the socket.
    ///
    /// Note that this will allocate a new vector for each message part; for many applications it
    /// will be possible to process the different parts sequentially and reuse allocations that
    /// way.
    fn recv_multipart(&self, i32) -> io::Result<Vec<Vec<u8>>>;
}

/// Sends simple and multipart `Message`s.
pub trait MessageSend {
    /// Send a message.
    ///
    /// Due to the provided From implementations, this works for `&[u8]`, `Vec<u8>` and `&str`,
    /// as well as on `Message` itself.
    fn send<T>(&self, T, i32) -> io::Result<()>
        where
            T: Sendable;
    /// Sends a multipart-message.
    fn send_multipart<I, T>(&self, I, i32) -> io::Result<()>
        where
            I: IntoIterator<Item = T>,
            T: Into<Message>;
}

/// Listens for incoming messages.
pub trait Listen: MessageRecv + MessageSend {
    type Stream: futures::Stream;

    /// Return a stream of received messages.
    fn listen(&self) -> Self::Stream;
}
