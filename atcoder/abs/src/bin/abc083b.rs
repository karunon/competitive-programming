use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut ans = 0;

    for i in 1..=n {
        let mut w = 0;
        let mut copy_i = i;
        loop {
            w += copy_i % 10;
            copy_i /= 10;
            if copy_i <= 0 {
                break;
            }
        }
        if a <= w && w <= b {
            ans += i;
        }
    }

    println!("{}", ans);
}
