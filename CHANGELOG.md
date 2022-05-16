
# Changelog

## 0.5.0

- Removed compulsory `Default`, `Clone` and `Copy` trait bounds
- Renamed `from()` constructor to a more descriptive `from_slice()`
- Changed generics in `apply_vals()` method to accept arrays with items of any type and also to return `PointND`'s with items of any type
- Changed generics in `apply_point()` method to accept and return `PointND`'s with items of any type
- Changed `modifier` arg in apply methods to accept function pointers
- Removed mutating math ops (`Add`, `Neg`, `AddAssign`, _etc_)
- Removed `into_vec()` method for `no_std` compatibility
- Removed (embarrassingly) incorrect documentation

## ..=0.4.1

- Sorry... development moved too fast to keep track

# Upcoming Changes

## 0.6.0

- Moving dimension macros into separate crate