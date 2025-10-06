fn main() {
    let outer_var = 42;
    
    // A regular function can't refer to variables in the enclosing environment
    // 通常の関数は周辺の環境の変数を参照できない。
    //fn function(i: i32) -> i32 { i + outer_var }
    // TODO: uncomment the line above and see the compiler error. The compiler
    // suggests that we define a closure instead.
    // TODO: 上の行をアンコメントしてコンパイラエラーを見てみよう。
    // 代わりにクロージャを定義することをコンパイラが提案してくれる。

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    // 型アノテーションは、通常の関数と同様の方法で行えるが、必須ではない。
    // `{}`も必須ではない。
    // クロージャは一種の無名関数なので、適切な変数にバインディングしてやるとよい
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i     |          i + outer_var  ;

    // Call the closures.
    // クロージャを呼び出す。
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    // Once closure's type has been inferred, it cannot be inferred again with another type.
    // クロージャの型が一度推論されると、別の型にあらためて推論することはできない。
    //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: uncomment the line above and see the compiler error.
    // TODO: 上の行をアンコメントしてコンパイルエラーを見てみよう。

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    // 引数なしで`i32`を返すクロージャ。
    // 戻り値の型は推論された。
    let one = || 1;
    println!("closure returning one: {}", one());

}
