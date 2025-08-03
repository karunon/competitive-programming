use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    }

    for i in 0..=n {
        for j in 0..=n - i {
            if y == i * 10000 + j * 5000 + (n - i - j) * 1000 {
                println!("{} {} {}", i, j, n - i - j);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
