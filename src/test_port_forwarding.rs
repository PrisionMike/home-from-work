use std::net::TcpListener;
// use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4114").unwrap();
    // let mut buffer = [0; 1024];

    println!("Psycho");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
        // let mut buffer = [0; 1024];

        // stream.read(&mut buffer).unwrap();

        // println!("{}",buffer);

        // let jai = "Jai Shree Ram";

        // stream.write(jai.as_bytes()).unwrap();
        // stream.flush().unwrap();
    }
}