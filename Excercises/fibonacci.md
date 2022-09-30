## Get Nth Fibonacci Number.

The Fibonacci sequence is a type series where each number is the sum of the two that precede it. Nth Fibonacci number can be found using Loop or Recursion.

1. Loop

   ```
   fn fib(n: u8) -> u64 {
       let mut prev = 0;
       let mut curr = 1;
       for _ in 1..n {
           let next = prev + curr;
           prev = curr;
           curr = next;
       }
       curr
   }

   fn main() {
       println!("{}", fib(5));
       println!("{}", fib(10));
       println!("{}", fib(20));
       println!("{}", fib(50));
   }
   ```

   [Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=442aa054302727dd1607aba43c8d2f29)

2. Recursion

   ```
    fn fib(n: u8) -> u64 {
        match n {
            1 => 0,
            2 | 3 => 1,
            _ => fib(n - 1) + fib(n - 2)
        }
    }

    fn main() {
        println!("{}", fib(1));
        println!("{}", fib(5));
        println!("{}", fib(10));
        println!("{}", fib(20));
        println!("{}", fib(50));
    }
   ```

   [Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b6246317ad55d6a9b2e3373df3223398)
