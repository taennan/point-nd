#![no_std]
/*!

A simple and flexible multidimensional point struct, based on an array.

This crate uses constant generics, it is recommended for use with a Rust version **>= 1.51**.

No imports are made from the standard library, so this crate is ```#![no_std]``` compatible

See the ```PointND``` struct for basic usage

 */

use core::ops::{ Deref, DerefMut, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg };


/**

The whole _point_ of the crate.

The ```PointND``` struct is really just a boxed array with
convenience methods for accessing, setting and transforming values.

Therefore, all methods implemented for arrays are available with this.

# Making a Point

There are three ```PointND``` constructors, ```new```, ```from_slice``` and ```fill```.

The ```from_slice``` and ```fill``` functions can only be used
if creating a point where the items implement ```Copy```

```
# use point_nd::PointND;
// Creating a 2D point from a given array
let arr = [0, 1];
let p: PointND<i32, 2> = PointND::new(arr);

// Creating a 3D point from values of a given slice
let vec: Vec<i32> = vec![0, 1, 2];
let p: PointND<_, 3> = PointND::from_slice(&vec);

// Creating a 4D point with all values set to 5
let p: PointND<i32, 4> = PointND::fill(5);

// Trying to create a PointND with zero dimensions using
//  any of the above will panic at runtime
// ERROR: Can't create a point with zero dimensions
// let p: PointND<_, 0> = PointND::new([]);
```

The second generic arg is a ```usize``` constant generic and for the ```fill()```
and ```from_slice()``` functions, specifying it is sometimes necessary when the
compiler cannot infer it itself.

See their documentation for cases when explicit types are not necessary

Otherwise, if you don't like writing ```PointND``` twice for type
annotation, use FQS (_fully qualified syntax_) instead:

```
# use point_nd::PointND;
let p1 = PointND::<_, 4>::from_slice(&vec![5,5,5,5]);
let p2 = PointND::<_, 4>::fill(5);

assert_eq!(p1, p2);
```

# Getting Values

If the dimensions of the point are within ```1..=4```, it is recommended to
use the convenience getters ```x()```, ```y()```, ```z()``` and ```w()```

The above all return references to the value, regardless of whether they implement ```Copy```

```
# use point_nd::PointND;
let p = PointND::new([0,1]);

// As the point has 2 dimensions, we can access
//  it's values with the x() and y() methods
let x: &i32 = p.x();
assert_eq!(*x, 0);
let y = p.y();
assert_eq!(*y, 1);

// If the point had 3 dimensions, we could use the above and:
//  let z = p.z();
// Or with 4 dimensions, the above and:
//  let w = p.w();
```

The above methods are not implemented for ```PointND```'s with more than 4 dimensions.

Instead, we must use the native implementations of the contained array

```rust
# use point_nd::PointND;
# use point_nd::{dim, dimr};
let p = PointND::new([0,1,2,3,4,5]);

// ERROR: Not implemented for PointND of 6 dimensions
// let x = p.x();

// Indexing
let x: i32 = p[0];
assert_eq!(x, 0);

// Safely Getting
let y: Option<&i32> = p.get(1);
assert_eq!(*y.unwrap(), 1);

// Slicing
let z_to_last: &[i32] = &p[2..];
assert_eq!(z_to_last, [2,3,4,5]);

// The dimension macros provided by this crate can make
//  direct indexing easier and more readable
// See their documentation for more info
let w = p[dim!(z)];
let the_rest = &p[dimr!(w..)];
```

To get **all** the values contained by a point, use the ```into_arr()``` method

```
# use point_nd::PointND;
let p = PointND::new([-10, -2, 0, 2, 10]);
assert_eq!(p.into_arr(), [-10, -2, 0, 2, 10])
```

# Querying Size

The number of dimensions can be retrieved using
the ```dims()``` method (short for _dimensions_)

```
# use point_nd::PointND;
let p: PointND<i32, 2> = PointND::new([0,1]);
assert_eq!(p.dims(), 2);

// Alternatively, as PointND implements Deref, we can use len().
// It's not as descriptive however...
assert_eq!(p.len(), 2);
```

# Transforming Values

If the dimensions of the point are within ```1..=4```,
it is recommended to use the convenience setters

```
# use point_nd::PointND;
let mut p = PointND::new([0, 1]);

// As the point has 2 dimensions, we can set
//  it's values with the set_x() and set_y() methods
// There are set_z() and set_w() methods available for
//  points with 3 and 4 dimensions respectively
p.set_x(-10);
p.set_y(-20);

assert_eq!(*p.x(), -10);
assert_eq!(*p.y(), -20);
```

The above methods are not implemented for ```PointND```'s with more than 4 dimensions.

Instead, we must use the native implementations of the contained array

```
# use point_nd::PointND;
# use point_nd::dim;
let mut p = PointND::new([0, 1]);

// Sets x via indexing
p[0] = -100;
assert_eq!(*p.x(), -100);

// Sets y via indexing and dimension macros
p[dim!(y)] = -100;
assert_eq!(*p.x(), *p.y());
```

The basic arithmetic and arithmetic assign operations
(```Neg```, ```Add```, ```AddAssign```, _etc_) are also available

```
# use point_nd::PointND;
let p = PointND::new([-1, 0, 1]);

// Neg
let neg_p = -p.clone();
assert_eq!(neg_p.into_arr(), [1, 0, -1]);

// Sub
let p_diff = p.clone() - p.clone();
assert_eq!(p_diff.into_arr(), [0, 0, 0]);

// MulAssign
let p_mul = p.clone() * p.clone();
assert_eq!(p_mul.into_arr(), [1, 0, 1]);

// ...and etc.
```

### Appliers

The ```apply```, ```apply_vals```, ```apply_dims``` and ```apply_point``` methods all
consume self and return a new point after calling a function or closure on all contained values

Multiple applier methods can be chained together to make complex transformations to a ```PointND```

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

The function or closure passed to the applier methods must all return a ```Result<T, ()>```
(where ```T``` is the type of the items contained by the point) to allow graceful error
handling by the applier instead of just panicking.

If an ```Err(())``` is thrown by the closure when called
on any item, the applier also returns ```Err(())```

If all goes well, the applier returns an ```Ok``` with the new ```PointND``` as it's value

Hopefully the above wasn't confusing, but here's an example just in case:

```
# use point_nd::PointND;
# fn modifier_creation_example() -> Result<(), ()> {
// Dividing by zero causes a runtime error,
//  so we return an Err if the second arg is zero
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
// Error is thrown by divide_items, causing apply_point() to throw
assert!(result.is_err());
# Ok(())
# }
```

# Iterating

Iterating over a ```PointND``` is as easy as:

```rust
# use point_nd::PointND;
let mut p = PointND::new([0,1]);

for _ in p.iter()      { /* Do stuff     */ }
for _ in p.iter_mut()  { /* Change stuff */ }
for _ in p.into_iter() { /* Move stuff (unless items implement Copy) */ }
```

It must be noted that if the items implement ```Copy```, using
```into_iter()``` will not actually move the point out of scope.

If this behaviour is necessary, use the ```into_arr()``` method
to consume the point and move the contained array into the loop

```
# use point_nd::PointND;
# let mut p = PointND::new([0,1]);
for _ in p.into_arr().into_iter() { /* Move stuff */ }

