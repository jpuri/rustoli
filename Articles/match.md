## Match

Match is a very powerful controle flow construct in Rust. Match allows to compare a value against a series of patterns and then execute code based on the result.

The power of this construct comes from:

1. expressiveness of patterns
2. compiler ensures that all possible cases are handled

A simple example:

```
enum Polygon {
    Triangle,
    Rectangle,
    Pentagon,
    Hexagon,
}

fn get_sides_count(polygon: Polygon) -> u8 {
    match polygon {
        Polygon::Triangle => 3,
        Polygon::Rectangle => 4,
        Polygon::Pentagon => 5,
        Polygon::Hexagon => 6 ,
    }
}
```

The structure of match consists of `match` keyword followed by an expession and `match` arms, arms are separated by comma. Each arm has 2 parts an pattern and code. The code associated with each arm is an expression. The resulting value of the expression in the matching arm is the value that gets returned for entire match expression.

### Patterm matching

Pattern matching in Rust is very expressive. Following are some examples:

- Capturing `enum` data
  Consider example of `enum` where one of the variants hold data

  ```
  enum Polygon {
      Triangle,
      Rectangle (width, height),
      Pentagon,
      Hexagon,
  }
  ```

  Match for this `enum` can be written as

  ```
  fn get_sides_count(polygon: Polygon) -> u8 {
      match polygon {
          Polygon::Triangle => 3,
          Polygon::Rectangle(height, width) => {
            println!("Area of the Rectangle is: {}", height * width);
            4
          },
          Polygon::Pentagon => 5,
          Polygon::Hexagon => 6 ,
      }
  }
  ```

- Matching with Option<T>
  Consider an example where we are writing a function to find square of a number:

  ```
  fn square(num: Option<i32>) -> u32 {
      match num {
          None => None,
          Some(i) => i * i,
      }
  }
  ```

- CatchAll pattern
  `other` below will capture all cases not specified

  ```
    fn speak(num: i32) -> String {
        match polygon {
          Polygon::Triangle => "three",
          Polygon::Rectangle => "four",
          other => "more than four",
      }
    }
  ```

- `_` placeholder
  `_` can be used to match any value but not capture it

  ```
    enum Polygon {
        Triangle,
        Rectangle,
        Pentagon,
        Hexagon,
    }

    fn get_sides_count(polygon: Polygon) -> String {
        match polygon {
            Polygon::Triangle => "3 sides".to_string(),
            Polygon::Rectangle => "4 sides".to_string(),
            _ => "5 or more sides".to_string(),
        }
    }
  ```

- Ignoring variable
  Variables can be ignored by starting their name with `_`.

  ```
    fn get_num_string(num: i8) -> String {
      match num {
        1 => "one",
        2 => "two",
        3 => "three",
        Some(_num) => "some other number",
      }
    }

  ```

- Matching literal
  Match can be done against literals

  ```
    fn get_num_string(num: i8) -> String {
      match num {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "anything",
      }
    }
  ```

- Match multiple values
  Match can be done against multiple values using `|` syntax.

  ```
    fn get_num_string(num: Number) -> String {
      match num {
        1 | 2 => "one or two",
        3 => "three",
        _ => "anything",
      }
    }
  ```

- Match range of values
  Match can be done for range of values using `..=` syntax.

  ```
    fn get_num_string(num: Number) -> String {
      match num {
        1..=5 => "one through five",
        _ => "something else",
      }
    }
  ```

- Match destructuring structs, enums and tuples.
  Destructuring can be used to match different parts of structs, enums and tuples.

  ```
    struct Point {
      x: i32,
      y: i32,
    }

    fn get_point_on_axis(pt: Point) -> String {
      match pt {
        Point { x, y: 0 } => format!("On the x axis at {}", x),
        Point { x: 0, y } => format!("On the y axis at {}", y),
        Point { x, y } => format!("On neither axis: ({}, {})", x, y),
    }
  ```

- Ignoring parts of value with `...`

```
  fn print_first_last() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }

    }
  }
```

- Match Guards
  A match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen.

```
  fn find_even_odd(num: i32) -> String {
    match num {
        Some(x) if x % 2 == 0 => format!("The number {} is even", x),
        Some(x) => format!("The number {} is odd", x),
        None => (),
    }
  }

```

- `@` bindings
  `@` lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match

```
  enum Message {
      Hello { id: i32 },
  }

  fn print_hello_with_id(msg: Message) {
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
  }
```
