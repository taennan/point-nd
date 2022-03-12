/*!

A simple multidimensional point struct, based on an array.

See the ```PointND``` struct for basic usage

 */

use std::{
    ops::{
        Deref, DerefMut,
        Add, Sub, Mul, Div,
        AddAssign, SubAssign, MulAssign, DivAssign,
        Neg
    }
};


/**

The whole _point_ of the crate (get it?)

This is really just a wrapper around an array with convenience methods for accessing values if it's dimensions are within ```1..=4```

# Basic Usage

## Making a Point

No matter how a PointND is constructed, the second generic arg must be filled with the number of dimensions it needs to have (i.e - the length of the array to push on the stack)

If a point of zero dimensions is constructed, it will panic

```
use point_nd::PointND;

// Creates a 2D point from values of a given vector or array
let vec: Vec<i32> = vec![0, 1];
let p: PointND<_, 2> = PointND::from(&vec);

// Creates a 3D point with all values set to 5
//  When using this function, complete type annotation is necessary
let p: PointND<i32, 3> = PointND::fill(5);

// ERROR: Can't create a point with zero dimensions
// let p: PointND<_, 0> = PointND::fill(9);

// If you don't like writing PointND twice, use this syntax instead
//  Note: The second generic must still be specified
let p = PointND::<_, 2>::from(&vec);
```

## Accessing Values

It is recommended to use the convenience getters if the dimensions of the point are from ```1..=4```

```
use point_nd::PointND;

// A 2D point
let arr: [i32; 2] = [0,1];
let p: PointND<_, 2> = PointND::from(&arr);

// As the point has 2 dimensions, we can access it's values with the x() and y() methods
let x: &i32 = p.x();
let y = p.y();

assert_eq!(*y, arr[1]);

// If the point had 3 dimensions, we could use the above and:
// let z = p.z();

// Or 4:
// ...
// let w = p.w();
```

Otherwise indexing or the ```get()``` method can be used

```
use point_nd::PointND;

let arr: [i32; 2] = [0,1];
let p: PointND<_, 2> = PointND::from(&arr);

// Safely getting
//  Returns None if index is out of bounds
let x: Option<&i32> = p.get(0);
assert_eq!(*x.unwrap(), arr[0]);

// Unsafely indexing
//  If the index is out of bounds, this will panic
//  Note that unlike other accessing methods, this will return a copy of the value
let y: i32 = p[1];
assert_eq!(y, arr[1]);
```

## Querying Size

The number of dimensions can be retrieved using the ```dims()``` method (short for _dimensions_)

```
use point_nd::PointND;

let p: PointND<i32, 2> = PointND::fill(10);
assert_eq!(p.dims(), 2);
```

# Transforming Points

## Appliers

 The ```apply```, ```apply_vals```, ```apply_dims``` and ```apply_point``` methods all consume self and return a new point after applying a function to all contained values

 Multiple appliers can be chained together to make complex transformations to a ```PointND```
 */
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointND<T, const N: usize>([T; N])
    where T: Clone + Copy + Default;

impl<T, const N: usize> PointND<T, N>
    where T: Clone + Copy + Default {

    /**
     Returns a new ```PointND``` with values from the specified array

     This is the only constructor that does not need type annotation

     ### Panics

     If the length of the array is zero
     */
    pub fn new(arr: [T; N]) -> Self {
        if arr.len() == 0 {
            panic!("Cannot construct PointND with zero dimensions");
        }
        PointND(arr)
    }

    /**
     Returns a new ```PointND``` with values from the specified slice

     ### Panics

     If the length of the slice is zero
     */
    pub fn from(slice: &[T]) -> Self {
        let arr: [T; N] = slice.try_into().unwrap();
        PointND::new(arr)
    }

    /**
     Returns a new ```PointND``` with all values set as specified

     ### Panics

     If the dimensions of the point being constructed is zero
     */
    pub fn fill(value: T) -> Self {
        PointND::<T, N>::from(&[value; N])
    }


    /**
     Returns the number of dimensions of the point (a 2D point will return 2, a 3D point 3, _etc_)

     Equivalent to calling ```len()```
     */
    pub fn dims(&self) -> usize {
        self.len()
    }


    /**
     Safe method of setting values

     Sets value at index ```i``` to ```new_val``` and returns ```Ok```. If the index specified was out of range, does nothing and returns ```Err```
     */
    pub fn set(&mut self, i: usize, new_val: T) -> Result<(), ()> {
        if self.dims() < i { return Err(()) }

        self[i] = new_val;
        Ok(())
    }


    // Did not call apply_dims() inside this to avoid the dimension checks it does
    pub fn apply<F>(self, modifier: F) -> Result<Self, ()>
        where F: Fn(T) -> Result<T, ()> {

        let mut arr = [T::default(); N];
        for i in 0..N {
            arr[i] = modifier(self[i])?;
        }

        Ok( PointND::<T, N>::from(&arr) )
    }

    pub fn apply_dims<F>(self, dims: &[usize], modifier: F) -> Result<Self, ()>
        where F: Fn(T) -> Result<T, ()> {

        let mut arr = [T::default(); N];
        for i in 0..N {
            if dims.contains(&i) {
                arr[i] = modifier(self[i])?;
            } else {
                arr[i] = self[i];
            }
        }

        Ok( PointND::<T, N>::from(&arr) )
    }

    pub fn apply_vals<F>(self, values: [T; N], modifier: F) -> Result<Self, ()>
        where F: Fn(T, T) -> Result<T, ()> {

        let mut arr = [T::default(); N];
        for i in 0..N {
            arr[i] = modifier(self[i], values[i])?;
        }

        Ok( PointND::<T, N>::from(&arr) )
    }

    pub fn apply_point<F>(self, other: PointND<T, N>, modifier: F) -> Result<Self, ()>
        where F: Fn(T, T) -> Result<T, ()> {

        self.apply_vals(other.into_arr(), modifier)
    }


    /// Consumes self, returning the contained array
    pub fn into_arr(self) -> [T; N] {
        self.0
    }

    /// Consumes self, returning the contained array as a vector
    pub fn into_vec(self) -> Vec<T> {
        Vec::from(&self[..])
    }

}


