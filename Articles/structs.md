## Structs

A `struct` in Rust is a user defined data type that helps group related data. It also allows defining behavior assicoated with the data using methods.

Structs allow us to take more advantage of Rust's type checking, as we can then define variables, function parameters, etc of struct type.

### Defining struct

`struct`  defination are more like creating templates.

```
struct Rectangle {
    height: u32,
    width: u32
}
```

### Instiating struct

`struct` can be instantiated by providing actual value for the fields.

```
let rect = Rectangle {
    height: 40,
    width: 20,
};
```

### Accessing value

`struct` value can be accessed using dot notation. Height of rectangle above will be `rect.height`

Value can also be changed using dot notation, to be able to change a value in `struct` whole instance should be declared as mutable

```
let mut rect = Rectangle {
    height: 40,
    width: 20,
};

rect.height = 80;
```

### Adding method

`method` is a functions defined in context of the struct. It is defined using `impl` block. A struct can have multiple `impl` blocks associated to it.

 It's first argument is always `self` which is the instance of struct the method is called upon. A methods is declared as:

```
impl Rectangle {
    fn area(&self) -> {
        self.height * self.width
    }
}
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
```

In above case method has borrowed the instance of struct thus `&self` is used. Methods can even take ownership of `self` or borrow mutably. Though use case of method taking ownership of `self` are rare as it would prevent use of instance after method call.

More parameters can be passed to method as below:

```
impl Rectangle {
    fn can_contain(&self, &rect2: &Rectangle) -> {
        self.height > &rect2.height && self.width > &rect2.width
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 5,
    };
    println!("rect1 can contain rect2: ", rect1.can_contain(&rect2));
}
```

### Associated functions

Functions that do not take `self` parameter are called associated functions as they are associated with the struct type. They can be called using `::` syntax:

```
impl Rectangle {
    fn new(height: u32, width: u32) -> Rectangle {
        Rectangle {
            height,
            width
        }
    }
}
fn main() {
    let rect1 = Rectangle::new(30, 50);
}
```