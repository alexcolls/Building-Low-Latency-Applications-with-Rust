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
