# Find scrabble score of string

```
fn letter_score(l: char) -> i32 {
    match l.to_ascii_uppercase() {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0
    }
}

fn scrabble_score(word: &str) -> i32 {
    word.chars().map(letter_score).sum()
}

fn main() {
    let word = "cabbage";
    println!("Scrabble score for {} is {}", word, scrabble_score(word));
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3a55588b4419fc1139a5c84e6f91f382)
