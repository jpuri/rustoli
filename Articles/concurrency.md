## Concurrency

Rust is able to handled concurrent programming very elegantly. Concurrency errors are reported at compile time by leveraging ownership and type checking.

### Threads for Simultaneous Execution

Threads can be used to run independent parts of the program simultaneously.
Splitting program to run multiple tasks at same time can improve performance but also increase complexity and issues like race conditiond, deadlocks and other subtle bugs.

Rust library uses 1:1 model of thread implementation, whereby program uses one OS thread per one program thread.

#### Create a New Thread

`thread::spawn` can be used to create a new thread. The code to be executed can be passed to it as closure. Example

```
use std::thread;
use std::time::duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

Call to `thread::sleep` forces the thread to stop execution allowing other threads to run. The turn that threads will take is not guaranteed.

When the main thread of Rust completes all the spawned threads are shut down, whether or not they have finished running.

#### Waiting for all Threads to Finish

`JoinHandle` can be used to block the current thread from running until the thread represented by the handle terminates. Example:

```
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

#### Using `move` Closures

Closures used for creating thread can not borrow variables from its environment, reason is that it is no guarantee that variable will be valid when thread gets executed.

If we use `move` keyword with closure passed to `thread::spawn` closure will take ownership of values it will use from the environment.

```
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let hanlde = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```
