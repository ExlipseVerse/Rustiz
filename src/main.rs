mod store;
mod command;

use command::{Command, Response, decode_input};

use std::net::{TcpListener, TcpStream};
use std::io::{Read};
use std::{thread};

use std::sync::{Arc, Mutex};

use crate::command::write_response;
use crate::store::Store;

fn handle_command(command: Command, shared_store: Arc<Mutex<Store>>) -> Response {
    let mut store = shared_store.lock().unwrap();

    //covert the command into a str and match
    match command {
        Command::Get(key) => {
            match store.get(&key) {
                Some(value) => Response::Value(value.clone()),
                None => Response::NotFound
            }
        }

        Command::Set(key, value) => {
            store.set(&key, &value);
            Response::Ok
        }

        Command::Delete(key) => {
            match store.delete(&key) {
                Some(_) => Response::Ok,
                None => Response::NotFound
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

                loop {
                    match decode_input(&mut buffer) {
                        Ok(Some(cmd)) => {
                            let store_clone = Arc::clone(&shared_store);
                            let response = handle_command(cmd, store_clone);

                            if let Err(e) = write_response(&mut stream, response) {
                                eprintln!("Failed to write response: {}", e);
                                return;
                            }
                        }

                        Ok(None) => {
                            break;
                        }

                        Err(e) => {
                            eprintln!("Protocol error: {}", e);
                            let response = Response::Error(e);

                            if let Err(e) = write_response(&mut stream, response) {
                                eprintln!("Failed to write response: {}", e);
                                
                            }

                            return;
                        }
                    }
                }

                // while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                //     let cmd_bytes = buffer.drain(..=pos).collect::<Vec<_>>();
                //     let input = String::from_utf8_lossy(&cmd_bytes);

                //     let command = parse(&input);
                //     match command {
                //         Ok(command) => {
                //             
                //         }

                //         Err(e) => {
                //             let response = Response::Error(e);

                //             if let Err(e) = write_response(&mut stream, response) {
                //                 eprintln!("Failed to write response: {}", e);
                //                 break;
                //             }
                //         }
                //     }

                //     // let parts: Vec<&str> = command.split_whitespace().collect();
                    
                // }
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