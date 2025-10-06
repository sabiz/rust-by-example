fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            // 変数"addition"の型がu32であるため、
            // このmatch式はu32をリターンしなければならないことに注意。
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                // 変数"i"はu32型であるため、全く問題ない。
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                // 一方、"continue"式はu32をリターンしないが、これでも問題ない。
                // 決してリターンしないため、このmatch式が要求する型に違反しないからである。
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
