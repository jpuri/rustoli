## Ownership

Different programming languages have different approach towards memory management. Rust has a unique approach of memory management using the concept of ownership.

In Rust there are 2 places where variables are stored:

1. Stack, fixed sized variables line integers, characters, etc are stored on stack.
2. Heap, dynamically sized variables are stored on heap. Managing variables on heap is comparatively harder and this is where ownership comes into picture.

Ownership consists of certain rules, Rust compiler will not let compile the program if any of these rules is violated:

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

Consider example of simple string:

```
{
  let str = String::from("hello world"); // str is valid from this point forward
}
//  str is invalid here
```

As Stringi s created using `String::from` memory is allocated to it. It is owner by variable `str`, as variable `str` goes out of scope, memory is de-allocated.

### Moving

Consider variable assignment like below:

```
{
  let str1 = String::from("hello world");
  let str2 = str1; // str1 is not accessible after this point
}
```

Here, the value of `str1` is moved to `str2`, Trying yo access `str1` after assignment will give compilation issue.

Rust enforces that a value can have only one owner at any given time, this helps in avoiding double free error. Double free error can occur if a value is referenced by more than one owner and the runtime tries to free memory multiple times as those owners try to go out of scope.

Values are moved when passed in funciton call or returned from functions.
