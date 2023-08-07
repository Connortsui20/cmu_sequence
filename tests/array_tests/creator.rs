use cmu_sequence::{array_sequence::ArraySequence, Sequence};

#[test]
fn singleton_basics() {
    let singleton = ArraySequence::singleton(3);
    assert_eq!(*singleton.nth(0), 3);
}

#[test]
fn tabulate_basics() {
    assert_eq!(
        ArraySequence::tabulate(5, |i| i),
        vec![0, 1, 2, 3, 4].into()
    );

    assert_eq!(
        ArraySequence::tabulate(5, |i| 2 * i + 1),
        vec![1, 3, 5, 7, 9].into()
    );

    assert_eq!(
        ArraySequence::tabulate(5, |_| "Hello"),
        vec!["Hello", "Hello", "Hello", "Hello", "Hello",].into()
    );
}

#[test]
fn tabulate_slice() {
    let slice = [1, 2, 3, 4, 5];
    assert_eq!(
        ArraySequence::tabulate(5, |i| &slice[..=i]),
        vec![
            &slice[..1],
            &slice[..2],
            &slice[..3],
            &slice[..4],
            &slice[..5]
        ]
        .into()
    )
}

#[test]
fn tabulate_arc() {
    use std::sync::Arc;
    let arc = Arc::new(42);
    assert_eq!(
        ArraySequence::tabulate(3, |_| &arc),
        vec![&Arc::new(42), &Arc::new(42), &Arc::new(42)].into()
    )
}

#[test]
fn tabulate_tuple() {
    assert_eq!(
        ArraySequence::tabulate(5, |i| (i as i32, -(i as i32))),
        vec![(0, 0), (1, -1), (2, -2), (3, -3), (4, -4)].into()
    )
}

#[test]
fn tabulate_struct() {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct TestStruct {
        i: i32,
        x: usize,
        y: char,
        z: Option<bool>,
    }
    assert_eq!(
        ArraySequence::tabulate(3, |_| TestStruct::default()),
        vec![
            TestStruct::default(),
            TestStruct::default(),
            TestStruct::default(),
        ]
        .into()
    )
}

#[test]
fn tabulate_ref() {
    let hello_str = "Hello World";
    assert_eq!(
        ArraySequence::tabulate(6, |i| &hello_str[0..(i * 2)]),
        vec!["", "He", "Hell", "Hello ", "Hello Wo", "Hello Worl",].into()
    )
}

#[test]
fn tabulate_struct_ref() {
    let hello_str = "Hello World";
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct TestStruct<'a> {
        s: &'a str,
        r: Option<char>,
    }
    assert_eq!(
        ArraySequence::tabulate(2, |i| TestStruct {
            s: &hello_str[i + 2..i + 5],
            r: std::char::from_digit((i * 1000) as u32, 10)
        }),
        vec![
            TestStruct {
                s: "llo",
                r: Some('0')
            },
            TestStruct { s: "lo ", r: None }
        ]
        .into()
    )
}
