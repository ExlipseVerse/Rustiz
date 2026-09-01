mod store;
mod command;

use command::Command;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{thread};

use std::sync::{Arc, Mutex};

use crate::store::Store;

fn handle_command(stream: &mut TcpStream, command: Command, shared_store: Arc<Mutex<Store>>) {
    let mut store = shared_store.lock().unwrap();

    //covert the command into a str and match
    match command {
        Command::Get(key) => {
            match store.get(&key) {
                Some(value) => {
                    let response = format!("Value: {}\n", value);
                    // println!("{}", response);
                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        eprintln!("Failed to write to client: {}", e);
                    }
                }

                None => {
                    if let Err(e) =
                        stream.write_all(b"ERROR: Key not found\n")
                    {
                        eprintln!("Failed to write to client: {}", e);
                    }
                }
            }
        }

        Command::Set(key, value) => {
            store.set(&key, &value);
            if let Err(e) = stream.write_all(b"OK\n") {
                eprintln!("Failed to write to client: {}", e);
            }
        }

        Command::Delete(key) => {
            match store.delete(&key) {
                Some(_) => {
                    // let response = format!("{} deleted successfully\n", val);
                    if let Err(e) = stream.write_all(b"OK\n") {
                        eprintln!("Failed to write to client: {}", e);
                    }
                }

                None => {
                    if let Err(e) =
                        stream.write_all(b"ERROR: Key not found\n")
                    {
                        eprintln!("Failed to write to client: {}", e);
                    }
                }
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, shared_store: Arc<Mutex<Store>>) {
    println!("connected");

    let mut buffer = Vec::new();
    let mut temp = [0; 1024];

    // let read = stream.read(&mut buffer).unwrap();
    // println!("received {} bytes", read);
    // println!("{:?}", &buffer[..read]);
    loop {
        let read = stream.read(&mut temp); // read the input from client
        match read {
            //if read 0 break
            Ok(0) => { 
                println!("Client disconneted");
                break;
            },

            // actually read n's
            Ok(n) => {
                let recieved = &temp[..n];
                buffer.extend_from_slice(recieved);
                println!("{:?}", recieved);

                while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                    let cmd_bytes = buffer.drain(..=pos).collect::<Vec<_>>();
                    let input = String::from_utf8_lossy(&cmd_bytes);

                    let command = command::parse(&input);
                    match command {
                        Ok(command) => {
                            let store_clone = Arc::clone(&shared_store);
                            handle_command(&mut stream, command, store_clone);
                        }

                        Err(e) => {
                            let response = format!("ERROR: {}\n", e);

                            if let Err(write_error) = stream.write_all(response.as_bytes()) {
                                eprintln!("Failed to write error: {}", write_error);
                            }
                        }
                    }

                    // let parts: Vec<&str> = command.split_whitespace().collect();
                    
                }
                // if let Err(e) = stream.write_all(b"successfull!") {
                //     eprint!("failed to write to client: {}", e);
                //     break;
                // }

                // let msg = String::from_utf8_lossy(recieved); // convert the byte into text using utf-8 encoding or whatever
                // let trimmed = msg.trim();
                // if trimmed.is_empty() { continue;}

                // let parts: Vec<&str> = msg.split_whitespace().collect();
                // print!("{:?}", parts);
                // println!("\nconverted text: {}", msg);

                
            }

            //error otherwise
            Err(e) => {
                eprint!("Error parsing {}", e);
            }
        }
    }

   
}

fn main()-> std::io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:8000")?;
    println!("listning on 123.0.0.1:8000");

    let store = Arc::new(Mutex::new(Store::new())); // create the store thread safe 

    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                let store_clone = Arc::clone(&store); //clone per thread 
                thread::spawn(move || handle_client(stream, store_clone));
            }

            Err(e) => {
                eprint!("error: {}",e);
            }
        }
    }

    Ok(())
}