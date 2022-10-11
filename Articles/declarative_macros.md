## Declarative macros

Declarative macros enable defining syntax extensions in declarative way. They are also called as "macros by example".

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

### Matcher and Transcriber

Each rule has 2 parts:

1. matcher: it describes the syntax that matches
2. transcriber: it describes the syntax that will replace successful matches

`(pattern) => {expression}`

As Rust compiler encounters a macro invocation it tries to match the pattern for each rule in lexical order. Pattern must match in entirety for the input to be considered a match.

### Pattern Matching

A few points about pattern matching:

- Patterns in both matcher and transcriber can specify metavariables, these allow capture actual inputs that are passed. These are written as `$<identifier>: <type>`. Type can be one of

  1. `item`: an item like function, strust, module, etc
  2. `block`: a block
  3. `stmt`: a statement
  4. `pat`: a pattern
  5. `expr`: an expression
  6. `ty`: a type
  7. `ident`: an identifier
  8. `path`: a path
  9. `meta`: a meta item
  10. `tt`: token tree

- Both matcher and transcriber can contain repetitions which allow a sequence of tokens to be matched. It has general form `$ ( ... ) sep rep`.

  - `$` is literal
  - `(...)` can contain any valid pattern.
  - `sep` is optional separator `;` and `,` are common.
  - `rep` is required repeat controller, it can be either `*` or `+`.

- Both matcher and transcriber can contain literal tokens which are matched exactly. Anything not starting with a `$` is literal. For instance `fn`, `&self` here are literal

  `test! { fn get_x(&self) -> u32; }` will match macro

  ```
  macro_rules! test {
      ( fn $name:ident(&self) -> $ret:ty; ) => {};
  }
  ```

- Grouping token used when invoking macro is not matched, for instance `test![]`, `test!()` or `test!{}` can be used to invoke macro

  ```
  macro_rules! test {
      () => "test";
  }
  ```

### Example

A macro to create hashmap `hashmap!`:

```
macro_rules! hashmap {
    ($($($key: expr => $val: expr)+$(,)?)*) => {{
        let mut map = std::collections::HashMap::new();
        $($(map.insert($key, $val);)*)*
        map
    }}
}
```
