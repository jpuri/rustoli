## Quick sort

```
fn quick_sort(l: Vec<i8>) -> Vec<i8> {

    if l.len() < 2 {
        return l;
    }
    
    let pivot = l.last().copied().unwrap();

    let mut l1: Vec<i8> = Vec::new();
    let mut l2: Vec<i8> = Vec::new();
    let mut pl: Vec<i8> = Vec::new();

    for elm in l {
        if elm < pivot {
            l1.push(elm);
        } else if elm == pivot {
            pl.push(elm);
        } else {
            l2.push(elm);
        }
    }

    let s1 = quick_sort(l1);
    let s2 = quick_sort(l2);

    [s1, pl, s2].concat()
}

fn main() {
    let l = vec![7,9,0,6,5,5,7,4,9,2];
    let s = quick_sort(l);
    println!("Sorted list is: {:?}", s);
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=660a5c90adf441aab60a9930bcd1306f)
