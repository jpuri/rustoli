## Write function that translates a text to Pig Latin.

English is translated to Pig Latin by taking the first letter of every word, moving it to the end of the word and adding ‘ay’. “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.

```
fn pig_latin(text: &String) -> String {

    let mut result = "".to_string();

    for word in text.split(' ') {
        let mut first = word.chars().nth(0).unwrap();
        let rest: String;
        
        if word.len() == 1 {
            rest = "".to_string();
        } else {
            let mut second = word.chars().nth(1).unwrap();
            if first.is_uppercase() {
                second = second.to_ascii_uppercase();
            }
            rest = format!("{}{}", second, &word[2..word.len()]);
        }
        
        first = first.to_ascii_lowercase();

        if result.len() > 0 {
            result = format!("{} {}{}", result, rest, first);
        } else {
            result = format!("{}{}{}", result, rest, first);
        }
    }

    result
}

fn main() {
    let eng_str = "Rust Just Trust".to_string();
    let pig_latin_str = pig_latin(&eng_str);
    println!("Pig Latin: {:?}", pig_latin_str);
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=5f817a1c0361cc8277adea0af05ded55)