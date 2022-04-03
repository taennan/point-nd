
#[cfg(any(feature = "appliers", feature = "var_dims"))]
pub const ARRVEC_CAP: usize = u32::MAX as usize;

/// Alias of the function pointer type to pass to  ```apply()``` in ```PointND```'s
pub type ApplyFn<T, U> = fn(T) -> U;

/// Alias of the function pointer type to pass to  ```apply_dims()``` in ```PointND```'s
pub type ApplyDimsFn<T> = fn(T) -> T;

///
/// Alias of the function pointer type to pass to  ```apply_vals()``` in ```PointND```'s
///
/// It is equivalent to the ```ApplyPointFn``` alias
///
pub type ApplyValsFn<T, U, V>  = fn(T, V) -> U;

///
/// Alias of the function pointer type to pass to  ```apply_point()``` in ```PointND```'s
///
/// It is equivalent to the ```ApplyValsFn``` alias
///
pub type ApplyPointFn<T, U, V> = ApplyValsFn<T, U, V>;

