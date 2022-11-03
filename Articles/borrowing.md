## Borrowing

Creating reference to a variable in Rust is called as borrowing.

For use cases like passing a String to a function moving the variable each time is complicated as variable can not be used in parent function once it is moved to another function. A better approach is creating a reference to the variable and passing the reference to the function.

### reference
**Reference** is like a pointer, it is an address we can follow to access data stored at that address. It does not own the data it references.

Example:
```
fn main() {
    let s1 = String::from("hello");

    some_function(&s1);

    println!("The string is {}.", s1);
}

fn some_function(s: &String) {
    println!("The string in function is {}.", s);
}
```

Note above that `&s1` is passed to the functions that takes `&String`. The ampersands represent **references**.

Rust compiler makes sure that a reference points to a valid value for a particular type for the life of that reference.

### Mutable References
Borrowed value can be modified using mutable reference.


Example:
```
fn main() {
    let mut s1 = String::from("hello");

    some_function(&mut s1);

    println!("The string is {}.", s1);
}

fn some_function(s: &mut String) {
    s.push_str(", World!");
    println!("The string in function is {}.", s);
}
```

Mutable reference can be created for a mutable variable using `&mut` and function signature accept a mutable reference using  type `&mut String`.

Mutable reference has a restriction, if we create mutable reference for a value no other reference can be created for it. This restriction is to avoid data race situation - which can happen due to delay in synchronising the data if pointers try to read it while it is being updated.