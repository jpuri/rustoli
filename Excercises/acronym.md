# Acronym

```
fn create_acronym(source: &str) -> String {
  source
    .split_whitespace()
    .flat_map(|s| s.chars().nth(0))
    .collect::<String>()
    .to_ascii_uppercase()
}


fn main() {
  println!("{}", create_acronym("To Be Done"));
  println!("{}", create_acronym("As far as I know"));
  println!("{}", create_acronym("Looks good to me"));
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3463fae04c097dd32430d12c9c795dd7)
