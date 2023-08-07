use super::Sequence;

#[non_exhaustive]
#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ArraySequence<T> {
    vec: Vec<T>,
}

impl<T: Copy + Ord> Sequence<T> for ArraySequence<T> {
    type Sequence<U> = ArraySequence<U>;

    // Creator functions
    fn singleton(x: T) -> Self {
        Self { vec: vec![x] }
    }

    fn tabulate<Tabulator>(len: usize, f: Tabulator) -> Self
    where
        Tabulator: Fn(usize) -> T,
    {
        Self {
            vec: (0..len).map(f).collect(),
        }
    }

    // Observer functions
    fn nth(&self, i: usize) -> T {
        self.vec[i]
    }

    fn length(&self) -> usize {
        self.vec.len()
    }

    fn is_sorted(&self) -> bool {
        self.vec.windows(2).all(|w| w[0] <= w[1])
    }

    fn iterate<Accumulator, U>(&self, base: U, f: Accumulator) -> U
    where
        Accumulator: Fn(U, &T) -> U,
    {
        self.vec.iter().fold(base, f)
    }

    fn reduce<Reducer>(&self, base: T, f: Reducer) -> T
    where
        Reducer: Fn(T, T) -> T,
    {
        match self.vec.iter().copied().reduce(f) {
            None => base,
            Some(x) => x,
        }
    }

    fn enumerate(&self) -> Self::Sequence<(usize, T)> {
        ArraySequence {
            vec: self.vec.iter().copied().enumerate().collect(),
        }
    }

    // Modifier functions
    fn swap(&mut self, i: usize, j: usize) {
        self.vec.swap(i, j)
    }

    fn reverse(&mut self) {
        self.vec.reverse()
    }

    // Ideally we can take in an impl Sequence<T> after implementing IntoIterator
    fn append(&mut self, mut other: Self) {
        self.vec.reserve(other.length());
        self.vec.append(&mut other.vec)
    }

    // Ideally we can take in an impl Sequence<T> after implementing IntoIterator
    fn merge(&mut self, right: Self) {
        debug_assert!(self.is_sorted());
        debug_assert!(right.is_sorted());

        let left = &mut self.vec;
        let right = right.vec;
        let mut i = 0;
        let mut j = 0;
        left.reserve(right.len());

        while i < left.len() && j < right.len() {
            if let std::cmp::Ordering::Less = left[i].cmp(&right[j]) {
                left.push(left[i]);
                i += 1;
            } else {
                left.push(right[j]);
                j += 1;
            }
        }

        if i < left.len() {
            while i < left.len() {
                left.push(left[i]);
                i += 1;
            }
        }

        if j < right.len() {
            while j < right.len() {
                left.push(right[j]);
                j += 1;
            }
        }

        debug_assert!(self.is_sorted());
    }

    fn filter<Predicate>(&mut self, p: Predicate)
    where
        Predicate: Fn(&T) -> bool,
    {
        self.vec.retain(p)
    }

    fn map<F>(&mut self, f: F)
    where
        F: Fn(T) -> T,
    {
        for elem in &mut self.vec {
            *elem = f(*elem);
        }
    }

    fn update(&mut self, update: (usize, T)) {
        self.vec[update.0] = update.1
    }

    fn inject(&mut self, updates: &[(usize, T)]) {
        for &(i, x) in updates {
            self.vec[i] = x;
        }
    }

    fn subseq<R>(&mut self, range: R) -> Self
    where
        R: std::ops::RangeBounds<usize>,
    {
        Self {
            vec: self.vec.drain(range).collect(),
        }
    }

    fn take(&mut self, until: usize) -> Self {
        self.subseq(..until)
    }

    fn drop(&mut self, from: usize) -> Self {
        self.subseq(from..)
    }

    fn scan<Reducer>(&mut self, base: T, f: Reducer) -> T
    where
        Reducer: Fn(T, T) -> T,
    {
        let mut acc = base;
        for elem in &mut self.vec {
            let tmp = *elem;
            *elem = acc;
            acc = f(acc, tmp)
        }
        acc
    }

    fn scan_incl<Reducer>(&mut self, base: T, f: Reducer)
    where
        Reducer: Fn(T, T) -> T,
    {
        let mut acc = base;
        for elem in &mut self.vec {
            acc = f(acc, *elem);
            *elem = acc;
        }
    }

    // Consumer functions
    fn split_at(mut self, mid: usize) -> (Self, Self)
    where
        Self: Sized,
    {
        let remaining = self.vec.split_off(mid);
        (Self { vec: self.vec }, Self { vec: remaining })
    }
}

impl<T> From<ArraySequence<T>> for Vec<T> {
    fn from(item: ArraySequence<T>) -> Self {
        item.vec
    }
}

impl<T> From<Vec<T>> for ArraySequence<T> {
    fn from(item: Vec<T>) -> Self {
        ArraySequence { vec: item }
    }
}

impl<T> AsRef<[T]> for ArraySequence<T> {
    fn as_ref(&self) -> &[T] {
        self.vec.as_ref()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for ArraySequence<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Display not implemented yet")
    }
}
