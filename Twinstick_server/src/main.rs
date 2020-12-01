use std::io::prelude::*;
use std::thread;
use std::net::{UdpSocket, TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, ErrorKind};
use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use std::sync::Arc;
use std::time;
use std::str;

use twinstick_logic::*;

const BUFFER_SIZE: usize = 512;

#[macro_use]
pub extern crate serde_derive;
pub extern crate bincode;

pub use bincode::{deserialize, serialize};

pub struct Server {
  udp: UdpSocket,
  clients: Vec<SocketAddr>,
  client_last_connection: Vec<time::Instant>,
  game: TwinstickGame,
}

impl Server {
  pub fn new(ip: &str) -> Server {
    println!("listening on udp port {}", ip);
    let mut udp = UdpSocket::bind(ip).unwrap();
    udp.set_nonblocking(true).unwrap();
    
    Server {
      udp,
      clients: Vec::new(),
      client_last_connection: Vec::new(),
      game: TwinstickGame::new(),
    }
  }
  
  pub fn game(&self) -> &TwinstickGame {
    &self.game
  }
  
  pub fn add_player(&mut self, src_addr: SocketAddr) {
    let index = self.clients.len();
    self.clients.push(src_addr);
    self.client_last_connection.push(time::Instant::now());
    self.game.add_player();
    self.send_data_to_client(src_addr, &DataType::AddPlayer(self.game.players()[index].clone()).serialise());
    self.send_data_to_client(src_addr, &DataType::PlayerNum(index).serialise());
  }
  
  pub fn remove_player(&mut self, src_addr: SocketAddr) {
    match self.clients.binary_search(&src_addr) {
      Ok(i) => {
        println!("Removing client: {}", src_addr);
        self.clients.remove(i);
        self.client_last_connection.remove(i);
        self.game.remove_player(i);
        self.send_data_to_client(src_addr, &DataType::RemovePlayer(i).serialise());
      },
      Err(e) => {
        println!("Error: {}", e);
      }
    }
  }
  
  pub fn send_data_to_clients(&mut self, buffer: &[u8]) {
    if self.clients.len() == 0 {
      return;
    }
    
    for i in 0..self.clients.len() {
      self.udp.send_to(&buffer, self.clients[i]).unwrap();
      self.send_data_to_client(self.clients[i], &DataType::Player(self.game.players()[i].clone(), i).serialise());
      self.send_data_to_client(self.clients[i], &DataType::PlayerNum(i).serialise());
    }
  }
  
  pub fn send_data_to_client(&mut self, addr: SocketAddr, buffer: &[u8]) {
    self.udp.send_to(&buffer, addr).unwrap();
  }
  
  pub fn listen(&mut self) {
    let mut buffer = [0; BUFFER_SIZE];
    
    match self.udp.recv_from(&mut buffer) {
      Ok((number_of_bytes, src_addr)) => {
        let filled_buf = &mut buffer[..number_of_bytes];
        
        if !self.clients.contains(&src_addr) {
          println!("New client connected: {}", src_addr);
          self.add_player(src_addr);
          
         // println!("Data from {}: {:?}", src_addr, filled_buf);
         // self.send_data_to_client(src_addr, &DataType::PlayerNum(self.clients.len()-1).serialise());
        } else {
          match self.clients.binary_search(&src_addr) {
            Ok(i) => {
              self.client_last_connection[i] = time::Instant::now();
            },
            _ => {}
          }
          match DataType::deserialise(filled_buf) {
            Some(data_type) => {
              match data_type {
                DataType::Player(p, idx) => {
                  self.game.set_player(idx, p);
                },
                DataType::Exit => {
                  self.remove_player(src_addr);
                },
                _ => {
                  
                },
              }
            },
            None => {
              
            }
          }
        }
      },
      Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
        // wait until network socket is ready, typically implemented
        // via platform-specific APIs such as epoll or IOCP
        //wait_for_fd();
        for i in (0..self.clients.len()).rev() {
          if self.client_last_connection[i].elapsed() > time::Duration::from_secs(5) {
            self.remove_player(self.clients[i]);
          }
        }
      },
      Err(e) => panic!("encountered IO error: {}", e),
    }
  }
}

const FPS_60: f64 = 1.0/60.0;

fn main() {
  let mut server = Server::new("0.0.0.0:8008");
  
  let mut delta_time: f64 = 0.0;
  let mut last_time = time::Instant::now();
  
  let mut tick = 0.0;
  
  loop {
    delta_time = last_time.elapsed().subsec_nanos() as f64 / 1000000000.0 as f64;
    last_time = time::Instant::now();
    
    server.listen();
    
    tick += delta_time;
    
    if tick >= FPS_60 {
      tick = 0.0;
      let mut data = DataType::Game(server.game().clone()).serialise();
      if data.len() > BUFFER_SIZE {
        for p in 0..server.game().players().len() {
          let mut data = DataType::Player(server.game().players()[p].clone(), p).serialise();
          if data.len() > BUFFER_SIZE {
            panic!("Buffer not large neough to send player information!");
          }
          
          server.send_data_to_clients(&data);
        }
      } else {
        server.send_data_to_clients(&data);
      }
    }
  }
}