// ERROR: Can't access moved value
// assert_eq!(p.dims(), 2);
```
 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointND<T, const N: usize>([T; N]);

impl<T, const N: usize> PointND<T, N> {

    /**
     Returns a new ```PointND``` with values from the specified array

     This is the only constructor that does not ever need type annotation

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
     Returns the number of dimensions of the point (a 2D point will return 2, a 3D point 3, _etc_)

     Equivalent to calling ```len()```
     */
    pub fn dims(&self) -> usize {
        self.0.len()
    }

    /// Consumes ```self```, returning the contained array
    pub fn into_arr(self) -> [T; N] {
        self.0
    }

}

impl<T, const N: usize> PointND<T, N>
    where T: Clone + Copy {

    /**
     Returns a new ```PointND``` with values from the specified slice

     This constructor is probably only useful when ```Vec```'s of unknown length are
     the only collections available

     If the compiler is not able to infer the dimensions (a.k.a - length)
     of the point, it needs to be explicitly specified

     ```
     # use point_nd::PointND;
     // Explicitly specifying dimensions
     let p = PointND::<_, 3>::from_slice(&vec![0,1,2]);

     // The generics don't always have to be specified though, for example
     let p1 = PointND::new([0,1]);       // Compiler knows this has 2 dimensions
     let p2 = PointND::from_slice(&vec![2,3]);

     // Later, p2 is added to p1. The compiler is able to infer its dimensions
     let p = p1 + p2;
     ```

     ### Panics

     If the length of the slice is zero

     If the length of the slice is not equal to the dimensions specified by the constant generic

     If the slice passed cannot be converted into an array
     */
    pub fn from_slice(slice: &[T]) -> Self {
        let arr: [T; N] = slice.try_into().unwrap();
        PointND::new(arr)
    }

    /**
     Returns a new ```PointND``` with all values set as specified

     If the compiler is not able to infer the dimensions (a.k.a - length)
     of the point, it needs to be explicitly specified

     See the ```from()``` function for cases when generics don't need to be explicitly specified

     ```
     # use point_nd::PointND;
     // A point with 10 dimensions with all values set to 2
     let p = PointND::<_, 10>::fill(2);

     assert_eq!(p.dims(), 10);
     for i in p.into_iter() {
        assert_eq!(i, 2);
     }
     ```

     ### Panics

     If the dimensions of the point being constructed is zero
     */
    pub fn fill(value: T) -> Self {
        PointND::<T, N>::from_slice(&[value; N])
    }

}

impl<T, const N: usize> PointND<T, N>
    where T: Clone {

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
    pub fn apply<F>(mut self, modifier: F) -> Result<Self, ()>
        where F: Fn(T) -> Result<T, ()> {

        for i in 0..N {
            self[i] = modifier(self[i].clone())?;
        }

        Ok( self )
    }

    /**
     Consumes ```self``` and calls the ```modifier``` on the items at the specified ```dims``` to create a new ```PointND```

     Any items at dimensions not specified will be passed to the new point without change
     ```
     # use point_nd::PointND;
     # fn apply_dims_example() -> Result<(), ()> {
     let p = PointND
         ::new([0,1,2,3,4])                            // Creates a new PointND
         .apply_dims(&[1,3], |item| Ok( item * 2  ))?  // Multiplies items 1 and 3 by 2
         .apply_dims(&[0,2], |item| Ok( item + 10 ))?; // Adds 10 to items 0 and 2
     assert_eq!(p.into_arr(), [10, 2, 20, 6, 4]);
     # Ok(())
     # }
     ```
     */
    pub fn apply_dims<F>(mut self, dims: &[usize], modifier: F) -> Result<Self, ()>
        where F: Fn(T) -> Result<T, ()> {

        for i in 0..N {
            if dims.contains(&i) {
                self[i] = modifier(self[i].clone())?;
            }
        }

        Ok( self )
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
    pub fn apply_vals<F, X>(mut self, values: [X; N], modifier: F) -> Result<Self, ()>
        where F: Fn(T, X) -> Result<T, ()>,
              X: Clone {

        for i in 0..N {
            self[i] = modifier(self[i].clone(), values[i].clone())?;
        }

        Ok( self )
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
         .apply_point(p2, |a, b| Ok ( a * b ))?;  // Multiplies items in the returned point
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

}


// Deref
impl<T, const N: usize> Deref for PointND<T, N> {

    type Target = [T; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }

}
impl<T, const N: usize> DerefMut for PointND<T, N> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }

}


// Math operators
//  Negation
impl<T, const N: usize> Neg for PointND<T, N>
    where T: Clone + Neg<Output = T> {

    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut arr = self.into_arr();
        for i in 0..N {
            arr[i] = -arr[i].clone();
        }

        PointND::new(arr)
    }

}

//  Arithmetic
impl<T, const N: usize> Add for PointND<T, N>
    where T: Add<Output = T> + Clone  {

    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut arr = self.into_arr();
        for i in 0..N {
            arr[i] = arr[i].clone() + rhs[i].clone();
        }

        PointND::new(arr)
    }

}
impl<T, const N: usize> Sub for PointND<T, N>
    where T: Sub<Output = T> + Clone {

    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut arr = self.into_arr();
        for i in 0..N {
            arr[i] = arr[i].clone() - rhs[i].clone();
        }

        PointND::new(arr)
    }

}
impl<T, const N: usize> Mul for PointND<T, N>
    where T: Mul<Output = T> + Clone {

    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut arr = self.into_arr();
        for i in 0..N {
            arr[i] = arr[i].clone() * rhs[i].clone();
        }

        PointND::new(arr)
    }

}
/**
 ### Warning

 Dividing by a ```PointND``` that contains a zero will cause a panic
 */
impl<T, const N: usize> Div for PointND<T, N>
    where T: Div<Output = T> + Clone {

    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut arr = self.into_arr();
        for i in 0..N {
            arr[i] = arr[i].clone() / rhs[i].clone();
        }

        PointND::new(arr)
    }

}

//  Arithmetic Assign
impl<T, const N: usize> AddAssign for PointND<T, N>
    where T: AddAssign + Clone {

    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] += rhs[i].clone();
        }
    }

}
impl<T, const N: usize> SubAssign for PointND<T, N>
    where T: SubAssign + Clone {

    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] -= rhs[i].clone();
        }
    }

}
impl<T, const N: usize> MulAssign for PointND<T, N>
    where T: MulAssign + Clone {

    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] *= rhs[i].clone();
        }
    }

}
/**
 ### Warning

 Dividing by a ```PointND``` that contains a zero will cause a panic
 */
impl<T, const N: usize> DivAssign for PointND<T, N>
    where T: DivAssign + Clone {

    fn div_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] /= rhs[i].clone();
        }
    }

}


// Convenience Getters and Setters
/// Functions for safely getting and setting the value contained by a 1D ```PointND```
impl<T> PointND<T, 1>
    where T: Clone {

    pub fn x(&self) -> &T { &self[0] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }

}
/// Functions for safely getting and setting the values contained by a 2D ```PointND```
impl<T> PointND<T, 2>
    where T: Clone {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }

}
/// Functions for safely getting and setting the values contained by a 3D ```PointND```
impl<T> PointND<T, 3>
    where T: Clone {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }
    pub fn z(&self) -> &T { &self[2] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }
    pub fn set_z(&mut self, new_value: T) { self[2] = new_value; }

}
/// Functions for safely getting and setting the values contained by a 4D ```PointND```
impl<T> PointND<T, 4>
    where T: Clone {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }
    pub fn z(&self) -> &T { &self[2] }
    pub fn w(&self) -> &T { &self[3] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }
    pub fn set_z(&mut self, new_value: T) { self[2] = new_value; }
    pub fn set_w(&mut self, new_value: T) { self[3] = new_value; }

}


// Convenience Shifters
impl<T> PointND<T, 1>
    where T: Clone + AddAssign {

    pub fn shift_x(&mut self, delta: T) { self[0] += delta; }

}
impl<T> PointND<T, 2>
    where T: Clone + AddAssign {

    pub fn shift_x(&mut self, delta: T) { self[0] += delta; }
    pub fn shift_y(&mut self, delta: T) { self[1] += delta; }

}
impl<T> PointND<T, 3>
    where T: Clone + AddAssign {

    pub fn shift_x(&mut self, delta: T) { self[0] += delta; }
    pub fn shift_y(&mut self, delta: T) { self[1] += delta; }
    pub fn shift_z(&mut self, delta: T) { self[2] += delta; }

}
impl<T> PointND<T, 4>
    where T: Clone + AddAssign {

    pub fn shift_x(&mut self, delta: T) { self[0] += delta; }
    pub fn shift_y(&mut self, delta: T) { self[1] += delta; }
    pub fn shift_z(&mut self, delta: T) { self[2] += delta; }
    pub fn shift_w(&mut self, delta: T) { self[3] += delta; }

}


// Dimension Macros
/**
 Converts an identifier _x_, _y_, _z_ or _w_ to a ```usize``` value for indexing collections.

 Using any identifier apart from the above or multiple identifiers will result in a compile time error.

 It is recommended to use parentheses when calling this macro for clarity.

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dim;
 let first: usize = dim!(x);
 let second = dim!(y);
 let third  = dim!(z);
 let fourth = dim!(w);

 assert_eq!(first,  0);
 assert_eq!(second, 1);
 assert_eq!(third,  2);
 assert_eq!(fourth, 3);

 // ERROR: Only allowed to use one of x, y, z or w
 // let fifth_dimension = dim!(v);

 // ERROR: Only accepts one identifier
 //        If multiple dimensions are what you need, see the 'dims' macro
 // let third_and_fourth = dim!(z, w);
 # }
 ```

 This can be especially useful for indexing a ```PointND```.

 If a dimension is passed that is out of bounds, it will result in a compile time error.

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dim;
 # use point_nd::PointND;
 let p = PointND::new([0,1,2]);
 let y = p[dim!(y)];
 assert_eq!(y, 1);

 // ERROR: Index out of bounds
 // let w_val = p[dim!(w)];
 # }
 ```

 The dimensions of the point being indexed don't necessarily have to be within ```x..=w```

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dim;
 # use point_nd::PointND;
 // Works with points of any dimensions
 let five_d_point = PointND::new([0,1,2,3,4]);
 let z = five_d_point[dim!(z)];
 assert_eq!(z, 2);
 # }
 ```

 This macro is not just limited to ```PointND```'s though.

 Any collection that accepts a ```usize``` for indexing is compatible with this macro

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dim;
 let array = [0,1,2,3];
 let first_item = array[dim!(x)];
 let second     = array[dim!(y)];
 // ...etc
 # assert_eq!(first_item, 0);
 # assert_eq!(second, 1);
 # }
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
 Converts an array of identifiers _x_, _y_, _z_ or _w_ to an array of ```usize``` values

 Using any identifier or expression apart from the above will result in a compile time error

 It is recommended to use square brackets when calling this macro for clarity

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dims;
 let dim_arr: [usize; 4] = dims![
     x,  // 0usize
     y,  // 1
     z,  // 2
     w   // 3
 ];

 // Using identifiers multiple times is allowed,
 //  it's only a more readable way to specify indexes after all
 let index_arr = dims![x,x, y,y, z,z];
 assert_eq!(index_arr, [0,0, 1,1, 2,2usize]);
 # }
 ```

 This can be especially useful with the ```apply_dims``` method available to ```PointND```'s

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dims;
 # use point_nd::PointND;
 # fn apply_dims_example() -> Result<(), ()> {
 let p = PointND
     ::new([0,1,2,3])
     .apply_dims(&dims![y,w], |item| Ok( item * 2  ))?  // Multiplies items 1 and 3 by 2
     .apply_dims(&dims![x,z], |item| Ok( item + 10 ))?; // Adds 10 to items 0 and 2
 assert_eq!(p.into_arr(), [10, 2, 20, 6]);
 # Ok(())
 # }
 # }
 ```
 */
