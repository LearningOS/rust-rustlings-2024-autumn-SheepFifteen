// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        //word = optional_target {
        //    assert_eq!(word, target);
        //}
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        } else {
            panic!("optional_target was None when it should have been Some");
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // integer = optional_integers.pop() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }
        // while let integer = optional_integers.pop() {
        //     if let Some(expected) = cursor.checked_sub(1) {
        //         // 使用 checked_sub 避免溢出
        //         assert_eq!(integer, expected);
        //         cursor = expected;
        //     } else {
        //         panic!("Underflow error");
        //     }
        // }
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
