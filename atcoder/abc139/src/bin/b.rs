use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let mut ans = 0;
    let mut sum = 1;

    while sum < b {
        sum += a - 1;
        ans += 1;
    }

    println!("{}", ans);
}