#[macro_export]
macro_rules! dims {

    ( $( $d:ident ), * ) => { [ $( dim!($d), )* ] };

}

/**
 Converts a range of identifiers and ```usize``` expressions to a range of ```usize``` values

 Using any identifiers apart from _x_, _y_, _z_ or _w_ will result in a compile time error

 It is recommended to use parentheses when calling this macro for clarity

 ### Possible Variations:

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dimr;
 # use point_nd::PointND;
 // PLEASE NOTE!
 //  x = 0usize
 //  y = 1
 //  z = 2
 //  w = 3

 // Range with identifiers
 assert_eq!(dimr!(x..z), 0..2usize);

 // RangeInclusive with identifiers
 assert_eq!(dimr!(y..=w), 1..=3usize);

 // RangeTo with identifiers
 assert_eq!(dimr!(..z), ..2);

 // RangeToInclusive with identifiers
 assert_eq!(dimr!(..=w), ..=3);

 // RangeFrom with identifier
 assert_eq!(dimr!(y..), 1..);

 // Range with identifier and expression
 assert_eq!(dimr!(x..10), 0..10usize);

 // RangeInclusive with identifier and expression
 assert_eq!(dimr!(x..=7), 0..=7usize);
 # }
 ```

 This is especially useful when taking slices of a ```PointND```

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dimr;
 # use point_nd::PointND;
 let p = PointND::new([0,1,2,3,4,5]);
 let slice = &p[dimr!(x..=z)];
 assert_eq!(slice, [0,1,2]);
 # }
 ```

 This macro is not just limited to ```PointND```'s though.

 Any collection that accepts a range of ```usize```'s for indexing is compatible with this macro

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dimr;
 let array = [0,1,2,3,4,5];
 let first_to_third  = &array[dimr!(x..w)];
 let fourth_to_sixth = &array[dimr!(w..=5)];

 # assert_eq!(*first_to_third,  [0, 1, 2]);
 # assert_eq!(*fourth_to_sixth, [3, 4, 5]);
 # }
 ```
 */
