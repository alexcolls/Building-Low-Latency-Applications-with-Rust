/* Ownership and Borrowing */

fn main() {
  let s1 = String::from("hello");
  let s2 = s1; // Ownership is moved to s2
  // println!("{}", s1); // Error: s1 no longer owns the value
}

/* Borrowing and References */

fn main() {
  let mut s = String::from("hello");
  let r1 = &s; // Immutable borrow
  let r2 = &s;
  // let r3 = &mut s; // Error: cannot borrow as mutable while immutable borrows exist
}

/* Zero-Cost Abstractions */

let numbers = vec![1, 2, 3];
let sum: i32 = numbers.iter().map(|x| x * 2).sum(); // Compiles into an efficient loop

/* Concurrency Primitives */

use std::sync::Mutex;
use std::thread;

fn main() {
  let counter = Mutex::new(0);
  let handles: Vec<_> = (0..10)
    .map(|_| {
      let counter = counter.clone();
      thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
      })
    })
    .collect();
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Result: {}", *counter.lock().unwrap());
}

/* Generic and Traits */

fn print_item<T: std::fmt::Display>(item: T) {
  println!("{}", item);
}

/* Compile-Time Guarantees */

fn main() {
  let x = 42;
  let y = &x;
  println!("{}", *y); // Safe because Rust ensures `x` is still in scope
}

/* Fearless Concurrency */

use std::thread;

fn main() {
  let data = vec![1, 2, 3, 4];
  let handles: Vec<_> = (0..4)
    .map(|i| {
      thread::spawn(move || {
        println!("Thread {} got {}", i, data[i]);
      })
    })
    .collect();
  for handle in handles {
    handle.join().unwrap();
  }
}

/* Readable and Maintainable Code */

fn divide(a: i32, b: i32) -> Result<i32, String> {
  if b == 0 {
    Err(String::from("Division by zero"))
  } else {
    Ok(a / b)
  }
}

fn main() {
  match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error: {}", err),
  }
}

/* Zero-Cost Abstraction */

let nums = vec![1, 2, 3, 4];
let sum: i32 = nums.iter().map(|x| x * 2).sum(); // Efficient and expressive

/* Automatic Deallocation */

fn main() {
  let s1 = String::from("hello");
  let s2 = s1; // Ownership moves to s2
  // println!("{}", s1); // Error: s1 is no longer valid
}

/* Borrowing for temporary access */

fn main() {
  let mut data = String::from("Rust");
  let reference = &data; // Immutable borrow
  println!("{}", reference);
  let mutable_reference = &mut data; // Error: cannot borrow as mutable while immutable borrow exists
}

/* Understanding Lifetimes */

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}

fn main() {
  let s1 = String::from("long");
  let s2 = String::from("short");
  println!("Longest: {}", longest(&s1, &s2));
}

/* Traits: Defining Behavior */

trait Greet {
  fn greet(&self) -> String;
}

struct Person {
  name: String,
}

impl Greet for Person {
  fn greet(&self) -> String {
      format!("Hello, {}!", self.name)
  }
}

/* Generics: Enabling Code Reuse */

fn largest<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list.iter() {
      if item > largest {
          largest = item;
      }
  }
  largest
}

/* Unsafe Rust */

fn main() {
  let mut num = 5;
  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  unsafe {
      println!("r1: {}", *r1);
      *r2 = 10;
      println!("r2: {}", *r2);
  }
}
