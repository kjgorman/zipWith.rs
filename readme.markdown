### zipWith.rs

The standard library presents several schemes for iterating
collections in the `std::iter` module. Amongst these there are the
`Zip` and `Map` iterators, however, there is no `ZipWith`.

It seems like you should be able to implement a `zip_with` function
pretty trivially something like this:

```rust
fn zipWith<R, U: Iterator, C: Fn(U::Item, U::Item) -> R> (combo: C, left: U, right: U) -> ??? {
   left.zip(right).map(| (l, r) | combo(l, r))
}
```

But, what should the return type be? Until abstract return types land
in rust, we can't return `impl Iterator<R>`, we can only return a
concrete implementation of an iterator -- in this case we would return
a `Map`.

I think this is ugly! Types should communicate the intent of a
function, and the intent of `zip_with` isn't (only) to `Map`.

So this library exposes a struct `ZipWith` that holds two iterators
and a closure for zipping elements together, and a trait `IntoZipWith`
(in the sense of `std::iter::IntoIterator`) that exposes the
`zip_with` function.

You can use it [like
this](https://github.com/kjgorman/zipWith.rs/blob/18d5252ab3bc37a3255a2b56c72985221d002931/tests/basics.rs#L9):

```rust
use zipWith::IntoZipWith;
use std::iter::Iterator;

#[test]
fn zipping_two_lists_of_numbers_with_plus_returns_their_sum () {
    let left: Vec<u8> = vec![1, 2, 3];
    let right: Vec<u8> = vec![4, 5, 6];

    let result: Vec<u8> = left.zip_with(right, | l, r | l + r).collect();

    assert_eq!(vec![5, 7, 9], result);
}
```