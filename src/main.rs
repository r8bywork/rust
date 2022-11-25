#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, non_snake_case))]

// use hello_rust::modules::lib::rustLib::rustLib::{*};
mod modules;
use hello_rust::modules::lib::rustLib::rustLib::closure;

use crate::modules::lib::rustLib::rustLib::{*};
use std::io::{self, Bytes};
use std::io::{BufReader, Write, BufRead, ErrorKind};
use std::fs::{File, read};

fn arrays2() {
  let arr = [1,2,3,4,5,6,7,8,9];
  let mut loop_idx = 0;
  println!("For loop: ");
  for val in arr.iter() {
    println!("For loop: {}", val);
  }

  println!("pedik {}", arr[0])
}

fn main() {
  arrays2();
}