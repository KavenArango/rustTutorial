#![allow(non_snake_case)]
#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
  let mut name = String::new();
  let greating: &str = "Nice to meet you";
  
  println!("what is your name");

  io::stdin().read_line(&mut name)
    .expect("Didn't Receive Input");

  // name = "hello".to_string();
  
  name = name.trim_end().to_string();

  println!("Hello, {0}! {1}", name, greating);
}