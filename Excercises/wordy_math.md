# Wordy Math

Parse and evaluate simple math word problems returning the answer as an integer.

```
fn evaluate(str: &str) -> i32 {

    let str_list: Vec<&str> = str
        .split_whitespace()
        .collect();

    match &str_list[..] {
        &[ref num_str1 @.., "multiplied", "by", num_str2] => {
            evaluate(&num_str1.join(" ")) * evaluate(num_str2)
        },
        &[ref num_str1 @.., "divided", "by", num_str2] => {
            evaluate(&num_str1.join(" ")) / evaluate(num_str2)
        },
        &[ref num_str1 @.., "plus", num_str2] => {
            evaluate(&num_str1.join(" ")) + evaluate(num_str2)
        },
        &[ref num_str1 @.., "minus", num_str2] => {
            evaluate(&num_str1.join(" ")) - evaluate(num_str2)
        },
        &[num_str] => {
            num_str.parse().unwrap()
        },
        _ => 0,
    }
}

fn evaluate_problem(problem_str: &str) -> i32 {

    let str_list: Vec<&str> = problem_str
        .trim_end_matches("?")
        .split_whitespace()
        .collect();

    match &str_list[..] {
        &["What", "is", ref num_str @..] => {
            evaluate(&num_str.join(" "))
        },
        _ => 0
    }
}


fn main() {
  println!("What is 5? = {}", evaluate_problem("What is 5?"));
  println!("What is 5 minus 10? = {}", evaluate_problem("What is 5 minus 10?"));
  println!("What is 5 minus 10 multiplied by 8? = {}", evaluate_problem("What is 5 minus 10 multiplied by 8?"));
  println!("What is 5 minus 2 divided by 2? = {}", evaluate_problem("What is 5 minus 2 divided by 2?"));
  println!("What is 5 minus 10 plus 8? = {}", evaluate_problem("What is 5 minus 10 plus 8?"));
  println!("What is 5 minus 10 plus 8 plus 2? = {}", evaluate_problem("What is 5 minus 10 plus 8 plus 2?"));
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6f7675ede4fc02a4550103f324f01b66)
