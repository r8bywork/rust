#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, non_snake_case))]
// use hello_rust::modules::lib::rustLib::rustLib::{*};
mod modules;
use hello_rust::modules::lib::rustLib::rustLib::closure;

use crate::modules::lib::rustLib::rustLib::{*};
use std::io::{self, Bytes};
use std::io::{BufReader, Write, BufRead, ErrorKind};
use std::fs::{File, read};

fn main() {
  closure();
}