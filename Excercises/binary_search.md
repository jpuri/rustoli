## Implement binary search in Rust

```
fn search(list: &Vec<isize>, num: isize, start: usize, end: usize) -> usize {
    let pos = start + (end - start) / 2;
    let &value = list.get(pos).unwrap();

    if value == num {
        return pos;
    } else if value > num {
        return search(list, num, start, pos - 1);
    }
    return search(list, num, pos + 1, end);
}

fn main() {
    let mut list = vec![1, 5, 10, 2, 15, -10, -100, 20, 5, 7, 19];
    let num = 19;
    list.sort();
    let loc = search(&list, num, 0, list.len());
    println!("Location of {} in {:?} is: {}", num, list, loc);
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ddf81d9abb9a6bb7acf38583d4a65294)
