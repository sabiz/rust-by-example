fn main() {
    // This is an example of a line comment.
    // There are two slashes at the beginning of the line.
    // And nothing written after these will be read by the compiler.
    // こちらはラインコメントです
    // 一番左にスラッシュが2つある行と、何も書かれていない行は
    // どちらもコンパイラによって無視されます。試しに実行してみてください

    // println!("Hello, world!");

    // Run it. See? Now try deleting the two slashes, and run it again.
    // でしょ？では次に、左のスラッシュを消去してから実行してください

    /*
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But block comments
     * are extremely useful for temporarily disabling chunks of code.
     * /* Block comments can be /* nested, */ */ so it takes only a few
     * keystrokes to comment out everything in this main() function.
     * /*/*/* Try it yourself! */*/*/
     */
    /*
     * こちらはもう一つのタイプのコメントでブロックコメントと呼ばれます。
     * 普通はラインコメントの方が優れているのですが、こちらはデバッグ時に
     * 役立つ場合があります。
     */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */
    /*
    このように、`*`は、実際にはコメントの前後に１つずつあれば十分です。
    */

    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    // ではブロックコメントがどのようにデバッグに役立つか見てみましょう。
    // 例えば下の例の場合、ブロックコメントがなくなれば結果が変わります。
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

