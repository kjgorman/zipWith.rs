#![crate_name = "zipWith"]
#![crate_type = "lib"]

use std::iter::{ Iterator, IntoIterator };

#[derive(Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct ZipWith<A, B, C> {
    left: A,
    right: B,
    zipper: C
}

impl<A, B, C, D> Iterator for ZipWith<A, B, C>
    where A: Iterator, B: Iterator, C: Fn(A::Item, B::Item) -> D
{
    type Item = D;

    #[inline]
    fn next(&mut self) -> Option<D> {
        self.left.next().and_then(|x| {
            self.right.next().and_then(|y| {
                Some( (self.zipper) (x, y))
            })
        })
    }
}

pub trait IntoZipWith {
    fn zip_with<R, F, S>(self, other: R, zipper: F)
                           -> ZipWith<Self::IntoIter, R::IntoIter, F>

        where Self: Sized + IntoIterator,
                 R: Sized + IntoIterator,
                 F: Fn(Self::Item, R::Item) -> S
    {
        ZipWith { left: self.into_iter(), right: other.into_iter(), zipper: zipper }
    }
}

impl<'a, T> IntoZipWith for &'a [T] {}
impl<T> IntoZipWith for Vec<T> {}
