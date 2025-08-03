use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [i32; n],
    }

    let mut ans = 0;

    d.sort();

    let mut now = 0;
    for i in 0..n {
        if now < d[i] {
            ans += 1;
            now = d[i];
        }
    }

    println!("{}", ans);
}
