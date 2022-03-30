
pub const MAX_POINT_DIMS: usize = u32::MAX as usize;

pub type ApplyFn<T, U> = fn(&T) -> U;
pub type ApplyDimsFn<T> = fn(&T) -> T;

pub type ApplyValsFn<T, U, V>  = fn(&T, &V) -> U;
pub type ApplyPointFn<T, U, V> = ApplyValsFn<T, U, V>;