// Deref
impl<T, const N: usize> Deref for PointND<T, N>
    where T: Clone + Copy + Default {

    type Target = [T; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }

}
impl<T, const N: usize> DerefMut for PointND<T, N>
    where T: Clone + Copy + Default {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }

}


// Math operators
//  Negation
impl<T, const N: usize> Neg for PointND<T, N>
    where T: Clone + Copy + Default + Neg<Output = T> {

    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut arr = self.into_arr();
        for i in 0..N {
            arr[i] = -arr[i]
        }

        PointND::new(arr)
    }

}

//  Arithmetic
impl<T, const N: usize> Add for PointND<T, N>
    where T: Add<Output = T> + Clone + Copy + Default  {

    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut arr = self.into_arr();
        for i in 0..N {
            arr[i] = arr[i] + rhs[i];
        }

        PointND::new(arr)
    }

}
impl<T, const N: usize> Sub for PointND<T, N>
    where T: Sub<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut arr = self.into_arr();
        for i in 0..N {
            arr[i] = arr[i] - rhs[i];
        }

        PointND::new(arr)
    }

}
impl<T, const N: usize> Mul for PointND<T, N>
    where T: Mul<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut arr = self.into_arr();
        for i in 0..N {
            arr[i] = arr[i] * rhs[i];
        }

        PointND::new(arr)
    }

}
/**
 ### Warning

 Use division with caution! Undefined behavior may occur!

 For example, dividing by a ```PointND``` that contains a zero will cause a panic
 */
impl<T, const N: usize> Div for PointND<T, N>
    where T: Div<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut arr = self.into_arr();
        for i in 0..N {
            arr[i] = arr[i] / rhs[i];
        }

        PointND::new(arr)
    }

}

// Arithmetic Assign
impl<T, const N: usize> AddAssign for PointND<T, N>
    where T: AddAssign + Clone + Copy + Default {

    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] += rhs[i];
        }
    }

}
impl<T, const N: usize> SubAssign for PointND<T, N>
    where T: SubAssign + Clone + Copy + Default {

    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] -= rhs[i];
        }
    }

}
impl<T, const N: usize> MulAssign for PointND<T, N>
    where T: MulAssign + Clone + Copy + Default {

    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] *= rhs[i];
        }
    }

}
/**
 ### Warning

 Use division with caution! Undefined behavior may occur!

 For example, dividing by a ```PointND``` that contains a zero will cause a panic
 */
impl<T, const N: usize> DivAssign for PointND<T, N>
    where T: DivAssign + Clone + Copy + Default {

    fn div_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] /= rhs[i];
        }
    }

}


// Convenience Getters and Setters
/// ### 1D
/// Functions for safely getting and setting the value contained by a 1D ```PointND```
impl<T> PointND<T, 1>
    where T: Clone + Copy + Default  {

    pub fn x(&self) -> &T { &self[0] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }

}
/// ### 2D
/// Functions for safely getting and setting the values contained by a 2D ```PointND```
impl<T> PointND<T, 2>
    where T: Clone + Copy + Default  {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }

}
/// ### 3D
/// Functions for safely getting and setting the values contained by a 3D ```PointND```
impl<T> PointND<T, 3>
    where T: Clone + Copy + Default  {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }
    pub fn z(&self) -> &T { &self[2] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }
    pub fn set_z(&mut self, new_value: T) { self[2] = new_value; }

}
/// ### 4D
/// Functions for safely getting and setting the values contained by a 4D ```PointND```
impl<T> PointND<T, 4>
    where T: Clone + Copy + Default  {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }
    pub fn z(&self) -> &T { &self[2] }
    pub fn w(&self) -> &T { &self[3] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }
    pub fn set_z(&mut self, new_value: T) { self[2] = new_value; }
    pub fn set_w(&mut self, new_value: T) { self[3] = new_value; }

}


