// Compile Errorなので多分この書き方ではないのよね。
// array.get()で範囲外の時はNoneで返すようにしているはずだから範囲外エラーではないのかな？？

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut i = s.len();

    while i >= 5 {
        if s.get(i - 5..i).unwrap() == "dream".to_string() {
            i -= 5;
        } else if s.get(i - 5..i).unwrap() == "erase".to_string() {
            i -= 5;
        } else if s.get(i - 7..i).unwrap() == "dreamer".to_string() {
            i -= 7;
        } else if s.get(i - 6..i).unwrap() == "eraser".to_string() {
            i -= 6;
        } else {
            println!("NO");
            return;
        }
    }
    if i == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
