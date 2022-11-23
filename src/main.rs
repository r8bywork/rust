#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, non_snake_case))]
use std::io::{self, Bytes};
use std::io::{BufReader, Write, BufRead, ErrorKind};
use rand::Rng;
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;

fn readIo() {
  println!("What is your name?");
  let mut name = String::new();
  io::stdin().read_line(&mut name)
    .expect("Didn't receive input");
  print!("Hello {}!", name.trim_end());
}

fn constants() {
  const ONE_MIL: u32 = 1_000_000;
  const PI: f32 = 3.141592;
  let age = "47";
  let mut age: u32 = age.trim().parse()
    .expect("Age wasn't assigment to number!");
  println!("Your age --> {}", {age+1})
}

fn data_types() {
  //signed int i8,i16,i32,i64,i128, isize
  //unsigned int u8,u16,u32,u64,u128, usize
  //bool true or false
  //char only single ''
  //string only double ""
}

fn random() {
  let random = rand::thread_rng().gen_range(1..10);
  println!("Random {random}");
}

fn branching(){
  let age:i8 = 54;
  if age>=1 && age<18 {
    println!("You are so young! Your age is {age}")
  } else if age==21 || age == 50 {
    println!("You are very strong! Your age is {age}")
  } else if age>=65 {
    println!("You have very important birthday! Your age is {age}")
  } else {
    println!("Not important!");
  }
  let can_vote = if age > 32 {
    true
  } else {
    false
  };
  println!("Can vote? {}", can_vote)
}

fn useMatch () {
  let age = 1;
  let voting_age = 18;
  match age.cmp(&voting_age) {
    Ordering::Less => println!("Can't vote"),
    Ordering::Greater => println!("Can vote"),
    Ordering::Equal => println!("You gained the right"),
    // 1..=18 => println!("Important birthday! {age}"),
    // 21 | 50 => println!("Important birthday! {age}"),
    // 65..=i32::MAX => println!("Important birthday! {age}"),
    // _ => println!("Else {age}")
  }
}

fn arrays() {
  let arr = [1,2,3,4,5,6,7,8,9];
  let mut loop_idx = 0;
  println!("For loop: ");
  for val in arr.iter(){
    println!("For loop: {}", val);
  }

  println!("While loop: ");
  while loop_idx < arr.len() {
    println!("While: {}", arr[loop_idx]);
    loop_idx+=1;
  }

  println!("Loop: ");
  loop_idx = 0;
  loop{
    if arr[loop_idx] % 2 == 0{
      loop_idx+=1;
      continue;
    }
    if arr[loop_idx] == 9 {
      break;
    }
    println!("Loop: {}", arr[loop_idx]);
    loop_idx+=1;
  }

}

fn tuples() {
  let tuples:(u8,String,f64) = (45, "Vanya".to_string(), 50_000.00); 
  println!("My name is {}", tuples.1);
  let (v1,v2,v3) = tuples;
  println!("Age: {}", v1);
  println!("Name: {}", v2);
  println!("Float: {}", v3);
}

fn strings() {
  let mut st1 = String::new();
  let st3 = String::from("x r t b h k k a m c");
  let mut v1: Vec<char> = st3.chars().collect();
  v1.sort(); //sort
  v1.dedup(); //delete duplicate
  for char in v1 { // loop for string of vector
    println!("{}", char);
  }
  let st4: &str = "Random string";
  let mut st5: String = st4.to_string();
  println!("{}", st5);
  let byte_arr = st5.as_bytes(); // gives array of bytes
  let st6 = &st5[0..6]; // slice
  println!("{}", st6);
  st5.clear(); // delete string;
  
  
  st1.push_str("vanya"); // push string to end
  st1.push_str("\tgamer"); // push string to end
  st1.push('A');  // push char to end
  for word in st1.split_whitespace(){ //split by space
    println!("{}", word);
  }
  let st2 = st1.replace('a', "B");  // replace
  println!("Strgin {}", st1);
  println!("Strgin {}", st2);

  let firstSt = String::from("Hello");
  let secondSt = String::from("World");
  let sumOfStrings = firstSt + &secondSt;
  println!("{}", sumOfStrings);

  for char in sumOfStrings.as_bytes() {
    println!("{}", char);
  }
}

