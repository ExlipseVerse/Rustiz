mod store;


use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

use std::sync::{Arc, Mutex};

use crate::store::Store;

fn handle_command(stream: &mut TcpStream, parts: &[&str], shared_store: Arc<Mutex<Store>>) {
    let command_upper = parts[0].to_uppercase(); // we get the first word which is probably the commadn and we make it into uppercase sp get -> GET
    let mut store = shared_store.lock().unwrap();

    //covert the command into a str and match
    match command_upper.as_str() {
        "GET" => {
            if let Some(key) = parts.get(1) {
                // let value = store.get(key);
                // // println!("{:?}", value);
                // stream.write_all(b{value});
                match store.get(key) {
                    Some(value) => {
                        let response = format!("Value: {}\n", value);
                        if let Err(e) = stream.write_all(response.as_bytes()) {
                            eprintln!("Failed to write to client: {}", e);
                        }
                    }

                    None => {
                        let _ = stream.write_all(b"ERROR: Key not found\n");
                    }
                }

            } else {
                let _ = stream.write_all(b"ERROR: Please provide a key!\n");
            }
        },

        "SET" => {
            if let (Some(key), Some(value)) = (parts.get(1), parts.get(2)) {
                store.set(key, value);
                 let _ = stream.write_all(b"done\n");
            } else {
                let _ = stream.write_all(b"ERROR: Please provide a key/value !\n");
            }
            
        }

        "DELETE" => {
            if let Some(key) = parts.get(1) {
                let value = store.delete(&key);
                match value {
                    Some(val) => {
                        let response = format!("{} deleted successfully\n", val);
                        stream.write_all(response.as_bytes()).unwrap();
                    }

                    None => {
                        stream.write_all(b"Key not found\n").unwrap();
                    }
                }
            } else {
                let _ = stream.write_all(b"ERROR: Please provide a key!\n");
            }
        }


        _ => {
            let _ = stream.write_all(b"Command not recogonized\n");
        }
    }
}

fn handle_client(mut stream: TcpStream, shared_store: Arc<Mutex<Store>>) {
    println!("connected");

    let mut buffer = [0; 1024];

    // let read = stream.read(&mut buffer).unwrap();
    // println!("received {} bytes", read);
    // println!("{:?}", &buffer[..read]);
    loop {
        let read = stream.read(&mut buffer); // read the input from client
        match read {
            //if read 0 break
            Ok(0) => { 
                println!("Client disconneted");
                break;
            },

            // actually read n's
            Ok(n) => {
                let recieved = &buffer[..n];
                println!("{:?}", recieved);

                // if let Err(e) = stream.write_all(b"successfull!") {
                //     eprint!("failed to write to client: {}", e);
                //     break;
                // }

                let msg = String::from_utf8_lossy(recieved); // convert the byte into text using utf-8 encoding or whatever
                // let trimmed = msg.trim();
                // if trimmed.is_empty() { continue;}

                let parts: Vec<&str> = msg.split_whitespace().collect();
                // print!("{:?}", parts);
                // println!("\nconverted text: {}", msg);

                let store_clone = Arc::clone(&shared_store);
                handle_command(&mut stream, &parts, store_clone);
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