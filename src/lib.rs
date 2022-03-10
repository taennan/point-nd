/*!

A simple multidimensional point struct, based on an array.

See the ```PointND``` struct for basic usage

 */

use std::{
    ops::{
        Add, Sub, Mul, Div,
        Index, IndexMut
    },
    slice::SliceIndex,
    convert::TryInto,
};


/**

The whole _point_ of the crate (get it?)

It really is just a small wrapper around an array with convenience methods for accessing values if it's dimensions are ```1..=4```

# Examples

## Constructing a Point

No matter how a PointND is constructed, the second generic arg must be filled with the number of dimensions it needs to have

If a point of zero dimensions is constructed, it will panic

```
use point_nd::PointND;

// Creates a 3D point with all values set to 5
//  When using this function, complete type annotation is necessary
let p: PointND<i32, 3> = PointND::fill(5);

// Creates a 2D point from values of a given vector or array
let vec = vec![0, 1];
let p: PointND<_, 2> = PointND::from(&vec);

// ERROR:
// let p: PointND<_, 0> = PointND::fill(9);
```

## Accessing Values

It is recommended to use the convenience getters if the dimensions of the point are from ```1..=4```

```
use point_nd::PointND;

// A 2D point
let arr: [i32; 2] = [0,1];
let p: PointND<_, 2> = PointND::from(&arr);

// As the point has 2 dimensions, we can access it's values with the x() and y() methods
let x: i32 = p.x();
let y = p.y();

// If the point had 3 dimensions, we could use the above and:
// let z = p.z();

// Or 4:
// ...
// let w = p.w();

assert_eq!(y, arr[1]);
```

Otherwise indexing or the ```get()``` method can be used

```
use point_nd::PointND;

let arr: [i32; 2] = [0,1];
let p: PointND<_, 2> = PointND::from(&arr);

// Safely getting
//  Returns None if index is out of bounds
let x: Option<i32> = p.get(0);
assert_eq!(x.unwrap(), arr[0]);

// Unsafely indexing
//  If the index is out of bounds, this will panic
let y: i32 = p[1];
assert_eq!(y, arr[1]);
```

## Querying Size

The number of dimensions can be retrieved using the ```dims()``` method (short for _dimensions_)

```
use point_nd::PointND;

// A 2D point
let p: PointND<i32, 2> = PointND::fill(10);
assert_eq!(p.dims(), 2);
```

 */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PointND<T, const N: usize>
    where T: Clone + Copy {
    arr: [T; N],
}

