## Pascal Triangle

```
struct PascalTriangle {
    row_count: u32
}

impl PascalTriangle {
    pub fn new (row_count: u32) -> PascalTriangle {
        return PascalTriangle {
            row_count: row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (1..self.row_count).map(|c| PascalTriangle::create_row(c)).collect()
    }

    fn create_row(row_num: u32) -> Vec<u32> {
        let mut r: Vec<u32> = vec![1];

        for i in 1..row_num {
            let p = r.last().unwrap();
            r.push((p * (row_num - i)) / i);
        }

        r
    }
}

fn main() {
    let size = 10;
    let triangle = PascalTriangle::new(size);
    println!("Pascal triangle of size {} is:", size);
    for row in triangle.rows() {
        println!("{:?}", row);
    }
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1a7eb6ecfe39e7e3153443a36d55fd02)
