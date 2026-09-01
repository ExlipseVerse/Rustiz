pub enum Command {
	Get(String),
	Set(String, String),
	Delete(String),
}

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