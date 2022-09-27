## Write function to find nth prime number.

```
fn nth_prime(pos: usize) -> u32 {
    let mut primes: Vec<u32> = vec![];

    (2..).filter(|num: &u32| {
        if !primes.iter().any(|p: &u32| num % p == 0) {
            return true;
        }
        primes.push(*num);
        return false;
    }).nth(pos).unwrap()
}

fn main() {
    let pos = 5;
    let num = nth_prime(pos);
    println!("{}th prime number is {}", pos, num);
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=5905bd822da32ed897f656ad343e8146)
