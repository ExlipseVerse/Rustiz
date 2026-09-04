
use std::convert::TryInto;
use std::net::TcpStream;
use std::io::Write;

pub enum Command {
	Get(String),
	Set(String, String),
	Delete(String),
}

pub enum Response {
	Value(String),
	Ok,
	NotFound,
	Error(String),
}

#[warn(dead_code)]
pub fn parse(input: &str) -> Result<Command, String> {
	let parts: Vec<&str> = input.split_whitespace().collect();

	let cmd = match parts.first() {
		Some(command) => command.to_uppercase(),
		None => return  Err("Empty Command".to_string())
	};

	match cmd.as_str() {
		"GET" => {
			if let Some(key) = parts.get(1) {
				Ok(Command::Get(key.to_string()))
			} else {
				Err("GET requires a key".to_string())
			}
			
		}

		"SET" => {
			if let (Some(key), Some(value)) = (parts.get(1), parts.get(2)) {
				Ok(Command::Set(key.to_string(), value.to_string()))
			} else {
				Err("SET requires a key and a value".to_string())
			}
			
		}

		"DELETE" => {
			if let Some(key) = parts.get(1) { 
				Ok(Command::Delete(key.to_string()))
			} else {
				Err("DELETE requires a key".to_string())
			}
		}

		_ => Err("Invalid command".to_string())
	}
}

pub fn write_response(stream: &mut TcpStream,response: Response) -> std::io::Result<()> {
	match response {
		Response::Value(value) => {
			let response = format!("Value: {}\n", value);
			stream.write_all(response.as_bytes())?;
		}

		Response::Ok => {
			stream.write_all(b"OK\n")?;
		}

		Response::NotFound => {
			stream.write_all(b"ERROR: Key not found\n")?;
		}

		Response::Error(e) => {
			let response = format!("ERROR: {}\n", e);
			stream.write_all(response.as_bytes())?;
		}
	}

	Ok(())
}


pub fn decode_input(buffer: &mut Vec<u8>) -> Result<Option<Command>, String> {
	if  buffer.is_empty() {
		return Ok(None);
	}

	let cmd_id = buffer[0];

	match cmd_id {
		1 => {
			let mut remaining = &buffer[1..];
			if remaining.len() < 4 {
				return Ok(None);
			}

			let (key_len_bytes, rest) = remaining.split_at(4);
			let key_len = u32::from_be_bytes(key_len_bytes.try_into().unwrap()) as usize;
			remaining = rest;

			if remaining.len() < key_len {
				return Ok(None);
			}

			let (key_bytes, rest) = remaining.split_at(key_len);
			let key = String::from_utf8(key_bytes.to_vec()).map_err(|_| "Key is not valid UTF-8".to_string())?;
			remaining = rest;

			if remaining.len() < 4 {
				return Ok(None);
			}


			let (value_len_bytes, rest) = remaining.split_at(4);
			let value_len = u32::from_be_bytes(value_len_bytes.try_into().unwrap()) as usize;
			remaining = rest;

			if remaining.len() < value_len {
				return Ok(None);
			}

			let (value_bytes, _) = remaining.split_at(value_len);
			let value = String::from_utf8(value_bytes.to_vec()).map_err(|_| "Value is not valid UTF-8".to_string())?;

			let total_consumed = 1 + 4 + key_len + 4 + value_len;
			buffer.drain(..total_consumed);
			Ok(Some(Command::Set(key,value)))
		}

		2 => {
			let mut remaining = &buffer[1..];
			if remaining.len() < 4 {
				return Ok(None);
			}

			let (key_len_bytes, rest) = remaining.split_at(4);
			let key_len = u32::from_be_bytes(key_len_bytes.try_into().unwrap()) as usize;
			remaining = rest;

			if remaining.len() < key_len {
				return Ok(None);
			}

			let (key_bytes, _) = remaining.split_at(key_len);
			let key = String::from_utf8(key_bytes.to_vec()).map_err(|_| "Key is not valid UTF-8".to_string())?;
			
			let total_consumed = 1 + 4 + key_len;
			buffer.drain(..total_consumed);

			Ok(Some(Command::Get(key)))
		}

		3 => {
			let mut remaining = &buffer[1..];
			if remaining.len() < 4 {
				return Ok(None);
			}

			let (key_len_bytes, rest) = remaining.split_at(4);
			let key_len = u32::from_be_bytes(key_len_bytes.try_into().unwrap()) as usize;
			remaining = rest;

			if remaining.len() < key_len {
				return Ok(None);
			}

			let (key_bytes, _) = remaining.split_at(key_len);
			let key = String::from_utf8(key_bytes.to_vec()).map_err(|_| "Key is not valid UTF-8".to_string())?;
			
			let total_consumed = 1 + 4 + key_len;
			buffer.drain(..total_consumed);

			Ok(Some(Command::Delete(key)))
		}

		_ => Err(format!("Unknown command ID: {}", cmd_id)),
	}
}