#[macro_export]
macro_rules! dimr {

    // Ident to Ident
    //  Range x..w
    ( $a:ident..$b:ident ) => { dim!($a)..dim!($b) };
    //  RangeInclusive y..=z
    ( $a:ident..=$b:ident ) => { dim!($a)..=dim!($b) };

    // Ident to Expr
    //  Range z...6
    ( $a:ident..$b:expr ) => { dim!($a)..$b };
    //  RangeInclusive w..=9
    ( $a:ident..=$b:expr ) => { dim!($a)..=$b };

    // Inf to Ident
    //  RangeTo ..w
    ( ..$a:ident ) => { ..dim!($a) };
    //  RangeToInclusive ..=z
    ( ..=$a:ident ) => { ..=dim!($a) };

    // Ident to Inf
    //  RangeFrom x..
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

            let p = PointND::<u8, 4>::from_slice(&arr);
            for (i, item) in p.iter().enumerate() {
                assert_eq!(arr[i], *item);
            }

            let mut p = PointND::<u8, 4>::from_slice(&arr);
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
            let p = PointND::<f64, 3>::from_slice(&arr);
            for i in 0..p.dims() {
                assert_eq!(arr[i], p[i]);
            }
        }
        #[test]
        #[should_panic]
        fn from_cannot_construct_with_zero_dimensions() {
            PointND::<i32, 0>::from_slice(&[]);
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

            let arr = [0,1,2,3];

            let p = PointND::<u8, 4>
                ::new(arr)
                .apply(|a| Ok( a * 2 ))
                .unwrap();

            assert_eq!(p.into_arr(), [0, 2, 4, 6]);
        }

        #[test]
        fn can_apply_dims() {

            let p = PointND::new([-2,-1,0,1,2])
                .apply_dims(&dims![x, w], |item| {
                    Ok( item - 10 )
                }).unwrap();
            assert_eq!(p.into_arr(), [-12,-1, 0, -9, 2]);
        }

        #[test]
        fn can_apply_vals() {

            let p = PointND::new([0,1,2])
                .apply_vals(
                    [Some(10), None, Some(20)],
                    |a, b| {
                        if let Some(i) = b {
                            Ok(a + i)
                        } else {
                            Ok( a )
                        }
                    }
                ).unwrap();
            assert_eq!(p.into_arr(), [10, 1, 22]);
        }

        #[test]
        fn can_apply_point() {

            let p1 = PointND::new([0, 1, 2, 3]);
            let p2 = PointND::new([0, -1, -2, -3]);
            let p3 = p1.apply_point(p2, |a, b| Ok( a - b )).unwrap();
            assert_eq!(p3.into_arr(), [0, 2, 4, 6]);
        }

    }

    #[cfg(test)]
    mod get {
        use super::*;

        #[test]
        fn can_get_slice_by_range_index() {
            let p = PointND::new([0,1,2,3,4]);
            let slice = &p[0..3];
            assert_eq!(slice, [0,1,2]);
        }

        #[test]
        #[should_panic]
        fn cannot_get_out_of_bounds_index() {
            let p = PointND::new([0,1,2]);
            let _x = p[p.dims() + 1];
        }


        #[test]
        fn getter_for_1d_points_work() {
            let arr = [0];
            let p = PointND::new(arr);
            assert_eq!(*p.x(), arr[0]);
        }

        #[test]
        fn getters_for_2d_points_work() {
            let arr = [0,1];
            let p = PointND::new(arr);

            assert_eq!(*p.x(), arr[0]);
            assert_eq!(*p.y(), arr[1]);
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
        fn getters_for_4d_points_work() {
            let arr = [0,1,2,3];
            let p = PointND::new(arr);

            assert_eq!(*p.x(), arr[0]);
            assert_eq!(*p.y(), arr[1]);
            assert_eq!(*p.z(), arr[2]);
            assert_eq!(*p.w(), arr[3]);
        }

    }

    #[cfg(test)]
    mod set {
        use super::*;

        #[test]
        fn can_set_value_by_index() {

            let mut p = PointND::new([0,1,2]);

            let new_val = 9999;
            p[1] = new_val;

            assert_eq!(p.into_arr(), [0, new_val, 2]);
        }

        #[test]
        fn setter_for_1d_points_work() {

            let old_vals = [0];
            let new_val = 4;
            let mut p = PointND::new(old_vals);

            p.set_x(new_val);
            assert_eq!(*p.x(), new_val);
        }

        #[test]
        fn setters_for_2d_points_work() {

            let old_vals = [0,1];
            let new_vals = [4,5];
            let mut p = PointND::new(old_vals);

            p.set_x(new_vals[0]);
            p.set_y(new_vals[1]);

            assert_eq!(*p.x(), new_vals[0]);
            assert_eq!(*p.y(), new_vals[1]);
        }

        #[test]
        fn setters_for_3d_points_work() {

            let old_vals = [0,1,2];
            let new_vals = [4,5,6];
            let mut p = PointND::new(old_vals);

            p.set_x(new_vals[0]);
            p.set_y(new_vals[1]);
            p.set_z(new_vals[2]);

            assert_eq!(*p.x(), new_vals[0]);
            assert_eq!(*p.y(), new_vals[1]);
            assert_eq!(*p.z(), new_vals[2]);
        }

        #[test]
        fn setters_for_4d_points_work() {

            let old_vals = [0,1,2,3];
            let new_vals = [4,5,6,7];
            let mut p = PointND::new(old_vals);

            p.set_x(new_vals[0]);
            p.set_y(new_vals[1]);
            p.set_z(new_vals[2]);
            p.set_w(new_vals[3]);

            assert_eq!(*p.x(), new_vals[0]);
            assert_eq!(*p.y(), new_vals[1]);
            assert_eq!(*p.z(), new_vals[2]);
            assert_eq!(*p.w(), new_vals[3]);
        }

    }

    #[cfg(test)]
    mod shift {
        use super::*;

        #[test]
        fn can_shift_1d_points() {
            let mut p = PointND::new([0.1]);
            p.shift_x(1.23);

            assert_eq!(p.into_arr(), [1.33]);
        }

        #[test]
        fn can_shift_2d_points() {
            let mut p = PointND::new([12, 345]);
            p.shift_x(-22);
            p.shift_y(-345);

            assert_eq!(p.into_arr(), [-10, 0]);
        }

        #[test]
        fn can_shift_3d_points() {
            let mut p = PointND::new([42.4, 2.85, 75.01]);
            p.shift_x(40.6);
            p.shift_y(-2.85);
            p.shift_z(24.99);

            assert_eq!(p.into_arr(), [83.0, 0.0, 100.0]);
        }

        #[test]
        fn can_shift_4d_points() {
            let mut p = PointND::new([0,1,2,3]);
            p.shift_x(10);
            p.shift_y(-2);
            p.shift_z(5);
            p.shift_w(0);

            assert_eq!(p.into_arr(), [10, -1, 7, 3]);
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
    mod macros {
        use super::*;

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
