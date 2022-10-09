## Macro to create Hashmap

```
macro_rules! hashmap {
    ($($($key: expr => $val: expr)+$(,)?)*) => {{
        let mut map = std::collections::HashMap::new();
        $($(map.insert($key, $val);)*)*
        map
    }}
}

fn main() {
    let map = hashmap!('a' => 3, 'b' => 11, 'z' => 32);
    println!("{:?}", map)
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=496a2968805fc021b1d01907c505f65b)