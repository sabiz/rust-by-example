// `NanoSecond`, `Inch`, and `U64` are new names for `u64`.
// `NanoSecond` `Inch` `U64` を `u64`の別名として使用する。
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

// Use an attribute to silence warning.
// 警告を抑えるアトリビュートを使用。
#[allow(non_camel_case_types)]
type u64_t = u64;
// TODO ^ Try removing the attribute
// TODO ^ アトリビュートを使用しない場合、どうなるか見てみましょう。

fn main() {
    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    // 型のエイリアスは、元の型をより型安全にしてくれる **わけではない** ことに注意しましょう。
    // なぜならば、エイリアスは新たな型を定義している **わけではない** からです。
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
