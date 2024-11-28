/* Ownership and Borrowing Concurrency */

use std::thread;

fn main() {
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
      println!("{:?}", data); // Ownership of `data` is moved to the thread
    });

    handle.join().unwrap();
    // println!("{:?}", data); // Error: `data` is no longer accessible
}

/* Concurrency Traits: The Send Trait */

let data = vec![1, 2, 3];
let handle = thread::spawn(move || {
  println!("{:?}", data); // `data` is moved, and its type must implement `Send`
});
handle.join().unwrap();

/* Concurrency Primitives: Mutex for Mutual Exclusion */

use std::sync::Mutex;
use std::thread;

fn main() {
  let data = Mutex::new(0);
  let handles: Vec<_> = (0..10)
    .map(|_| {
      let data = data.clone();
      thread::spawn(move || {
        let mut num = data.lock().unwrap();
        *num += 1;
      })
    })
    .collect();
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Result: {}", *data.lock().unwrap());
}

/* Concurrency Primitives: Arc for Atomic Reference Counting */

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let data = Arc::new(Mutex::new(0));
  let handles: Vec<_> = (0..10)
    .map(|_| {
      let data = Arc::clone(&data);
      thread::spawn(move || {
        let mut num = data.lock().unwrap();
        *num += 1;
      })
    })
    .collect();
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Result: {}", *data.lock().unwrap());
}

/* Concurrency Primitives: Channels for Message Passing */

use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    tx.send("Hello from thread").unwrap();
  });
  let message = rx.recv().unwrap();
  println!("{}", message);
}

/* Immutable Borrowing in Concurrent Contexts */

use std::thread;

fn main() {
  let data = vec![1, 2, 3];
  let data_ref = &data; // Immutable borrow
  let handle = thread::spawn(move || {
    println!("Thread sees: {:?}", data_ref); // Safe read-only access
  });
  handle.join().unwrap();
  println!("Main thread sees: {:?}", data_ref); // Safe read-only access
}

/* Mutable Borrowing Restrictions */

use std::thread;

fn main() {
  let mut data = vec![1, 2, 3];
  let handle = thread::spawn(move || {
    // This would result in a compile-time error if another thread tried to access `data`
    data.push(4);
    println!("Thread modified data: {:?}", data);
  });
  handle.join().unwrap();
  // println!("{:?}", data); // Error: Ownership of `data` was moved to the thread
}

/* Ownership Transfer in Threads */

use std::thread;

fn main() {
  let data = vec![1, 2, 3];
  let handle = thread::spawn(move || {
    println!("Thread owns data: {:?}", data); // Ownership moved to thread
  });
  handle.join().unwrap();
  // println!("{:?}", data); // Compile-time error: data's ownership is moved
}

/* Primitives for Explicit Synchronization */

use std::sync::Mutex;
use std::thread;

fn main() {
  let data = Mutex::new(vec![1, 2, 3]);
  let handle = thread::spawn({
    let data = data.clone();
    move || {
      let mut locked_data = data.lock().unwrap();
      locked_data.push(4);
      println!("Thread modified data: {:?}", locked_data);
    }
  });
  handle.join().unwrap();
  println!("Main thread sees: {:?}", data.lock().unwrap());
}

/* Combining Arc and Mutex */

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let data = Arc::new(Mutex::new(vec![1, 2, 3]));
  let handles: Vec<_> = (0..3)
    .map(|i| {
      let data = Arc::clone(&data);
      thread::spawn(move || {
        let mut locked_data = data.lock().unwrap();
        locked_data.push(i);
        println!("Thread {:?} added {:?}", i, locked_data);
      })
    })
    .collect();
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Final data: {:?}", data.lock().unwrap());
}

/* Channels for Message Passing */

use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    tx.send(vec![1, 2, 3]).unwrap();
  });
  let received = rx.recv().unwrap();
  println!("Main thread received: {:?}", received);
}

/* Using Send for Ownership Transfer */

use std::thread;

fn main() {
  let data = vec![1, 2, 3]; // A type that implements Send
  let handle = thread::spawn(move || {
    println!("Thread sees: {:?}", data); // Ownership of `data` is moved here
  });
  handle.join().unwrap();
  // data is no longer accessible here
}

/* Non-Send Type */

use std::thread;

fn main() {
  let raw_ptr: *const i32 = &10;
  // Attempting to transfer a raw pointer to a thread will not compile
  let handle = thread::spawn(move || {
    unsafe {
      println!("Value at pointer: {}", *raw_ptr); // Unsafe and not thread-safe
    }
  });
  handle.join().unwrap();
}

/* Manual Send Implementation */

struct MyType(*mut i32); // Contains a raw pointer

unsafe impl Send for MyType {} // Declare it Send, but the developer is responsible for ensuring safety

fn main() {
  let my_data = MyType(Box::into_raw(Box::new(42)));
  let handle = thread::spawn(move || {
    unsafe {
      println!("Data: {}", *my_data.0); // Unsafe access
    }
  });
  handle.join().unwrap();
}

