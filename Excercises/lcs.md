## Given two strings, write a program that efficiently finds the longest common subsequence

```
fn lcs(s1: &String, s2: &String) -> String {

    if s1.len() == 0 || s2.len() == 0 {
        return "".to_string();
    }

    let c1 = s1.chars().nth(s1.len() - 1).unwrap();
    let c2 = s2.chars().nth(s2.len() - 1).unwrap();

    let s1_short = &s1[0..s1.len() - 1].to_string();
    let s2_short = &s2[0..s2.len() - 1].to_string();

    if c1 == c2 {
        return format!("{}{}", lcs(s1_short, s2_short), c1);
    }

    let m1 = lcs(&s1, s2_short);
    let m2 = lcs(s1_short, &s2);

    if m2.len() > m1.len() {
        return m2;
    }

    m1
}

fn main() {
    let s1 = "test".to_string();
    let s2 = "rest".to_string();
    let cs = lcs(&s1, &s2);
    println!("Longest comon sequence is: {}", cs);
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=28ef0020f6c9613c2888c9d7f112c328)
