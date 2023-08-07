pub trait Sequence<T: Copy + Ord> {
    type Sequence<U>;

    // Creator functions
    fn singleton(x: T) -> Self;

    fn tabulate<Tabulator>(len: usize, f: Tabulator) -> Self
    where
        Tabulator: Fn(usize) -> T;

    // Observer functions
    fn nth(&self, index: usize) -> &T;

    fn length(&self) -> usize;

    fn is_sorted(&self) -> bool;

    fn iterate<Accumulator, U>(&self, base: U, f: Accumulator) -> U
    where
        Accumulator: Fn(U, &T) -> U;

    fn reduce<Reducer>(&self, base: T, f: Reducer) -> T
    where
        Reducer: Fn(T, T) -> T;

    fn enumerate(&self) -> Self::Sequence<(usize, T)>;

    // Modifier functions
    fn swap(&mut self, i: usize, j: usize);

    fn reverse(&mut self);

    fn append(&mut self, other: Self);

    fn merge(&mut self, other: Self);

    fn filter<Predicate>(&mut self, p: Predicate)
    where
        Predicate: Fn(&T) -> bool;

    // fn filter_idx();

    fn map<F>(&mut self, f: F)
    where
        F: Fn(T) -> T;

    // fn map_idx();

    fn update(&mut self, update: (usize, T));

    fn inject(&mut self, updates: &[(usize, T)]);

    fn subseq<R>(&mut self, range: R) -> Self
    where
        R: std::ops::RangeBounds<usize>;

    fn take(&mut self, until: usize) -> Self;

    fn drop(&mut self, from: usize) -> Self;

    fn scan<Reducer>(&mut self, base: T, f: Reducer) -> T
    where
        Reducer: Fn(T, T) -> T;

    fn scan_incl<Reducer>(&mut self, base: T, f: Reducer)
    where
        Reducer: Fn(T, T) -> T;

    // Consumer functions
    fn split_at(self, mid: usize) -> (Self, Self)
    where
        Self: Sized;

    // To implement in the future
    // fn map_into<F, U>(self, f: F) -> Self::Sequence<U>
    // where
    //     F: Fn(T) -> U;

    // fn zip();

    // fn flatten();

    // fn iterate_prefixes<Accumulator, U>(self, base: U, f: Accumulator) -> (Self::Sequence<U>, U)
    // where
    //     Accumulator: Fn(U, T) -> U;

    // fn iterate_prefixes_incl<Accumulator, U>(self, base: U, f: Accumulator) -> Self::Sequence<U>
    // where
    //     Accumulator: Fn(U, T) -> U;
}

pub mod array_sequence;
