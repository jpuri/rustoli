## Write a function that takes a list of strings an prints them, one per line, in a rectangular frame.

```
*********
* Hello *
* World *
* in    *
* a     *
* frame *
*********
```

```
fn print_times(t: usize, c: char, new_line: bool) {
    for _ in 0..t {
        print!("{}", c);
    }
    if new_line {
        println!("");
    }
}

fn rec_print(v: Vec<&str>) {
    let longest = v.iter().map(|x| x.len()).max().unwrap_or(0);
    print_times(longest + 4, '*', true);
    for w in v.iter() {
        print!("* ");
        print!("{}", w);
        let filler_length: usize = longest - w.len();
        print_times(filler_length, ' ', false);
        println!(" *");
    }
    print_times(longest + 4, '*', true);
}

fn main() {
    let v = vec!["Hello", "World", "in", "a", "Frame"];
    rec_print(v);
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=fc76d295ec0e448a81aa3f229ff239cc)
