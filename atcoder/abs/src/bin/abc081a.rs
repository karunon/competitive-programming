use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut ans = 0;

    for i in s.chars() {
        if i == '1' {
            ans += 1;
        }
    }
    println!("{}", ans);
}
