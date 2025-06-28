#![doc = include_str!("../README.md")]
#![no_std]
use core::iter::FusedIterator;

#[doc(hidden)]
pub trait Counter: Copy + Default {
    fn inc(&mut self);
    fn dec(&mut self);
    fn inc_n(&mut self, n: usize);
}
macro_rules! impl_counter {
    ($ty:ty) => {
        impl Counter for $ty {
            #[inline]
            fn inc(&mut self) { *self += 1 as $ty }

            #[inline]
            fn dec(&mut self) { *self -= 1 as $ty }

            #[inline]
            fn inc_n(&mut self, n: usize) { *self += n as $ty }
        }
    };
}
impl_counter!(i8);
impl_counter!(i16);
impl_counter!(i32);
impl_counter!(i64);
impl_counter!(i128);
impl_counter!(isize);
impl_counter!(u8);
impl_counter!(u16);
impl_counter!(u32);
impl_counter!(u64);
impl_counter!(u128);
impl_counter!(usize);
impl_counter!(f32);
impl_counter!(f64);

#[derive(Debug, Clone, Default)]
pub struct Enumerate<I: Iterator, C: Counter> {
    iter: I,
    count: C,
}

impl<I: Iterator, C: Counter> Iterator for Enumerate<I, C> {
    type Item = (C, I::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let i = self.count;
        self.count.inc();
        Some((i, a))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let a = self.iter.nth(n)?;
        self.count.inc_n(n);
        let i = self.count;
        self.count.inc();
        Some((i, a))
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where F: FnMut(B, Self::Item) -> B,
    {
        let mut count = self.count;
        self.iter.fold(init, |acc, ele| {
            let acc = f(acc, (count, ele));
            count.inc();
            acc
        })
    }
}

impl<I, C> DoubleEndedIterator for Enumerate<I, C>
where I: DoubleEndedIterator + ExactSizeIterator,
      C: Counter,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let a = self.iter.next_back()?;
        let len = self.iter.len();
        let mut count = self.count;
        count.inc_n(len);
        Some((count, a))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let a = self.iter.nth_back(n)?;
        let len = self.iter.len();
        let mut count = self.count;
        count.inc_n(len);
        Some((count, a))
    }

    fn rfold<B, F>(self, init: B, mut f: F) -> B
    where F: FnMut(B, Self::Item) -> B,
    {
        let mut count = self.count;
        count.inc_n(self.iter.len());
        self.iter.rfold(init, |acc, ele| {
            count.dec();
            f(acc, (count, ele))
        })
    }
}

impl<I: FusedIterator, C: Counter> FusedIterator for Enumerate<I, C> {}

impl<I: ExactSizeIterator, C: Counter> ExactSizeIterator for Enumerate<I, C> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

macro_rules! def_iterator_ext {
    ($name:ident : $ty:ty) => {
        /// Like [`EnumerateNumber::enumerate_number`]
        #[inline]
        fn $name(self) -> Enumerate<Self, $ty> {
            Enumerate { iter: self, count: Default::default() }
        }
    };
}

pub trait EnumerateNumber: Iterator + Sized {
    def_iterator_ext!(enumerate_i8: i8);
    def_iterator_ext!(enumerate_i16: i16);
    def_iterator_ext!(enumerate_i32: i32);
    def_iterator_ext!(enumerate_i64: i64);
    def_iterator_ext!(enumerate_i128: i128);
    def_iterator_ext!(enumerate_isize: isize);
    def_iterator_ext!(enumerate_u8: u8);
    def_iterator_ext!(enumerate_u16: u16);
    def_iterator_ext!(enumerate_u32: u32);
    def_iterator_ext!(enumerate_u64: u64);
    def_iterator_ext!(enumerate_u128: u128);
    def_iterator_ext!(enumerate_f32: f32);
    def_iterator_ext!(enumerate_f64: f64);

    /// Use other number for enumerate
    ///
    /// # Examples
    ///
    /// ```
    /// use enumerate_number::EnumerateNumber as _;
    ///
    /// let iter = "some".chars().enumerate_number();
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(0.0, 's'), (1.0, 'o'), (2.0, 'm'), (3.0, 'e')])
    /// ```
    #[inline]
    fn enumerate_number<N: Counter>(self) -> Enumerate<Self, N> {
        Enumerate { iter: self, count: Default::default() }
    }
}
impl<I: Iterator> EnumerateNumber for I { }

#[cfg(test)]
mod tests {
    use super::*;

    extern crate alloc;
    use alloc::vec;

    #[test]
    fn fold() {
        let mut elems = vec![];
        (0..5).enumerate_i16().fold((), |(), ele| {
            elems.push(ele);
        });
        assert_eq!(elems, vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        ]);
    }

    #[test]
    fn fold1() {
        let mut elems = vec![];
        let mut iter = (0..5).enumerate_i16();
        assert_eq!(iter.next(), Some((0, 0)));
        iter.fold((), |(), ele| {
            elems.push(ele);
        });
        assert_eq!(elems, vec![
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        ]);
    }

    #[test]
    fn rfold() {
        let mut elems = vec![];
        (0..5).enumerate_i16().rfold((), |(), ele| {
            elems.push(ele);
        });
        assert_eq!(elems, vec![
            (4, 4),
            (3, 3),
            (2, 2),
            (1, 1),
            (0, 0),
        ]);
    }

    #[test]
    fn rfold1() {
        let mut elems = vec![];
        let mut iter = (0..5).enumerate_i16();
        assert_eq!(iter.next(), Some((0, 0)));
        iter.rfold((), |(), ele| {
            elems.push(ele);
        });
        assert_eq!(elems, vec![
            (4, 4),
            (3, 3),
            (2, 2),
            (1, 1),
        ]);
    }

    #[test]
    fn rfold2() {
        let mut elems = vec![];
        let mut iter = (0..5).enumerate_i16();
        assert_eq!(iter.next_back(), Some((4, 4)));
        iter.rfold((), |(), ele| {
            elems.push(ele);
        });
        assert_eq!(elems, vec![
            (3, 3),
            (2, 2),
            (1, 1),
            (0, 0),
        ]);
    }

    #[test]
    fn nth() {
        let mut iter = (0..5).enumerate_i16();
        assert_eq!(iter.nth(1), Some((1, 1)));
        assert_eq!(iter.nth(0), Some((2, 2)));
        assert_eq!(iter.nth(1), Some((4, 4)));
    }

    #[test]
    fn nth_back() {
        let mut iter = (0..5).enumerate_i16();
        assert_eq!(iter.nth_back(1), Some((3, 3)));
        assert_eq!(iter.nth_back(0), Some((2, 2)));
        assert_eq!(iter.nth_back(1), Some((0, 0)));
    }

    #[test]
    fn nth_back1() {
        let mut iter = (0..5).enumerate_i16();
        assert_eq!(iter.nth_back(1), Some((3, 3)));
        assert_eq!(iter.nth(0), Some((0, 0)));
        assert_eq!(iter.nth(0), Some((1, 1)));
    }
}
