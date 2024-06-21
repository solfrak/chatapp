
use std::{env, io, thread};
use std::env::Args;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main()
{
    let mode = get_mode_from_args(env::args());
    match mode {
        ApplicationType::CLIENT(addr) => connect(&addr),
        ApplicationType::HOST => listen()
    }
}

enum ApplicationType{
    CLIENT(String), HOST
}

fn get_mode_from_args(args: Args) -> ApplicationType
{
    let args: Vec<String> = args.collect();
    if let Some(arg) = args.get(1)
    {
        return match arg.as_str() {
            "client" => ApplicationType::CLIENT(args.get(2).unwrap().clone()),
            "host" => ApplicationType::HOST,
            _ => ApplicationType::HOST
        }
    }
    ApplicationType::HOST
}


fn listen()
{
    let listener = TcpListener::bind("0.0.0.0:3000");
    match listener {
        Ok(listener) => {
            match listener.accept()
            {
                Ok((stream, _)) => Client::new(stream).run(),
                Err(e) => eprintln!("Error when accepting connection: {}", e)
            }

        }
        Err(e) => eprintln!("Error when binding: {}", e)
    }
}

fn connect(addr: &str)
{
    match TcpStream::connect(addr) {
        Ok(stream) => Client::new(stream).run(),
        Err(e) => eprintln!("Error when connecting to host: {}", e)
    }
}

struct Client
{
    stream: TcpStream,
}

impl Client {
    fn new(stream: TcpStream) -> Self
    {
        Client{stream}
    }

    fn run(self)
    {
        let mut reader_stream = self.stream.try_clone().unwrap();
        let mut writer_stream = self.stream.try_clone().unwrap();

        thread::spawn(move || {
            loop {
                let mut buffer = [0; 1024];
                match reader_stream.read(&mut buffer)
                {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            println!("Connection closed by other client");
                            break;
                        }
                        let received = &buffer[..bytes_read];
                        println!("Received: {}", String::from_utf8_lossy(received));
                    }
                    Err(_) => break
                }
            }
        });
        thread::spawn(move || {
            loop {
                let mut input: String = String::new();
                let _ = io::stdin().read_line(&mut input);
                match writer_stream.write_all(input.as_bytes())
                {
                    Err(_) => break,
                    _ => {}
                }
            }
        });

        loop {
            if !Client::is_connection_active(&self.stream) {
                println!("Closing application");
                std::process::exit(0);
            }
        }
    }

    fn is_connection_active(stream: &TcpStream) -> bool
    {
        let mut buffer = [0; 1];
        match stream.peek(&mut buffer)
        {
            Ok(t) => {
                if t == 0 {
                    return false;
                }
                true
            },
            Err(_) => false
        }
    }
}

