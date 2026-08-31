use std::collections::HashMap; // hashhhhmapppppppppppppp


//create the store 
pub struct Store {
	pub data: HashMap<String, String>
}


//add funcs to it
impl Store {

	//init func
	pub fn new() -> Self {
		println!("Created the store!");
		Store {
			data: HashMap::new(),
		}
		
	}

	//get func
	pub fn get(&self, key: &str) -> Option<&String> {
	   	self.data.get(key)
	}

	//set func
	pub fn set(&mut self, key: &str, value: &str) {
		self.data.insert(key.to_string(), value.to_string());
	}

	//delete func
	pub fn delete(&mut self, key: &str) -> Option<String> {
		self.data.remove(key)
	} 
}