## Declarative macros

Declarative macros enable defining syntax extensions in declarative way. They are also called as "macros by example".

### `macro_rules`

`macro_rules` can be used to declare a macro. Each macro declaration has a name and one or more rules. It has form:

```
macro_rules! $name {
    $rule0 ;
    $rule1 ;
    // …
    $ruleN ;
}
```

Each rule has 2 parts:

1. matcher: it describes the syntax that matches
2. transcriber: it describes the syntax that will replace successful matches

`($pattern) => {$expansion}`

### Pattern matching

As Rust compiler encounters a macro invocation it tries to match the pattern for each rule in lexical order. Pattern must match in entirety for the input to be considered a match. A few points about patterns:

- Grouping token used when invoking pattern is not matched, for instance `test![]`, `test!()` or `test!{}` can be used to invoke macro

  ```
  macro_rules! test {
      () => "test";
  }
  ```

- Pattern can contain literal tokens which are matched exactly. Anything not starting with a `$` is literal. For instance

  `test! { fn get_x(&self) -> u32; }` will match macro

  ```
  macro_rules! test {
      ( fn $name:ident(&self) -> $ret:ty; ) => {};
  }
  ```

- Matterns can specify metavariables which allow input to be matched. These are written as `$<identifier>: <type>`. Type can be one of

  1. `item`: an item like function, strust, module, etc
  2. `block`: a block
  3. `stmt`: a statement
  4. `pat`: a pattern
  5. `expr`: an expression
  6. `ty`: a type
  7. `ident`: a identifier
  8. `path`: a path
  9. `meta`: a meta item
  10. `tt`: token tree

- Patterns can contain repetitions while allow a sequence of tokens to be matched. It has general form `$ ( ... ) sep rep`.
  `(...)` can contain any valid pattern. `sep` is optional separator `;` and `,` are common. And `rep` is required repeat controller, it can be either `*` or `+`.
  A simple example is pattern used in vec! macro declaration below `( $( $x:expr ),* )`.

### Example

A simple example of `vec!` macro:

```
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```
