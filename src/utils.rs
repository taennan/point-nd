
#[cfg(any(feature = "appliers", feature = "var-dims"))]
use arrayvec::ArrayVec;

///
/// Forces an ArrayVec to return it's contained array
///
/// For use ONLY within the apply, extend and contract methods as their constant
/// generics ensure that ArrayVec's are always filled with initialised values
///
#[cfg(any(feature = "appliers", feature = "var-dims"))]
pub(crate) fn arrvec_into_inner<T, const N: usize>(arrvec: ArrayVec<T, N>, method_name: &str) -> [T; N] {
    match arrvec.into_inner() {
        Ok(arr) => arr,
        _ => panic!(
            "Couldn't convert ArrayVec into array in {}() method. \
             This operation should never have panicked. Please contact \
             the maintainers of PointND if troubles persist",
             method_name
        )
    }
}

#[cfg(any(feature = "appliers", feature = "var-dims"))]
pub const ARRVEC_CAP: usize = u32::MAX as usize;

/// Function pointer type to pass to  `apply()` in `PointND`'s
#[cfg(feature = "appliers")]
pub type ApplyFn<T, U> = fn(T) -> U;

/// Function pointer type to pass to  `apply_dims()` in `PointND`'s
#[cfg(feature = "appliers")]
pub type ApplyDimsFn<T> = fn(T) -> T;

///
/// Function pointer type to pass to  `apply_vals()` in `PointND`'s
///
/// Is equivalent to the `ApplyPointFn` alias
///
#[cfg(feature = "appliers")]
pub type ApplyValsFn<T, U, V>  = fn(T, V) -> U;

///
/// Function pointer type to pass to  `apply_point()` in `PointND`'s
///
/// Is equivalent to the `ApplyValsFn` alias
///
#[cfg(feature = "appliers")]
pub type ApplyPointFn<T, U, V> = ApplyValsFn<T, U, V>;


