// ユーザー入力を受け付け、結果を出力するためにはioライブラリをスコープに入れる必要がある。stdは標準ライブラリ。
// stdは標準ライブラリの中に含まれている。この標準ライブラリのセットのことをpreludeと呼ぶ。
// preludeについて詳しい情報はhttps://doc.rust-lang.org/std/prelude/index.html
// 使いたい型がpreludeにない場合はuseで明示的にスコープに入れる必要がある。
use std::io;

// main関数がプログラムへのエントリーポイント
// fn構文で関数を宣言している
// ()内には引数を入力。このコードは引数なし。
fn main() {
    // println!は標準出力のマクロ
    println!("Guess the number!");          // 数を当ててごらん

    println!("Please input your guess.");   // ほら、予想を入力してね

    // let文を使って変数を定義
    // let apples = 5; これはimmutable
    // let mut apples = 5; これはmutable
    let mut guess = String::new(); // String::newは新しい空の文字列を作成する

    io::stdin() // 標準入力へのハンドル
        .read_line(&mut guess) // メソッドでユーザからの入力を得る。guessは可変なのでmutをつける。
        .expect("Failed to read line");     // 行の読み込みに失敗しました
        // io::Resultインスタンスは、okの場合は受け取った値のみ返し、予期せぬ値なら例外値を返す。Errが出るのはOS起因。

    println!("You guessed: {}", guess);     // 次のように予想しました: {}
}