impl<T, const N: usize>  PointND<T, N>
    where T: Clone + Copy {

    /**
     Returns a new ```PointND``` with values from the specified array or vector

     ### Panics

     If the length of the slice is zero
     */
    pub fn from(slice: &[T]) -> Self {
        if slice.len() == 0 {
            panic!("Cannot construct Point with zero dimensions");
        }
        let arr: [T; N] = slice.try_into().unwrap();
        PointND { arr }
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
     */
    pub fn dims(&self) -> usize {
        self.arr.len()
    }

    /**
     Returns the ```Some(value)``` at the specified dimension or ```None``` if the dimension is out of bounds

     The value of the first dimension is indexed at ```0``` for easier interoperability with standard indexing
     */
    pub fn get(&self, dim: usize) -> Option<T> {
        self.arr.get(dim).copied()
    }


    /**
     Returns a new ```PointND``` from the values contained by self after applying the modifier function to them

     ### Examples

     ```
     use point_nd::PointND;

     let p = PointND::<i32, 3>::from(&[0, 1, 2]);
     let p = p.apply(|item| item * 10);

     assert_eq!(p.as_arr(), [0, 10, 20]);
     ```
     */
    pub fn apply<F>(self, modifier: F) -> Self
        where F: Fn(T) -> T {

        let mut vec = Vec::<T>::with_capacity(N);
        for item in self.as_arr() {
            vec.push(modifier(item));
        }

        PointND::<T, N>::from(&vec)
    }

    /**
     Returns a new ```PointND``` from the values at the specified dimensions after applying the modifier function to them

     Any values at dimensions that were not specified are passed as is

     If any dimensions specified are out of bounds, this method will ignore it

    ### Examples

     ```
     use point_nd::PointND;

     let p = PointND::<i32, 4>::from(&[0, 1, 2, 3]);
     let p = p.apply_dims(&[1, 2], |item| item * 2);

     assert_eq!(p.as_arr(), [0, 2, 4, 3]);
     ```
     */
    pub fn apply_dims<F>(self, dims: &[usize], modifier: F) -> Self
        where F: Fn(T) -> T {

        let mut vec = Vec::<T>::with_capacity(N);
        for (i, item) in self.as_arr().into_iter().enumerate() {
            if dims.contains(&i) {
                vec.push(modifier(item));
            } else {
                vec.push(item);
            }
        }

        PointND::<T, N>::from(&vec)
    }

    /**
     Returns a new ```PointND``` from the values specified and those contained by self after applying the modifier to both

     ### Examples

     ```
     use point_nd::PointND;

     let p = PointND::<i32, 3>::from(&[0, 1, 2]);
     let p = p.apply_with([1, 2, 3], |a, b| a + b);

     assert_eq!(p.as_arr(), [1, 3, 5]);
     ```
     */
    pub fn apply_with<F>(self, values: [T; N], modifier: F) -> Self
        where F: Fn(T, T) -> T {

        let mut vec = Vec::<T>::with_capacity(N);
        for (a, b) in self.as_arr().into_iter().zip(values) {
            vec.push(modifier(a, b));
        }

        PointND::<T, N>::from(&vec)
    }


    /**
     Returns an array of all the values contained by the point
     */
    pub fn as_arr(&self) -> [T; N] {
        self.arr
    }

    /**
     Returns a vector of all the values contained by the point
     */
    pub fn as_vec(&self) -> Vec<T> {
        Vec::from(&self.arr[..])
    }

}

// Convenience Getters
/// Function for safely getting and setting the first value contained by a 1D ```PointND```
impl<T> PointND<T, 1> where T: Clone + Copy {

    pub fn x(&self) -> T { self.arr[0] }

    pub fn set_x(&mut self, new_value: T) { self.arr[0] = new_value; }

}
/// Functions for safely getting and setting the first and second values contained by a 2D ```PointND```
impl<T> PointND<T, 2> where T: Clone + Copy {

    pub fn x(&self) -> T { self.arr[0] }
    pub fn y(&self) -> T { self.arr[1] }

    pub fn set_x(&mut self, new_value: T) { self.arr[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self.arr[1] = new_value; }

}
/// Functions for safely getting and setting the first, second and third values contained by a 3D ```PointND```
impl<T> PointND<T, 3> where T: Clone + Copy {

    pub fn x(&self) -> T { self.arr[0] }
    pub fn y(&self) -> T { self.arr[1] }
    pub fn z(&self) -> T { self.arr[2] }

    pub fn set_x(&mut self, new_value: T) { self.arr[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self.arr[1] = new_value; }
    pub fn set_z(&mut self, new_value: T) { self.arr[2] = new_value; }

}
/// Functions for safely getting and setting the first, second, third and fourth values contained by a 4D ```PointND```
impl<T> PointND<T, 4> where T: Clone + Copy {

    pub fn x(&self) -> T { self.arr[0] }
    pub fn y(&self) -> T { self.arr[1] }
    pub fn z(&self) -> T { self.arr[2] }
    pub fn w(&self) -> T { self.arr[3] }

    pub fn set_x(&mut self, new_value: T) { self.arr[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self.arr[1] = new_value; }
    pub fn set_z(&mut self, new_value: T) { self.arr[2] = new_value; }
    pub fn set_w(&mut self, new_value: T) { self.arr[3] = new_value; }

}

// Basic math operators
impl<T, const N: usize> Add for PointND<T, N> where T: Add<Output = T> + Clone + Copy {

    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if &self.dims() != &rhs.dims() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values = Vec::<T>::with_capacity(N);
        for i in 0..N {
            ret_values.push(values_left[i] + values_right[i]);
        }

        PointND::<T, N>::from(&ret_values)
    }

}
impl<T, const N: usize> Sub for PointND<T, N> where T: Sub<Output = T> + Clone + Copy {

    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if &self.dims() != &rhs.dims() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values = Vec::<T>::with_capacity(N);
        for i in 0..N {
            ret_values.push(values_left[i] - values_right[i]);
        }

        PointND::<T, N>::from(&ret_values)
    }

}
impl<T, const N: usize> Mul for PointND<T, N> where T: Mul<Output = T> + Clone + Copy {

    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if &self.dims() != &rhs.dims() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values = Vec::<T>::with_capacity(N);
        for i in 0..N {
            ret_values.push(values_left[i] * values_right[i]);
        }

        PointND::<T, N>::from(&ret_values)
    }

}
impl<T, const N: usize> Div for PointND<T, N> where T: Div<Output = T> + Clone + Copy {

    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if &self.dims() != &rhs.dims() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values = Vec::<T>::with_capacity(N);
        for i in 0..N {
            ret_values.push(values_left[i] / values_right[i]);
        }

        PointND::<T, N>::from(&ret_values)
    }

}

