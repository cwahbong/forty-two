use types::Event;

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use serde_json;
use std::io;
use tokio_core;

pub struct EventCodec;

impl tokio_core::io::Codec for EventCodec {
    type In = Event;
    type Out = Event;

    fn decode(&mut self, buf: &mut tokio_core::io::EasyBuf) -> io::Result<Option<Self::In>> {
        trace!("Decode");
        let (string_msg_size, position) = {
            let mut cursor = io::Cursor::new(buf.as_slice());
            let read_u64 = cursor.read_u64::<NetworkEndian>();
            if read_u64.is_err() {
                return Ok(None);
            }
            (read_u64.unwrap() as usize, cursor.position() as usize)
        };
        debug!("Expected size: {}, buflen: {}", string_msg_size, buf.len());
        if buf.len() < position + string_msg_size {
            Ok(None)
        } else {
            buf.drain_to(position);
            let msg_buf = buf.drain_to(string_msg_size);
            match serde_json::from_slice::<Event>(msg_buf.as_slice()) {
                Ok(event) => Ok(Some(event)),
                Err(error) => {
                    warn!("Json from slice error: {}", error);
                    Err(io::Error::new(io::ErrorKind::Other, "invalid json"))
                }
            }
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        trace!("Encode");
        let string_msg = serde_json::to_string(&msg).unwrap();
        debug!("Encode msg size: {}, buf size: {}", string_msg.len(), buf.len());
        buf.write_u64::<NetworkEndian>(string_msg.len() as u64)?;
        buf.extend(string_msg.as_bytes());
        debug!("Encoded buf size: {}", buf.len());
        Ok(())
    }
}
