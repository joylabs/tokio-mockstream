extern crate bytes;

use self::bytes::{BufMut, BytesMut};
use super::MockStream;
use std::{io, str};
use tokio::codec::{Decoder, Encoder};
use tokio::prelude::*;

#[tokio::test]
async fn writing_to_mockstream() {
    let mut stream = LineCodec::new().framed(MockStream::empty());
    stream
        .send("This is a test of the emergency broadcast system.".to_owned())
        .await
        .unwrap();
    let inner = stream.into_inner();
    assert!(inner.received().is_empty());
    let expected = b"This is a test of the emergency broadcast system.\n";
    assert_eq!(inner.written().to_owned(), expected.to_vec());
}

#[tokio::test]
async fn reading_from_mockstream() {
    let mut iter = LineCodec::new().framed(MockStream::new(b"Hello\nGoodbye\n"));
    let first = iter.next().await.unwrap();
    assert_eq!(&first.unwrap(), "Hello");
    let second = iter.next().await.unwrap();
    assert_eq!(&second.unwrap(), "Goodbye");
    assert!(iter.next().await.is_none());
}

/// A line codec taken from the Tokio examples.
struct LineCodec;

impl LineCodec {
    fn new() -> Self {
        LineCodec {}
    }
}

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        // Check to see if the frame contains a new line
        if let Some(n) = buf.as_ref().iter().position(|b| *b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(n);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            return match str::from_utf8(&line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
            };
        }

        Ok(None)
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        // Reserve enough space for the line
        buf.reserve(msg.len() + 1);

        buf.extend(msg.as_bytes());
        buf.put_u8(b'\n');

        Ok(())
    }
}
