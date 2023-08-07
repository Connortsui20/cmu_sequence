use cmu_sequence::{array_sequence::ArraySequence, Sequence};

#[test]
fn test_is_sorted() {
    assert!(ArraySequence::from(vec![1, 2, 3, 4, 5]).is_sorted());
    assert!(!ArraySequence::from(vec![1, 2, 4, 3, 5]).is_sorted());
}

#[test]
fn test_sorted_struct() {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct CompareStruct {
        i: i32,
        u: usize,
        a: [u8; 5],
    }
    let test = ArraySequence::from(vec![
        CompareStruct {
            i: -1,
            u: 42,
            a: [1, 2, 3, 4, 5],
        },
        CompareStruct {
            i: -1,
            u: 52,
            a: [1, 2, 3, 4, 5],
        },
        CompareStruct {
            i: 0,
            u: 42,
            a: [1, 2, 3, 4, 5],
        },
        CompareStruct {
            i: 0,
            u: 42,
            a: [1, 2, 6, 4, 5],
        },
        CompareStruct {
            i: 10,
            u: 40,
            a: [0, 2, 3, 4, 5],
        },
    ]);
    assert!(test.is_sorted());

    let test_bad = ArraySequence::from(vec![
        CompareStruct {
            i: -1,
            u: 42,
            a: [1, 2, 3, 4, 5],
        },
        CompareStruct {
            i: 0,
            u: 42,
            a: [1, 2, 3, 4, 6],
        },
        CompareStruct {
            i: 0,
            u: 42,
            a: [1, 2, 3, 4, 5],
        },
        CompareStruct {
            i: 0,
            u: 42,
            a: [1, 2, 6, 4, 5],
        },
        CompareStruct {
            i: 10,
            u: 40,
            a: [0, 2, 3, 4, 5],
        },
    ]);
    assert!(!test_bad.is_sorted());
}