// Indexing operators
impl<I, T, const N: usize> Index<I> for PointND<T, N> where T: Clone + Copy, I: Sized + SliceIndex<[T], Output = T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        &self.arr[index]
    }
}
impl<I, T, const N: usize> IndexMut<I> for PointND<T, N> where T: Clone + Copy, I: Sized + SliceIndex<[T], Output = T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.arr[index]
    }
}


#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod constructor {

        use crate::*;

        #[test]
        fn constructable_with_from_function() {
            let vec = vec![1,2,3,4];

            let _p = PointND::<_, 4>::from(&vec);
            let _p = PointND::<_, 3>::from(&vec[..3]);
        }

        #[test]
        fn constructable_with_fill_function() {

            #[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
            struct A {
                pub x: i32
            }
            impl A {
                pub fn new(x: i32) -> Self { A{ x } }
            }
            impl Add for A {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    A::new(self.x + rhs.x)
                }
            }

            let p = PointND::<A, 3>::fill(A::new(0));
            let p = p + PointND::from(&[
                A::new(1),
                A::new(2),
                A::new(3)
            ]);

            assert_ne!(p.x(), p.y());
            assert_ne!(p.x(), p.z());
            assert_ne!(p.y(), p.z());
        }

        #[test]
        #[should_panic]
        fn cant_construct_0_dim_point_with_from_function() {
            let _p = PointND::<u8, 0>::from(&[]);
        }
        #[test]
        #[should_panic]
        fn cant_construct_0_dim_point_with_fill_function() {
            let _p = PointND::<u8, 0>::fill(0);
        }

    }

    #[cfg(test)]
    mod dimensions {

        use crate::*;

        #[test]
        fn returns_correct_dimensions() {
            let vec = vec![0,1,2,3];
            let p = PointND::<_, 4>::from(&vec);

            assert_eq!(p.dims(), vec.len());
        }

    }

    #[cfg(test)]
    mod values {

        use crate::*;

        #[test]
        fn returns_value_on_get() {
            let vec = vec![0,1,2,3];
            let p = PointND::<_, 4>::from(&vec);

            for i in 0..vec.len() {
                assert_eq!(p.get(i).unwrap(), vec[i]);
            }
        }

        #[test]
        fn convenience_getters_for_1d_points_work() {
            let vec = vec![0];
            let p = PointND::<_, 1>::from(&vec);

            assert_eq!(p.x(), vec[0]);
        }
        #[test]
        fn convenience_getters_for_2d_points_work() {
            let vec = vec![0,1];
            let p = PointND::<_, 2>::from(&vec);

            assert_eq!(p.x(), vec[0]);
            assert_eq!(p.y(), vec[1]);
        }
        #[test]
        fn convenience_getters_for_3d_points_work() {
            let vec = vec![0,1,2];
            let p = PointND::<_, 3>::from(&vec);

            assert_eq!(p.x(), vec[0]);
            assert_eq!(p.y(), vec[1]);
            assert_eq!(p.z(), vec[2]);
        }
        #[test]
        fn convenience_getters_for_4d_points_work() {
            let vec = vec![0,1,2,3];
            let p = PointND::<_, 4>::from(&vec);

            assert_eq!(p.x(), vec[0]);
            assert_eq!(p.y(), vec[1]);
            assert_eq!(p.z(), vec[2]);
            assert_eq!(p.w(), vec[3]);
        }

    }

    #[cfg(test)]
    mod operators {

        use crate::*;

        #[test]
        #[should_panic]
        fn cannot_get_index_out_of_bounds() {
            let p = PointND::<i32, 3>::from(&[0,1,2]);
            let _x = p[p.dims() + 1];
        }

        #[test]
        fn can_set_value_by_index() {

            let arr = [0, 1, 2];
            let mut p = PointND::<_, 3>::from(&arr);

            let new_val = 9999;
            p[1] = new_val;

            assert_eq!(p.as_arr(), [0, new_val, 2]);
        }

        #[test]
        #[should_panic]
        fn cannot_set_out_of_bounds_value() {

            let arr = [0, 1, 2];
            let mut p = PointND::<_, 3>::from(&arr);

            let new_val = 9999;
            p[1002] = new_val;
        }

        #[test]
        fn can_add_two() {
            let vec = vec![0,1,2,3];
            let p1 = PointND::<_, 4>::from(&vec);
            let p2 = PointND::from(&vec);

            let p3 = p1 + p2;
            for (a, b) in p3.as_arr().into_iter().zip(vec){
                assert_eq!(a, b + b);
            }
        }

        #[test]
        fn can_subtract() {
            let vec = vec![0,1,2,3];
            let p1 = PointND::<_, 4>::from(&vec);
            let p2 = PointND::from(&vec);

            let p3 = p1 - p2;
            for (a, b) in p3.as_arr().into_iter().zip(vec){
                assert_eq!(a, b - b);
            }
        }

        #[test]
        fn can_multiply() {
            let vec = vec![0,1,2,3];
            let p1 = PointND::<_, 4>::from(&vec);
            let p2 = PointND::from(&vec);

            let p3 = p1 * p2;
            for (a, b) in p3.as_arr().into_iter().zip(vec){
                assert_eq!(a, b * b);
            }
        }

        #[test]
        fn can_divide() {
            let vec = vec![1,2,3,4];
            let p1 = PointND::<_, 4>::from(&vec);
            let p2 = PointND::from(&vec);

            let p3 = p1 / p2;
            for (a, b) in p3.as_arr().into_iter().zip(vec){
                assert_eq!(a, b / b);
            }
        }

        #[test]
        #[should_panic]
        fn cannot_divide_if_one_item_is_zero() {
            let vec = vec![0, 1,2,3,4];
            let p1 = PointND::<_, 5>::from(&vec);
            let p2 = PointND::from(&vec);

            let p3 = p1 / p2;
            for (a, b) in p3.as_arr().into_iter().zip(vec){
                assert_eq!(a, b / b);
            }
        }

        #[test]
        fn can_equal() {
            let vec = vec![1,2,3,4];
            let p1 = PointND::<_, 4>::from(&vec);
            let p2 = PointND::from(&vec);

            assert_eq!(p1, p2);
        }

        #[test]
        fn can_not_equal() {
            let vec1 = vec![1,2,3,4];
            let p1 = PointND::<_, 4>::from(&vec1);
            let vec2 = vec![5,6,7,8];
            let p2 = PointND::from(&vec2);

            assert_ne!(p1, p2);
        }

    }

}