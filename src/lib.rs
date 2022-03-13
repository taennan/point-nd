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

The whole _point_ of the crate

This is really just a wrapper around an array with convenience methods for accessing values if it's dimensions are within ```1..=4```

Therefore, all methods implemented for arrays are available with this

# Making a Point

```
# use point_nd::PointND;
// Creating a 2D point from a given array
let arr: [i32; 2] = [0,1];
let p: PointND<_, 2> = PointND::new(arr);

// Creating a 3D point from values of a given slice
let vec: Vec<i32> = vec![0, 1, 2];
let p: PointND<_, 3> = PointND::from(&vec);

// Creating a 4D point with all values set to 5
let p: PointND<i32, 4> = PointND::fill(5);

// The second generic arg is a usize constant and for the ::fill()
//  and ::from() functions, specifying it is usually necessary
// If you don't like writing PointND twice for type annotation,
//  use FQS (fully qualified syntax) instead:
let p = PointND::<_, 4>::fill(5);

// Trying to create a PointND with zero dimensions using the above will panic at runtime
//  ERROR: Can't create a point with zero dimensions
//  let p: PointND<_, 0> = PointND::new([]);
```

# Getting and Setting Values

If the dimensions of the point are within ```1..=4```, it is recommended to use the convenience getters and setters

```
# use point_nd::PointND;
let arr = [0,1];
let p: PointND<_, 2> = PointND::new(arr);

// As the point has 2 dimensions, we can access
//  it's values with the x() and y() methods
let x: &i32 = p.x();
assert_eq!(x, &arr[0]);
let y = p.y();
assert_eq!(*y, arr[1]);

// If the point had 3 dimensions, we could use the above and:
//  let z = p.z();
// Or with 4 dimensions, the above and:
//  let w = p.w();

// Setting values is just as simple
let mut p = PointND::new(arr);
p.set_x(101);
assert_eq!(*p.x(), 101);

// As with the getters, there are respective methods for setting the
//  values at y, z and w depending on the dimensions of the point
```

Alternatively, since ```PointND``` implements ```Deref```, all methods of getting and setting array elements can work as well

These are the only methods which can be used for ```PointND```'s with dimensions *not* within ```1..=4```

```
# use point_nd::PointND;
# let arr: [i32; 2] = [101,1];
# let mut p: PointND<_, 2> = PointND::new(arr);
// Exactly like safely accessing an array
let x: Option<&i32> = p.get(0);
assert_eq!(*x.unwrap(), 101);

// Exactly like indexing an array
//  Note: Unlike other accessing methods, this returns a copy of the value
let y: i32 = p[1];
assert_eq!(y, arr[1]);

// Setting via indexing
p[1] = 345;
assert_eq!(p[1], 345);
```

# Querying Size

The number of dimensions can be retrieved using the ```dims()``` method (short for _dimensions_)

```
# use point_nd::PointND;
let p: PointND<i32, 2> = PointND::new([0,1]);
assert_eq!(p.dims(), 2);
// Alternatively, as PointND implements Deref:
assert_eq!(p.len(), 2)
```

# Transforming Points

### Appliers

The ```apply```, ```apply_vals```, ```apply_dims``` and ```apply_point``` (henceforth referred to as _appliers_)
methods all consume self and return a new point after applying a function to all contained values

Multiple appliers can be chained together to make complex transformations to a ```PointND```

This is probably best explained with an example:

```
# use point_nd::PointND;
# fn apply_example() -> Result<(), ()> {
// A trivial transformation more easily done via other methods...
//  but it gets the point across
let p = PointND
    ::new([0,1,2])                      // Creates a new PointND
    .apply(|item| Ok( item + 2 ))?      // Adds 2 to each item
    .apply(|item| Ok( item * 3 ))?;     // Multiplies each item by 3
assert_eq!(p.into_arr(), [6, 9, 12]);
# Ok(())
# }
```

### Creating a Function to Pass to Appliers

The function or closure passed to the applier methods (henceforth referred to as _modifiers_)
accept either one or two args of type ```T``` (where ```T``` is the type of the items contained
by the point) depending on whether one or two sets of values are being modified.

Modifiers must all return a ```Result<T, ()>``` to allow graceful error handling by the applier instead of just panicking.

If an ```Err``` is returned by the modifier when called on any item, the applier returns an ```Err(())```

If all goes well, the applier returns an ```Ok``` with the new ```PointND``` as it's value

Hopefully the above wasn't confusing, but here's an example just in case:

```
# use point_nd::PointND;
# fn modifier_creation_example() -> Result<(), ()> {
// Dividing by zero causes a runtime error, so we return an Err if the second arg is zero
let divide_items = |a: f32, b: f32| -> Result<f32, ()> {
    if b == 0.0 {
        Err(())
    } else {
        Ok( a / b )
    }
};

let p1 = PointND::new([-1.2, 2.0, -3.0, 4.5]);
let p2 = PointND::new([2.3,  9.0, -3.0, 1.0]);
let zero_point = PointND::fill(0.0);

// Divides each item in p1 with each in p2
let result = p1.clone().apply_point(p2, divide_items);
// No zeros in p2, so everything's Ok
assert!(result.is_ok());

// Divides each item in p1 with each in zero_point
let result = p1.apply_point(zero_point, divide_items);
// Error is thrown by divide_items, causing apply_point() to throw error
assert!(result.is_err());
# Ok(())
# }
```
 */
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointND<T, const N: usize>([T; N])
    where T: Clone + Copy + Default;

