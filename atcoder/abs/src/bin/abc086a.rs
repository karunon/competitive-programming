use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let ans = if (a * b) % 2 == 1 { "Odd" } else { "Even" };

    println!("{}", ans);
}
