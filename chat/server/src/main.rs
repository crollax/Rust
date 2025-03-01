use std::io::{ErrorKind, Read, Write};
// Create server and listen on a port
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn sleep() {
  thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {
  // Instantiate server
  let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
  server.set_nonblocking(true).expect("failed to initalize non-blocking");

  let mut clients = vec![];
  // Instantiate receiver
    // tx == transmitter, rx == receiver
  let (tx, rx) = mpsc::channel::<String>();
  loop {
    // server.accept() allows connections to server
    // addr == socket address
    if let Ok((mut socket, addr)) = server.accept() {
      println!("Client {} connected", addr);

      // clone socket and push into clients vector / thread
      let tx = tx.clone();
      clients.push(socket.try_clone().expect("failed to clone client"));

      thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];

        match socket.read_exact(&mut buff) {
          Ok(_) => {
            let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
            let msg = String::from_utf8(msg).expect("Invalid utf8 message");

            println!("{}: {:?}", addr, msg);
            tx.send(msg).expect("failed to send msg to rx");
          },
          // check error inside of error
          Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
          Err(_) => {
            println!("closing connection with: {}", addr);
            break;
          }
        }

        sleep();

      });
    }

    // what happens when server recieves a message
      // functional programming
        // convert msg into bytes
        // resize buffer to msg size
        // write/map buffer into client
        // collect into a vector
    if let Ok(msg) = rx.try_recv() {
      clients = clients.into_iter().filter_map(|mut client| {
        let mut buff = msg.clone().into_bytes();
        buff.resize(MSG_SIZE, 0);

        client.write_all(&buff).map(|_| client).ok()
      }).collect::<Vec<_>>();
    }

    sleep();
  }
}
