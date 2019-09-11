//! A fake stream for testing network applications backed by buffers.
#![warn(missing_docs)]

extern crate futures;
extern crate tokio;

use std::task::Context;
use std::pin::Pin;
use std::io;
use std::io::{Cursor, Read, Write};
use futures::Poll;
use tokio::io::{AsyncRead, AsyncWrite};

/// A fake stream for testing network applications backed by buffers.
#[derive(Clone, Debug)]
pub struct MockStream {
    written: Cursor<Vec<u8>>,
    received: Cursor<Vec<u8>>,
}

impl MockStream {
    /// Creates a new mock stream with nothing to read.
    pub fn empty() -> MockStream {
        MockStream::new(&[])
    }

    /// Creates a new mock stream with the specified bytes to read.
    pub fn new(initial: &[u8]) -> MockStream {
        MockStream {
            written: Cursor::new(vec![]),
            received: Cursor::new(initial.to_owned()),
        }
    }

    /// Gets a slice of bytes representing the data that has been written.
    pub fn written(&self) -> &[u8] {
        self.written.get_ref()
    }

    /// Gets a slice of bytes representing the data that has been received.
    pub fn received(&self) -> &[u8] {
        self.received.get_ref()
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.received.read(buf)
    }
}

impl AsyncRead for MockStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(self.read(buf))
    }
}

impl Write for MockStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.written.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.written.flush()
    }
}

impl AsyncWrite for MockStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        Poll::Ready(self.write(buf))
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>
    ) -> Poll<Result<(), io::Error>> {
        Poll::Ready(self.flush())
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>
    ) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}


#[cfg(test)]
mod tests;
