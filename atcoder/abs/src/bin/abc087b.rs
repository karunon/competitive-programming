use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }

    let mut ans = 0;
    for i in 0..=a {
        for j in 0..=b {
            let k = x - (500 * i + 100 * j);
            if 0 <= k && k / 50 <= c {
                ans += 1;
            }
        }
    }

    println! {"{}", ans};
}