#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn cannot_set_out_of_bounds_index() {

        let arr = [0,-1,2,-3];
        let mut p = PointND::new(arr);

        let err = p.set(100, 100);

        assert_eq!(err, Err(()));
        assert_eq!(p.into_arr(), arr);
    }

    #[cfg(test)]
    mod iterating {
        use super::*;

        #[test]
        fn can_iter() {

            let arr = [0, 1, 2, 3];

            let p = PointND::<u8, 4>::from(&arr);
            for (i, item) in p.iter().enumerate() {
                assert_eq!(arr[i], *item);
            }

            let mut p = PointND::<u8, 4>::from(&arr);
            for item in p.iter_mut() {
                *item = 10;
            }

            for i in p.into_iter() {
                assert_eq!(i, 10u8);
            }

        }

    }

    #[cfg(test)]
    mod constructors {
        use super::*;

        #[test]
        fn new_works() {
            let p = PointND::new([0,1,2]);
            assert_eq!(p.dims(), 3);
        }
        #[test]
        #[should_panic]
        fn new_cannot_construct_with_zero_dimensions() {
            PointND::<i32, 0>::new([]);
        }

        #[test]
        fn from_works() {
            let arr = [0.0, 0.1, 0.2];
            let p = PointND::<f64, 3>::from(&arr);
            for i in 0..p.dims() {
                assert_eq!(arr[i], p[i]);
            }
        }
        #[test]
        #[should_panic]
        fn from_cannot_construct_with_zero_dimensions() {
            PointND::<i32, 0>::from(&[]);
        }

        #[test]
        fn fill_works() {
            let fill_val = 21u8;
            let p = PointND::<u8, 5>::fill(fill_val);
            for i in p.into_iter() {
                assert_eq!(i, fill_val);
            }
        }
        #[test]
        #[should_panic]
        fn fill_cannot_construct_with_zero_dimensions() {
            PointND::<i32, 0>::fill(100);
        }

    }

    #[cfg(test)]
    mod appliers {
        use super::*;

        #[test]
        fn can_apply() {

            let arr = [0, 1, 2, 3];

            let p = PointND::<u8, 4>
                ::from(&arr)
                .apply(|a| Ok( a * 2 ))
                .unwrap();

            for (a, b) in arr.iter().zip(p.iter()) {
                assert_eq!(*a * 2, *b);
            }
        }

    }

    #[cfg(test)]
    mod indexing {
        use super::*;

        #[test]
        #[should_panic]
        fn cannot_get_out_of_bounds_index() {
            let p = PointND::new([0,1,2]);
            let _x = p[p.dims() + 1];
        }

        #[test]
        fn can_set_value_by_index() {

            let mut p = PointND::new([0,1,2]);

            let new_val = 9999;
            p[1] = new_val;

            assert_eq!(p.into_arr(), [0, new_val, 2]);
        }

    }

    #[cfg(test)]
    mod operators {
        use super::*;

        #[test]
        fn can_add() {
            let arr = [0, -1, 2, -3];
            let p1 = PointND::new(arr);
            let p2 = PointND::new(arr);

            let p3 = p1 + p2;
            for (a, b) in p3.into_arr().into_iter().zip(arr){
                assert_eq!(a, b + b);
            }
        }
        #[test]
        fn can_add_assign() {
            let arr = [0, -1, 2, -3, 4, -5];
            let mut p1 = PointND::new(arr);

            p1 += PointND::new(arr);
            for i in 0..p1.dims() {
                assert_eq!(p1[i], arr[i] + arr[i]);
            }
        }

        #[test]
        fn can_sub() {
            let arr = [0, -1, 2, -3];
            let p1 = PointND::new(arr);
            let p2 = PointND::new(arr);

            let p3 = p1 - p2;
            for (a, b) in p3.into_arr().into_iter().zip(arr){
                assert_eq!(a, b - b);
            }
        }
        #[test]
        fn can_sub_assign() {
            let arr = [0, -1, 2, -3, 4, -5];
            let mut p1 = PointND::new(arr);

            p1 -= PointND::new(arr);
            for i in 0..p1.dims() {
                assert_eq!(p1[i], arr[i] - arr[i]);
            }
        }

        #[test]
        fn can_mul() {
            let arr = [0, -1, 2, -3];
            let p1 = PointND::new(arr);
            let p2 = PointND::new(arr);

            let p3 = p1 * p2;
            for (a, b) in p3.into_arr().into_iter().zip(arr){
                assert_eq!(a, b * b);
            }
        }
        #[test]
        fn can_mul_assign() {
            let arr = [0, -1, 2, -3, 4, -5];
            let mut p1 = PointND::new(arr);

            p1 *= PointND::new(arr);
            for i in 0..p1.dims() {
                assert_eq!(p1[i], arr[i] * arr[i]);
            }
        }

        #[test]
        fn can_div() {
            let arr = [-1, 2, -3, 4];
            let p1 = PointND::new(arr);
            let p2 = PointND::new(arr);

            let p3 = p1 / p2;
            for (a, b) in p3.into_arr().into_iter().zip(arr){
                assert_eq!(a, b / b);
            }
        }
        #[test]
        fn can_div_assign() {
            let arr = [-1, 2, -3, 4, -5];
            let mut p1 = PointND::new(arr);

            p1 /= PointND::new(arr);
            for i in 0..p1.dims() {
                assert_eq!(p1[i], arr[i] / arr[i]);
            }
        }

        #[test]
        #[should_panic]
        fn cannot_div_if_one_item_is_zero() {
            let arr = [0, -1, 0, -3, 0];
            let p1 = PointND::new(arr);
            let p2 = PointND::new(arr);

            let p3 = p1 / p2;
            for (a, b) in p3.into_arr().into_iter().zip(arr){
                assert_eq!(a, b / b);
            }
        }
        #[test]
        #[should_panic]
        fn cannot_div_assign_if_one_item_is_zero() {
            let arr = [-1, 0, -3, 4, 0];
            let mut p1 = PointND::new(arr);

            p1 /= PointND::new(arr);
            for i in 0..p1.dims() {
                assert_eq!(p1[i], arr[i] / arr[i]);
            }
        }

        #[test]
        fn can_eq() {
            let arr = [0, -1, 2, -3];
            let p1 = PointND::new(arr);
            let p2 = PointND::new(arr);

            assert_eq!(p1, p2);
        }

        #[test]
        fn can_ne() {
            let arr1 = [1,2,3,4];
            let p1 = PointND::new(arr1);
            let arr2 = [5,6,7,8];
            let p2 = PointND::new(arr2);

            assert_ne!(p1, p2);
        }

    }

    #[cfg(test)]
    mod convenience_methods {
        use super::*;

        #[test]
        fn getter_for_1d_points_work() {
            let arr = [0];
            let p = PointND::new(arr);

            assert_eq!(*p.x(), arr[0]);
        }
        #[test]
        fn setter_for_1d_points_work() -> Result<(), ()> {

            let old_vals = [0];
            let new_vals = [4];
            let mut p = PointND::new(old_vals);

            for i in 0..p.dims() {
                p.set(i, new_vals[i])?;
                assert_eq!(p[i], new_vals[i]);
            }

            Ok(())
        }

        #[test]
        fn getters_for_2d_points_work() {
            let arr = [0,1];
            let p = PointND::new(arr);

            assert_eq!(*p.x(), arr[0]);
            assert_eq!(*p.y(), arr[1]);
        }
        #[test]
        fn setters_for_2d_points_work() -> Result<(), ()> {

            let old_vals = [0,1];
            let new_vals = [4,5];
            let mut p = PointND::new(old_vals);

            for i in 0..p.dims() {
                p.set(i, new_vals[i])?;
                assert_eq!(p[i], new_vals[i]);
            }

            Ok(())
        }

        #[test]
        fn getters_for_3d_points_work() {
            let arr = [0,1,2];
            let p = PointND::new(arr);

            assert_eq!(*p.x(), arr[0]);
            assert_eq!(*p.y(), arr[1]);
            assert_eq!(*p.z(), arr[2]);
        }
        #[test]
        fn setters_for_3d_points_work() -> Result<(), ()> {

            let old_vals = [0,1,2];
            let new_vals = [4,5,6];
            let mut p = PointND::new(old_vals);

            for i in 0..p.dims() {
                p.set(i, new_vals[i])?;
                assert_eq!(p[i], new_vals[i]);
            }

            Ok(())
        }

        #[test]
        fn getters_for_4d_points_work() {
            let arr = [0,1,2,3];
            let p = PointND::new(arr);

            assert_eq!(*p.x(), arr[0]);
            assert_eq!(*p.y(), arr[1]);
            assert_eq!(*p.z(), arr[2]);
            assert_eq!(*p.w(), arr[3]);
        }
        #[test]
        fn setters_for_4d_points_work() -> Result<(), ()> {

            let old_vals = [0,1,2,3];
            let new_vals = [4,5,6,7];
            let mut p = PointND::new(old_vals);

            for i in 0..p.dims() {
                p.set(i, new_vals[i])?;
                assert_eq!(p[i], new_vals[i]);
            }

            Ok(())
        }

    }

}
