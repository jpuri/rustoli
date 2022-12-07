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
