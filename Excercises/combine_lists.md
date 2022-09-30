## Write a function that combines two lists by alternatingly taking elements.

```
fn combine(l1:[i32;5], l2:[i32;5]) -> [i32; 10] {
    let mut result: [i32;10] = [0;10];
    let mut i = 0;

    for j in 0..5 {
        result[i] = l1[j];
        result[i + 1] = l2[j];
        i = i + 2;
    }

    result
}

fn main() {
    println!("{:?}", combine([1,2,3,4,5], [9,8,7,6,5]));
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=0b8c331a0ce29d8dcba3e31bd99ae118)