/* Sync with Immutable Data */

use std::thread;

fn main() {
  let data = vec![1, 2, 3]; // Vec<T> is Sync because &Vec<T> is Send
  let handle = thread::spawn(|| {
    println!("Thread reads: {:?}", data); // Immutable access is safe
  });
  handle.join().unwrap();
  println!("Main thread reads: {:?}", data); // Main thread can also access
}

/* Mutex<T> and Sync */

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let data = Arc::new(Mutex::new(vec![1, 2, 3])); // Shared mutable data
  let handles: Vec<_> = (0..3)
    .map(|i| {
      let data = Arc::clone(&data);
      thread::spawn(move || {
        let mut locked_data = data.lock().unwrap();
        locked_data.push(i);
      })
    })
    .collect();
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Final data: {:?}", data.lock().unwrap());
}

/* Types That Are Not Sync: Unsafe Sharing */

use std::thread;

struct UnsafeData(*mut i32);

unsafe impl Sync for UnsafeData {}

fn main() {
  let data = UnsafeData(Box::into_raw(Box::new(42)));
  let handle = thread::spawn(move || {
    unsafe {
      println!("Value: {}", *data.0); // Unsafe and thread-unsafe access
    }
  });
  handle.join().unwrap();
}

/* Concurrency Primitives: Counting with Mutex */

use std::sync::Mutex;
use std::thread;

fn main() {
  let data = Mutex::new(0);
  let handles: Vec<_> = (0..10)
    .map(|_| {
      let data = data.clone();
      thread::spawn(move || {
        let mut num = data.lock().unwrap();
        *num += 1; // Safely increment the shared counter
      })
    })
    .collect();
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Final count: {}", *data.lock().unwrap());
}

/* Combining Arc and Mutex */

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let data = Arc::new(Mutex::new(0));
  let handles: Vec<_> = (0..10)
    .map(|_| {
      let data = Arc::clone(&data);
      thread::spawn(move || {
        let mut num = data.lock().unwrap();
        *num += 1; // Safely increment shared data
      })
    })
    .collect();
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Final count: {}", *data.lock().unwrap());
}

/* Using Channels for Message Passing */

use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();
  let handle = thread::spawn(move || {
    tx.send("Hello from the thread").unwrap(); // Send a message
  });
  handle.join().unwrap();
  let message = rx.recv().unwrap(); // Receive the message
  println!("Main thread received: {}", message);
}

/* Multiple Producers */

use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();
  for i in 0..5 {
    let tx_clone = tx.clone();
    thread::spawn(move || {
      tx_clone.send(format!("Message {}", i)).unwrap();
    });
  }
  drop(tx); // Close the original sender to signal the end of messages
  for received in rx {
    println!("Received: {}", received);
  }
}

/* Safe Data Sharing with Mutex */

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let data = Arc::new(Mutex::new(0));
  let handles: Vec<_> = (0..10)
    .map(|_| {
      let data = Arc::clone(&data);
      thread::spawn(move || {
        let mut num = data.lock().unwrap();
        *num += 1;
      })
    })
    .collect();
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Final count: {}", *data.lock().unwrap());
}

/* Catching Thread Safety Errors at Compile Time */

use std::sync::Arc;

fn main() {
  let data = Arc::new(42);
  let handle = std::thread::spawn(move || {
    println!("Thread sees: {}", data);
  });
  // println!("Main thread sees: {}", data); // Compile-time error: `data` moved
  handle.join().unwrap();
}

/* Explicit Synchronization with Mutex */

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let data = Arc::new(Mutex::new(vec![1, 2, 3]));
  let data_clone = Arc::clone(&data);
  let handle = thread::spawn(move || {
    let mut locked_data = data_clone.lock().unwrap();
    locked_data.push(4); // Explicitly synchronized access
  });
  handle.join().unwrap();
  println!("Final data: {:?}", data.lock().unwrap());
}

/* Message Passing for Concurrency */

use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    tx.send("Hello from thread").unwrap(); // Message-passing instead of shared state
  });
  let message = rx.recv().unwrap();
  println!("Received: {}", message);
}

/* The Problem with Thread-Based I/O */

use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
  for stream in listener.incoming() {
    let mut stream = stream.unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello, world!").unwrap();
  }
}

/* Rust's Asynchronous Model: Futures */

pub trait Future {
  type Output;

  fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}

/* Rust's Asynchronous Model: Aync Functions */

async fn fetch_data() -> String {
  "Hello, async!".to_string()
}

/* Rust's Asynchronous Model: Await */

async fn main_task() {
  let data = fetch_data().await;
  println!("{}", data);
}

/* Building an Asynchronous Server */

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
  loop {
    let (mut socket, _) = listener.accept().await.unwrap();
    tokio::spawn(async move {
      let mut buffer = [0; 1024];
      socket.read(&mut buffer).await.unwrap();
      socket.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello, async world!").await.unwrap();
    });
  }
}

/* Lifetimes in async */

async fn example(data: &str) {
  println!("{}", data);
}
