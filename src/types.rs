
pub type ApplyFn<T, U> = fn(&T) -> U;
pub type ApplyDimsFn<T> = fn(&T) -> T;

pub type ApplyValsFn<T, U, V>  = fn(&T, &V) -> U;
pub type ApplyPointFN<T, U, V> = ApplyValsFn<T, U, V>;
