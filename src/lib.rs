/*!

A simple multidimensional point struct, based on an array.

See the ```PointND``` struct for basic usage

 */

use std::ops::{Deref, DerefMut};


/**

The whole _point_ of the crate (get it?)

This is really just a wrapper around an array with convenience methods for accessing values if it's dimensions are within ```1..=4```

# Examples

## Constructing a Point

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
 */
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointND<T, const N: usize>([T; N])
    where T: Clone + Copy + Default;


impl<T, const N: usize> PointND<T, N>
    where T: Clone + Copy + Default {

    /**
     Returns a new ```PointND``` with values from the specified array or vector

     ### Panics

     If the length of the slice is zero
     */
    pub fn from(slice: &[T]) -> Self {
        if slice.len() == 0 {
            panic!("Cannot construct PointND with zero dimensions");
        }
        let arr: [T; N] = slice.try_into().unwrap();
        PointND(arr)
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

    pub fn apply_with<F>(self, other: PointND<T, N>, modifier: F) -> Result<Self, ()>
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


// Convenience Getters and Setters
/// ### 1D
/// Function for safely getting and setting the first value contained by a 1D ```PointND```
impl<T> PointND<T, 1>
    where T: Clone + Copy + Default  {

    pub fn x(&self) -> &T { &self[0] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }

}
/// ### 2D
/// Functions for safely getting and setting the first and second values contained by a 2D ```PointND```
impl<T> PointND<T, 2>
    where T: Clone + Copy + Default  {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }

}
/// ### 3D
/// Functions for safely getting and setting the first, second and third values contained by a 3D ```PointND```
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
/// Functions for safely getting and setting the first, second, third and fourth values contained by a 4D ```PointND```
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
mod test {

    use crate::*;

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