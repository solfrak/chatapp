use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::{io, thread};
use std::time::Duration;
use serde::{Deserialize, Serialize};


struct Server
{
    tcp_listener: TcpListener,
    request_handler: Option<Box<dyn Fn(TcpStream)>>,
}

impl Server {
    fn new(port: u16) -> io::Result<Self> {
        let tcp_listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        Ok(Server{tcp_listener, request_handler: None })
    }

    fn set_handler<F>(&mut self, handler: F) where F: Fn(TcpStream) + 'static {
        self.request_handler = Some(Box::new(handler));
    }

    fn run(self) {
        for stream in self.tcp_listener.incoming() {
           match stream {
               Ok(stream) => {
                   if let Some(ref handler) = self.request_handler {
                       handler(stream);
                   } else {
                       println!("No request handler set");
                   }
               }
               Err(e) => {
                   eprintln!("Error when receiving request: {}", e);
               }
           }
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    name: String,
    age: u8,
}



fn my_handler(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let buffer : Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{}", buffer[0]);
}

fn main() -> io::Result<()> {
    let mut server = Server::new(4000)?;
    server.set_handler(my_handler);
    server.run();

    Ok(())
}




// fn main() -> std::io::Result<()> {
//     // Shared buffer with Arc, Mutex, and Condvar
//     let buffer: Arc<(Mutex<Vec<TcpStream>>, Condvar)> = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
//     let producer_ref = Arc::clone(&buffer);
//     let consumer_ref = Arc::clone(&buffer);
//
//     // Spawn a consumer thread to handle requests from the buffer
//     thread::spawn(move || {
//         loop {
//             let (lock, cvar) = &*consumer_ref;
//             let mut data = lock.lock().unwrap();
//
//             // Wait until there's data in the buffer
//             while data.is_empty() {
//                 data = cvar.wait(data).unwrap();
//             }
//
//             // Take the first TcpStream from the buffer
//             let stream = data.remove(0);
//             drop(data); // Unlock the Mutex early
//
//             // Handle the request
//             if let Err(e) = handle_request(stream) {
//                 eprintln!("Error handling request: {}", e);
//             }
//         }
//     });
//
//     // Main thread that listens for incoming connections
//     let listener = TcpListener::bind("127.0.0.1:3000")?;
//     println!("Listening on: 127.0.0.1:3000");
//
//     for stream in listener.incoming() {
//         match stream {
//             Ok(mut stream) => {
//                 println!("Putting some stream in the buffer");
//
//
//                 let response = "HTTP/1.1 200 OK\r\n\r\n";
//                 stream.write_all(response.as_bytes())?;
//                 stream.flush()?;
//                 let mut data = producer_ref.0.lock().unwrap();
//
//                 data.push(stream);
//                 producer_ref.1.notify_one(); // Notify the consumer thread
//             }
//             Err(e) => {
//                 eprintln!("Error accepting connection: {}", e);
//             }
//         }
//     }
//
//     Ok(())
//
//
// }
//
// fn handle_request(mut stream: TcpStream) -> std::io::Result<()> {
//     println!("Handling request from {:?}", stream.peer_addr()?);
//
//     thread::sleep(Duration::new(5, 0));
//     let buf_reader = BufReader::new(&mut stream);
//     let _ : Vec<_> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();
//
//     // Craft a simple HTTP response
//     // let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 12\r\n\r\nHello World!";
//     let response = "HTTP/1.1 200 OK\r\n\r\n";
//     stream.write_all(response.as_bytes())?;
//     stream.flush()?;
//
//     Ok(())
// }
