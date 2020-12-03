use std::str;
use std::net::{UdpSocket, SocketAddr};


use std::io;
use std::time;

use twinstick_logic::*;

const BUFFER_SIZE: usize = 512;

pub struct TwinstickClient {
  udp: UdpSocket,
  server: String,
  disconnected: bool,
}

impl Drop for TwinstickClient {
  fn drop(&mut self) {
    self.disconnect();
  }
}

impl TwinstickClient {
  pub fn new(ip: &str) -> TwinstickClient {
    println!("Attempting to connect to server {}", ip);
    let addrs = [
   /*   SocketAddr::from(([127, 0, 0, 1], 8010)),
      SocketAddr::from(([127, 0, 0, 1], 8011)),
      SocketAddr::from(([127, 0, 0, 1], 8012)),
      SocketAddr::from(([127, 0, 0, 1], 8013)),
      SocketAddr::from(([127, 0, 0, 1], 8014)),
      SocketAddr::from(([127, 0, 0, 1], 8015)),
      SocketAddr::from(([127, 0, 0, 1], 8016)),
      SocketAddr::from(([127, 0, 0, 1], 8017)),*/
      SocketAddr::from(([0, 0, 0, 0], 8010)),
      SocketAddr::from(([0, 0, 0, 0], 8011)),
      SocketAddr::from(([0, 0, 0, 0], 8012)),
      SocketAddr::from(([0, 0, 0, 0], 8013)),
      SocketAddr::from(([0, 0, 0, 0], 8014)),
      SocketAddr::from(([0, 0, 0, 0], 8015)),
      SocketAddr::from(([0, 0, 0, 0], 8016)),
      SocketAddr::from(([0, 0, 0, 0], 8017)),
    ];
    let udp = UdpSocket::bind(&addrs[..]).unwrap();
    udp.set_nonblocking(true).unwrap();
    
    TwinstickClient {
      udp,
      server: ip.to_string(),
      disconnected: false,
    }
  }
  
  pub fn connect(&mut self) {
    self.udp.connect(self.server.clone()).is_ok();//.unwrap_or(panic!("Cannot connect to server: {}", self.server))
  }
  
  pub fn send_datatype(&mut self, data_type: DataType) {
    self.udp.send(&data_type.serialise());
  }
  
  pub fn send(&mut self) {
    let resposne = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    match self.udp.send(&resposne) {
      Ok(_) => {},
      Err(e) => {println!("{:?}",e);},
    }
  }
  
  pub fn recieve(&mut self) -> Option<DataType> {
    let mut buffer = [0; BUFFER_SIZE];
    
    match self.udp.recv_from(&mut buffer) {
      Ok((number_of_bytes, src_addr)) => {
        let filled_buf = &mut buffer[..number_of_bytes];
        return DataType::deserialise(filled_buf);
      },
      Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            // wait until network socket is ready, typically implemented
            // via platform-specific APIs such as epoll or IOCP
            //wait_for_fd();
      },
      Err(e) => panic!("encountered IO error: {}", e),
    }
    
    None
  }
  
  pub fn disconnect(&mut self) {
    if self.disconnected {
      return;
    }
    
    self.udp.send(&DataType::Exit.serialise()).unwrap();
    self.disconnected = true;
  }
}

fn main() {
  let mut c = TwinstickClient::new("127.0.0.1:8008");
  
  let mut delta_time: f64 = 0.0;
  let mut last_time = time::Instant::now();
  
  let mut tick = 0.0;
  
  c.connect();
  c.send();
  
  let mut counter = 0;
  
  loop {
    delta_time = last_time.elapsed().subsec_nanos() as f64 / 1000000000.0 as f64;
    last_time = time::Instant::now();
    tick += delta_time;
     
    if tick >= 1.0 {
      tick = 0.0;
      counter += 1;
      if counter > 12 {
        c.disconnect();
        break;
      }
    }
    
    c.recieve();
  }
}
