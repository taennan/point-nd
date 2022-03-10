/*!

A simple multidimensional point struct, based on an array.

See the ```PointND``` struct for basic usage

 */

use std::{
    ops::{Add, Sub, Mul, Div},
    slice::SliceIndex,
    convert::TryInto,
};
use std::ops::Index;

/**

The whole point of the crate (no pun intended)

# Examples

## Constructing a Point

No matter how a PointND is constructed, the second generic arg must be filled with the number of dimensions it needs to have

```
use point_nd::PointND;

// Creates a 3D point with all values set to 5
//  When using this function, complete type annotation is necessary
let p: PointND<i32, 3> = PointND::fill(5);

// Creates a 2D point from values of a given vector or array
let vec = vec![0, 1];
let p: PointND<_, 2> = PointND::from(&vec);
```

## Accessing Values
```
use point_nd::PointND;

let arr: [i32; 2] = [0,1];
let p: PointND<_, 2> = PointND::from(&arr);

// Safely
let x: Option<&i32> = p.get(0);
// Unsafely
let x: i32 = p[0];

assert_eq!(x, arr[0]);

// Depending on the number of dimensions of the point...
let x: i32 = p.x();
let y: i32 = p.y();

assert_eq!(y, arr[1]);
```

## Querying Size

```
use point_nd::PointND;

let p: PointND<i32, 2> = PointND::fill(10);
assert_eq!(p.dims(), 2);
```

 */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PointND<T, const N: usize>
    where T: Clone + Copy + Default {
    arr: [T; N],
}

impl<T, const N: usize>  PointND<T, N>
    where T: Clone + Copy + Default {

    pub fn from(slice: &[T]) -> Self {
        if slice.len() == 0 {
            panic!("Cannot construct Point with zero dimensions");
        }
        let arr: [T; N] = slice.try_into().unwrap();
        PointND { arr }
    }

    pub fn fill(value: T) -> Self {
        PointND::<T, N>::from(&[value; N])
    }


    pub fn dims(&self) -> usize {
        self.arr.len()
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.arr.get(i)
    }


    pub fn as_arr(&self) -> [T; N] {
        self.arr
    }

    pub fn as_vec(&self) -> Vec<T> {
        Vec::from(&self.arr[..])
    }

}

// Convenience Getters
impl<T: Clone + Copy + Default> PointND<T, 1> {

    pub fn x(&self) -> T { self.arr[0] }

}
impl<T: Clone + Copy + Default> PointND<T, 2> {

    pub fn x(&self) -> T { self.arr[0] }
    pub fn y(&self) -> T { self.arr[1] }

}
impl<T: Clone + Copy + Default> PointND<T, 3> {

    pub fn x(&self) -> T { self.arr[0] }
    pub fn y(&self) -> T { self.arr[1] }
    pub fn z(&self) -> T { self.arr[2] }

}
impl<T: Clone + Copy + Default> PointND<T, 4> {

    pub fn x(&self) -> T { self.arr[0] }
    pub fn y(&self) -> T { self.arr[1] }
    pub fn z(&self) -> T { self.arr[2] }
    pub fn w(&self) -> T { self.arr[3] }

}

// Basic operators
impl<T, const N: usize> Add for PointND<T, N> where T: Add<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if &self.dims() != &rhs.dims() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values= [T::default(); N];
        for i in 0..ret_values.len() {
            ret_values[i] = values_left[i] + values_right[i];
        }

        PointND::<T, N>::from(&ret_values)
    }

}
impl<T, const N: usize> Sub for PointND<T, N> where T: Sub<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if &self.dims() != &rhs.dims() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values= [T::default(); N];
        for i in 0..ret_values.len() {
            ret_values[i] = values_left[i] - values_right[i];
        }

        PointND::<T, N>::from(&ret_values)
    }

}
impl<T, const N: usize> Mul for PointND<T, N> where T: Mul<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if &self.dims() != &rhs.dims() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values= [T::default(); N];
        for i in 0..ret_values.len() {
            ret_values[i] = values_left[i] * values_right[i];
        }

        PointND::<T, N>::from(&ret_values)
    }

}
impl<T, const N: usize> Div for PointND<T, N> where T: Div<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if &self.dims() != &rhs.dims() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values= [T::default(); N];
        for i in 0..ret_values.len() {
            ret_values[i] = values_left[i] / values_right[i];
        }

        PointND::<T, N>::from(&ret_values)
    }

}

impl<I, T, const N: usize> Index<I> for PointND<T, N> where T: Clone + Copy + Default, I: Sized + SliceIndex<[T], Output = T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        &self.arr[index]
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
                assert_eq!(p.get(i).unwrap(), &vec[i]);
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
        fn cannot_index_out_of_bounds() {
            let p = PointND::<i32, 3>::from(&[0,1,2]);
            let _x = p[p.dims() + 1];
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