use std::io::{Read, Write};

use errors::{RecvError, SendError};

pub mod client;
pub mod custom_parser;
pub mod errors;
pub mod server;

fn send_string<Data, Writer>(data: Data, mut writer: Writer) -> Result<(), SendError>
where
    Data: AsRef<str>,
    Writer: Write,
{
    let data_bytes = data.as_ref().as_bytes();
    let len_bytes = (data_bytes.len() as u32).to_be_bytes();
    writer.write_all(&len_bytes)?;
    writer.write_all(data_bytes)?;
    Ok(())
}

fn recv_string<Reader>(mut reader: Reader) -> Result<String, RecvError>
where
    Reader: Read,
{
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let len = u32::from_be_bytes(len_buf);

    let mut buf = vec![0; len as usize];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}
