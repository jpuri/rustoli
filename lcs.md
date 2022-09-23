## Given two strings, write a program that efficiently finds the longest common subsequence

```
fn lcs(s1: &str, s2: &str) -> String {

    if s1.len() == 0 || s2.len() == 0 {
        return "".to_string();
    }

    let c1 = &s1[s1.len() - 1..s1.len()];
    let c2 = &s2[s2.len() - 1..s2.len()];

    if c1 == c2 {
        return format!("{}{}", lcs(&s1[0..s1.len() - 1], &s2[0..s2.len() - 1]), c1);
    }

    let m1 = lcs(&s1[0..s1.len()], &s2[0..s2.len() - 1]);
    let m2 = lcs(&s1[0..s1.len() - 1], &s2[0..s2.len()]);

    if m2.len() > m1.len() {
        return m2;
    }

    m1
}

fn main() {
    let s1 = "test";
    let s2 = "rest";
    let cs = lcs(s1, s2);
    println!("Longest comon sequence is: {}", cs);
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=69ee8a66f05b603848663daa49fe2414)
