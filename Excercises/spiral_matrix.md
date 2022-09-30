## generate spiral matrix of size provided.

```
const FACTOR_X: [isize; 4] = [0, 1, 0, -1];
const FACTOR_Y: [isize; 4] = [1, 0, -1, 0];

fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let mut mat: Vec<Vec<u32>> = vec![vec![0; size]; size];

    let mut x = 0;
    let mut y = -1;
    let mut v = 1;

    for i in 0..(size + size - 1) {
        for _ in 0..((size + size - i) / 2) {
            x = x + FACTOR_X[i % 4];
            y = y + FACTOR_Y[i % 4];
            mat[x as usize][y as usize] = v;
            v = v + 1;
        }
    }

    mat
}

fn main() {
    let size = 5;
    let spimat = spiral_matrix(size);
    println!("Spiral matrics of size {} is:", size);
    for row in spimat.iter() {
        println!("{:?}", row);
    }
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7a2c83ae0f9d5cb71b5bb0deeaefb30b)
