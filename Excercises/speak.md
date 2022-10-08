## Speak out numbers

```
const SPACE: &str = " ";

const ORDER_MAP: [&str; 5] = ["Hundred", "Thousand", "Million", "Billion", "Trillion"];

fn join_with_space (str1: String, str2: String) -> String {
    if str1.len() == 0 {
        return str2;
    }
    if str2.len() == 0 {
        return str1;
    }
    str1 + SPACE + &str2
}

fn add_order (num: String, order: usize) -> String {
    if num.len() > 0  {
        return join_with_space(num, ORDER_MAP[order].to_string());
    }
    num
}

fn upto_teen(num: u64) -> String {
    match num {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        11 => "Eleven",
        12 => "Twelve",
        13 => "Thirteen",
        14 => "Fourteen",
        15 => "Fifteen",
        16 => "Sixteen",
        17 => "Seventeen",
        18 => "Eighteen",
        19 => "Ninteen",
        _ => ""
    }.to_string()
}

fn less_than_hundred(num: u64) -> String {
    let pre = match num / 10 {
        2 => "Twenty",
        3 => "Thirty",
        4 => "Fourty",
        5 => "Fifty",
        6 => "Sixty",
        7 => "Seventy",
        8 => "Eighty",
        9 => "Ninty",
        _ => ""
    }.to_string();
    join_with_space(pre, upto_teen(num % 10))
}

fn big_num(num: u64, order: usize) -> String {
    let mut suf = speak(num % 1000);
    if order > 0 {
        suf = add_order(suf, order);
    }

    let mut pre: String;
    if num / 1000 > 1000 {
        pre = big_num(num / 1000, order + 1);
    } else {
        pre = speak(num / 1000);
        pre = add_order(pre, order + 1);
    }

    join_with_space(
        pre,
        suf
    )
}

fn speak(num: u64) -> String {
     match num {
        0 => "".to_string(),
        1..=19 => upto_teen(num),
        20..=99 => less_than_hundred(num),
        100..=999 => {
            join_with_space(
                add_order(speak(num / 100), 0),
                speak(num % 100)
            )
        },
        _ => big_num(num, 0),
    }
}

fn main() {
    println!("Speak {} is: {}", 1, speak(1));
    println!("Speak {} is: {}", 10, speak(10));
    println!("Speak {} is: {}", 19, speak(19));
    println!("Speak {} is: {}", 25, speak(25));
    println!("Speak {} is: {}", 50, speak(50));
    println!("Speak {} is: {}", 75, speak(75));
    println!("Speak {} is: {}", 150, speak(150));
    println!("Speak {} is: {}", 1050, speak(1050));
    println!("Speak {} is: {}", 1800, speak(1800));
    println!("Speak {} is: {}", 15000, speak(15000));
    println!("Speak {} is: {}", 250000, speak(250000));
    println!("Speak {} is: {}", 1500000, speak(1500000));
    println!("Speak {} is: {}", 500000000, speak(500000000));
    println!("Speak {} is: {}", 1500000000, speak(1500000000));
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4a8af1f7a3111d35fe56e540b951fc75)