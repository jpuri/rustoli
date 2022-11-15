## Enum

Enum in Rust is a way of saying a value is one of a possible set of values. For example

```
enum Polygon {
    TRIANGLE,
    SQUARE,
    PENTAGON,
    HEXAGON
}
```

To create instance of enum variants `::` operator can be used:

```
    let square = Polygon.SQUARE;
    let hexagon = Polygon.HEXAGON;
```

All instances of enum have same type which is the enum itself, for example any instance of enum `Polygon` can be passed to function below:

```
    fn draw_polygon(shape: Polygon) {
        // implementation
    }
```

Data can also be associated with enum types, the name of each enum variant that we define also becomes a function that constructs an instance of the enum with that data. Each enum type can be associated with different data types. Also any kind of data can be put inside enum including Strings, numeric types, Structs or other Enums.

```
enum Polygon {
    TRIANGLE([i32; 3], String),
    SQUARE([i32; 4], String),
    PENTAGON([i32; 5], String),
    HEXAGON([i32; 6], String)
}

let triangle = Polygon::TRIANGLE([10, 10, 10], String::from("RED"));
let square = Polygon::SQUARE([20, 20, 20, 20], String::from("YELLOW"));
```

Methods can also be associated with enum types:

```
impl Polygon {
    fn draw(&self) {
        // implementation
    }
}
let triangle = Polygon::TRIANGLE([10, 10, 10], String::from("RED"));
triangle.draw();
```

### The Option Enum

Option enum encode a very common scenario where a value could be something or nothing:

```
enum Option<T> {
    None,
    Some(T),
}

let some_number = Some(5);
let absent_number: Option<i32> = None;
```

`Option<T>` enum and its variants `None`, `Some(T)` are automatically included in rust code.
