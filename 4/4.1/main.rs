fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! Cannot assign a new value to an immutable variable
    // エラー! イミュータブルな変数には新しい値を代入できません
    // _immutable_binding += 1;
    // FIXME ^ Comment out this line
    // FIXME ^ この行をコメントアウトしましょう
}
