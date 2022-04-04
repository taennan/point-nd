
#[cfg(any(feature = "appliers", feature = "var_dims"))]
pub const ARRVEC_CAP: usize = u32::MAX as usize;

/// Function pointer type to pass to  ```apply()``` in ```PointND```'s
#[cfg(feature = "appliers")]
pub type ApplyFn<T, U> = fn(T) -> U;

/// Function pointer type to pass to  ```apply_dims()``` in ```PointND```'s
#[cfg(feature = "appliers")]
pub type ApplyDimsFn<T> = fn(T) -> T;

///
/// Function pointer type to pass to  ```apply_vals()``` in ```PointND```'s
///
/// It is equivalent to the ```ApplyPointFn``` alias
///
#[cfg(feature = "appliers")]
pub type ApplyValsFn<T, U, V>  = fn(T, V) -> U;

///
/// Function pointer type to pass to  ```apply_point()``` in ```PointND```'s
///
/// It is equivalent to the ```ApplyValsFn``` alias
///
#[cfg(feature = "appliers")]
pub type ApplyPointFn<T, U, V> = ApplyValsFn<T, U, V>;