impl<T, const N: usize> PointND<T, N>
    where T: Clone + Copy + Default {

    /**
     Returns a new ```PointND``` with values from the specified array

     This is the only constructor that does not need ever type annotation

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


    /**
     Consumes ```self``` and calls the ```modifier``` on each item contained by ```self``` to create a new ```PointND```
     ```
     # use point_nd::PointND;
     # fn apply_example() -> Result<(), ()> {
     let p = PointND
         ::new([0,1,2])                    // Creates a new PointND
         .apply(|item| Ok( item + 2 ))?    // Adds 2 to each item
         .apply(|item| Ok( item * 3 ))?;   // Multiplies each item by 3
     assert_eq!(p.into_arr(), [6, 9, 12]);
     # Ok(())
     # }
     ```
     */
    // Did not call apply_dims() inside this to avoid the dimension checks it does
    pub fn apply<F>(self, modifier: F) -> Result<Self, ()>
        where F: Fn(T) -> Result<T, ()> {

        let mut arr = [T::default(); N];
        for i in 0..N {
            arr[i] = modifier(self[i])?;
        }

        Ok( PointND::<T, N>::from(&arr) )
    }

    /**
     Consumes ```self``` and calls the ```modifier``` the items at specified ```dims``` contained by ```self``` to create a new ```PointND```

     Any items at dimensions not specified will be passed to the new point as is
     ```
     # use point_nd::PointND;
     # fn apply_dims_example() -> Result<(), ()> {
     let p = PointND
         ::new([0,1,2,3])                              // Creates a new PointND
         .apply_dims(&[1,3], |item| Ok( item * 2  ))?  // Multiplies items 1 and 3 by 2
         .apply_dims(&[0,2], |item| Ok( item + 10 ))?; // Adds 10 to items 0 and 2
     assert_eq!(p.into_arr(), [10, 2, 20, 6]);
     # Ok(())
     # }
     ```
     */
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

    /**
     Consumes ```self``` and calls the ```modifier``` on each item contained
     by ```self``` and ```values``` to create a new ```PointND```

     When creating a modifier function to be used by this method, keep in mind that the items in
     ```self``` are passed to it through the **first arg**, and the items in ```value``` through the **second**
     ```
     # use point_nd::PointND;
     # fn apply_vals_example() -> Result<(), ()> {
     let p = PointND
         ::new([0,1,2])                             // Creates a new PointND
         .apply_vals([1,3,5], |a, b| Ok( a + b ))?  // Adds items in point to items in array
         .apply_vals([2,4,6], |a, b| Ok( a * b ))?; // Multiplies items in point
                                                    //  to items in array
     assert_eq!(p.into_arr(), [2, 16, 42]);
     # Ok(())
     # }
     ```
     */
    pub fn apply_vals<F>(self, values: [T; N], modifier: F) -> Result<Self, ()>
        where F: Fn(T, T) -> Result<T, ()> {

        let mut arr = [T::default(); N];
        for i in 0..N {
            arr[i] = modifier(self[i], values[i])?;
        }

        Ok( PointND::<T, N>::from(&arr) )
    }

    /**
     Consumes ```self``` and calls the ```modifier``` on each item contained by ```self``` and another ```PointND``` to create a new point

     When creating a modifier function to be used by this method, keep in mind that the items in
     ```self``` are passed to it through the **first arg**, and the items in ```other``` through the **second**
     ```
     # use point_nd::PointND;
     # fn apply_point_example() -> Result<(), ()> {
     let p1 = PointND::new([0,9,3,1]);
     let p2 = PointND::fill(10);
     let p3 = PointND
         ::new([1,2,3,4])                         // Creates a new PointND
         .apply_point(p1, |a, b| Ok ( a - b ))?   // Subtracts items in p3 with those in p1
         .apply_point(p2, |a, b| Ok ( a * b ))?;  // Multiplies items in the new point returned
                                                  //  with the items in p2
     assert_eq!(p3.into_arr(), [10, -70, 0, 30]);
     # Ok(())
     # }
     ```
     */
    pub fn apply_point<F>(self, other: Self, modifier: F) -> Result<Self, ()>
        where F: Fn(T, T) -> Result<T, ()> {

        self.apply_vals(other.into_arr(), modifier)
    }


    /// Consumes ```self```, returning the contained array
    pub fn into_arr(self) -> [T; N] {
        self.0
    }

    /// Consumes ```self```, returning the contained array as a vector
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


/**
 Converts an identifier _x_, _y_, _z_ or _w_ to a usize value for indexing

 Using any identifier apart from the above or multiple identifiers will result in a compile time error

 It is recommended to use parentheses when calling this macro for clarity

 ```
 # use point_nd::dim;
 let x_index: usize = dim!(x);   // Expands to 0usize
 let y_index = dim!(y);          // Expands to 1usize

 // ERROR: Only allowed to use one of x, y, z or w
 // let fifth_dimension = dim!(v);
 ```

 This can be especially useful for indexing a ```PointND``` (or any collection indexable with a usize)

 If a dimension is passed that is out of bounds, it will result in a compile time error

 ```
 # use point_nd::{dim, PointND};
 let p = PointND::new([0,1,2]);
 let y_val = p[dim!(y)];
 assert_eq!(y_val, 1);

 // ERROR: Index out of bounds
 // let w_val = p[dim!(w)];
 ```

 */
#[macro_export]
macro_rules! dim {

    (x) => { 0usize };
    (y) => { 1usize };
    (z) => { 2usize };
    (w) => { 3usize };

}

/**
 Converts an array of identifiers to an array of usize values
 */
#[macro_export]
macro_rules! dims {

    ( $( $d:ident ), * ) => { [ $( dim!($d), )* ] };

}

/**
 Converts a range of identifiers and expressions to a range of usize values
 */
#[macro_export]
macro_rules! dimr {

    // Identity to Identity
    //  x..w
    ( $a:ident..$b:ident ) => { dim!($a)..dim!($b) };
    //  y..=z
    ( $a:ident..=$b:ident ) => { dim!($a)..=dim!($b) };

    // Identity to Expression
    //  z...6
    ( $a:ident..$b:expr ) => { dim!($a)..$b };
    //  w..=9
    ( $a:ident..=$b:expr ) => { dim!($a)..=$b };

    // Infinity to Identifier
    //  ..w
    ( ..$a:ident ) => { ..dim!($a) };
    // ..=z
    ( ..=$a:ident ) => { ..=dim!($a) };

    // Identifier to Infinity
    //  x..
    ( $a:ident.. ) => { dim!($a).. };

}



#[cfg(test)]
mod tests {
    use crate::*;

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
    mod get_and_set {
        use super::*;

        #[test]
        fn can_get_slice_by_range_index() {
            let p = PointND::new([0,1,2,3,4]);
            let slice = &p[0..3];
            assert_eq!(*slice, [0,1,2]);
        }

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

        #[test]
        fn cannot_set_out_of_bounds_index() {

            let arr = [0,-1,2,-3];
            let mut p = PointND::new(arr);

            let err = p.set(100, 100);

            assert_eq!(err, Err(()));
            assert_eq!(p.into_arr(), arr);
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

    #[cfg(test)]
    mod macros {
        use super::*;

        #[test]
        #[allow(unused_variables)]
        fn ident_wont_access_var() {
            let x = 2usize;
            assert_eq!(dim!(x), 0);
        }

        #[test]
        fn dim_works() {
            assert_eq!(dim!(x), 0);
            assert_eq!(dim!(y), 1);
            assert_eq!(dim!(z), 2);
            assert_eq!(dim!(w), 3);

            let p = PointND::new([-2,-1,0,1,2]);
            assert_eq!(p[dim!(x)], -2);
            assert_eq!(p[dim!(y)], -1);
            assert_eq!(p[dim!(z)], 0);
            assert_eq!(p[dim!(w)], 1);
        }

        #[test]
        fn dims_works() {
            assert_eq!(dims![x,y,z,w], [0,1,2,3]);
            assert_eq!(dims![x,z,y],   [0,2,1]);

            let p = PointND
                ::new([0,1,2])
                .apply_dims(&dims![x,y], |item| Ok( item + 10 ))
                .unwrap();
            assert_eq!(p.into_arr(), [10, 11, 2]);
        }
        #[test]
        fn can_repeat_identifier_in_dims() {
            assert_eq!(dims![x,x,x], [0,0,0]);
            assert_eq!(dims![x,y,x], [0,1,0]);
        }

        #[test]
        fn dimr_ident_to_ident_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[dimr![x..z]];
            assert_eq!(*slice, [0,1]);
        }
        #[test]
        fn dimr_ident_to_eq_ident_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[dimr![y..=w]];
            assert_eq!(*slice, [1,2,3]);
        }

        #[test]
        fn dimr_ident_to_expr_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[dimr![y..9]];
            assert_eq!(*slice, [1,2,3,4,5,6,7,8]);
        }
        #[test]
        fn dimr_ident_to_eq_expr_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[dimr![x..=5]];
            assert_eq!(*slice, [0,1,2,3,4,5]);
        }

        #[test]
        fn dimr_inf_to_ident_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[dimr![..w]];
            assert_eq!(*slice, [0,1,2]);
        }
        #[test]
        fn dimr_inf_to_eq_ident_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[dimr![..=w]];
            assert_eq!(*slice, [0,1,2,3]);
        }

        #[test]
        fn dimr_ident_to_inf_works() {
            let arr = [0,1,2,3,4,5,6,7,8,9];
            let slice = &arr[dimr![x..]];
            assert_eq!(*slice, arr);
        }

    }

}
