// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
// `enum`を作成してwebイベントを分類する。
// 名前と型情報を併せたものが要素型になっていることに注意。
// `PageLoad != PageUnload` であり、
// `KeyPress(char) != Paste(String)` である。
// 要素型は互いに異なり、互いに非依存である。
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    // `enum`要素型はユニット風でもよい
    PageLoad,
    PageUnload,
    // like tuple structs,
    // タプル風でもよい
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    // C言語スタイルの構造体風でもよい
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
// 引数として`WebEvent`列挙型をとり、何も返さない関数
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    // `to_owned()`は文字列スライスから所有権のある`String`を作成する
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