fn casting() {
  let int_u8:u8 = 5;
  let int2_u8:u8 = 5;
  let int3_u8:u32 = (int_u8 as u32) + (int2_u8 as u32);
  println!("{}", int3_u8);
}

fn enums () {
  enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
  }

  impl Day{
    fn is_weekend(&self) -> bool {
      match self {
        Day::Saturday | Day::Sunday => true,
        _ => false,
      }
    }
  }

  let today:Day = Day::Saturday;
  match today {
    Day::Monday => println!("Everyone hates Monday"),
    Day::Tuesday => println!("Donut Day"),
    Day::Wednesday => println!("Hump day"),
    Day::Thursday => println!("Pay Day"),
    Day::Friday => println!("Almost Weekend"),
    Day::Saturday => println!("Weekend"),
    Day::Sunday => println!("Weekend"),
  }
  println!("Is today the weekend? {}", today.is_weekend());
}

fn vectors() {
  let vec: Vec<i32> = Vec::new();
  let mut vec1 = vec![12,22,33,44];
  vec1.push(5);
  println!("1st: {}", vec1[0]);
  let second: &i32 = &vec1[1];
  match vec1.get(1) {
    Some(second) => println!("2nd: {}", second),
    None => println!("No 2nd value!"),
  }
  for i in &mut vec1 {
    *i *= 2;
  }
  for i in &vec1 {
    println!("{}",{i});
  }

  println!("Vec length: {}", vec1.len());
  println!("Pop: {:?}", vec1.pop())
}

fn fnTut(x:i32, y:i32) -> i32 {
  //return x + y; or x+y with out ;
  x+y
}

fn multiReturn(x:i32) -> (i32, f32) {
  //let (value1, value2) = multiReturn(32);
  return (x+2, (x as f32)+3.3);
}

fn sumList(list: &[i32]) -> i32{
  let mut sum = 0;
  for &val in list.iter() {
    sum += &val;
  }
  sum
  //  let num1 = vec![1,2,3,4,5];
  // println!("Sum of list = {}", sumList(&num1));
}

fn get_sum_generic<T:Add<Output = T>>(x: T, y: T) -> T {
  return x+y;
}

fn ownership() {
  let str1 = String::from("world!");
  // let str2 = str1; str1 will be delete
  let str2 = str1.clone();
  println!("Hello {}", str1);
}

fn hashMaps() {
  let mut heroes = HashMap::new();
  heroes.insert("SuperMan", "Clark Kent");
  heroes.insert("Batman", "Bruce Wayne");
  heroes.insert("The Flash", "Barry Allen");

  for (k,v) in heroes.iter() {
    println!("Key: {} = Value: {}", k, v);
  }

  if heroes.contains_key("Batman") {
    let the_batman = heroes.get("Batman");
    match the_batman {
      Some(x) => println!("Batman here!"),
      None => println!("No Batman here!"),
    }
  }
}

fn structure () {
  const PI:f32 = 3.141592;
  trait Shape {
    fn new(length: f32, width:f32) -> Self;
    fn area(&self) -> f32;
  }

  struct Rectangle {length: f32, width: f32};
  struct Circle {length: f32, width: f32};

  impl Shape for Rectangle {
    fn new(length: f32,width:f32) -> Rectangle{
      return Rectangle{length, width};
    }
    fn area(&self) -> f32{
      return self.length * self.width;
    }
  }
  impl Shape for Circle {
    fn new(length: f32,width:f32) -> Circle{
      return Circle{length, width};
    }
    fn area(&self) -> f32{
      return (self.length / 2.0).powf(2.0) * PI;
    }
  }

  let rec: Rectangle = Shape::new(10.0, 10.0); 
  let cir: Circle = Shape::new(10.0, 10.0); 
  println!("Rec area {}", rec.area());
  println!("Circle area {}", cir.area());

  // struct Customer{
  //   name: String,
  //   address: String,
  //   balance: f32,
  // }
  // let mut bob = Customer{
  //   name: String::from("Bob"),
  //   address: String::from("London"),
  //   balance: 100.0,
  // };
  // bob.address = String::from("Germany");
}

fn main() {
  structure();
}