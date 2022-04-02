
use core::ops::{Deref, DerefMut, AddAssign};
use core::convert::TryFrom;
use core::array::TryFromSliceError;
use arrayvec::ArrayVec;
use crate::utils::*;

// For use within methods that make use of ArrayVec
// Checks if the dimensions of a point are greater than the max capacity of ArrayVec's
macro_rules! check_transform_cap {
    ( $dims:expr, $method:expr ) => {
        if $dims > MAX_POINT_DIMS {
            panic!(
                "Attempted to call {}() on PointND with more than u32::MAX dimensions. \
                Try reducing the dimensions of the point via the contract_by() or \
                contract_to() before transforming",
                $method
            );
        }
    };
}

// For use ONLY within the apply, extend and contract methods as their constant
//  generics ensure that the ArrayVec is always filled with initialised values
// Converts ArrayVec<T,N> into [T;N] unsafely
macro_rules! arrvec_into_inner {
    ($arrvec:expr, $method:expr) => {
        match $arrvec.into_inner() {
            Ok(arr) => PointND::from(arr),
            _ => panic!("Couldn't convert ArrayVec into array in {}() method. \
                         This operation should never have panicked. Please contact \
                         the maintainers of PointND if troubles persist", $method)
        }
    };
}

/**

The whole _point_ of the crate.

The ```PointND``` struct is really just a smart pointer to an array of type ```[T; N]```
with convenience methods for accessing, setting and transforming values.

As the struct dereferences to a slice, all methods implemented for slices are available with this.

# Making a Point

There are three ```PointND``` constructors, ```from()```, ```from_slice()``` and ```fill()```.

The ```from_slice()``` and ```fill()``` functions can only be used
if creating a point where the items implement ```Copy```

```
# use point_nd::PointND;
// Creating a 2D point from a given array
let arr = [0, 1];
let p: PointND<i32, 2> = PointND::from(arr);

// Creating a 3D point from values of a given slice
let vec: Vec<i32> = vec![0, 1, 2];
let p: PointND<_, 3> = PointND::from_slice(&vec);

// Creating a 4D point with all values set to 5
let p: PointND<i32, 4> = PointND::fill(5);

// You can even construct a PointND with zero dimensions
let p: PointND<i32, 0> = PointND::from([]);
```

The second generic is a ```usize``` constant generic and for the ```fill()```
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
let p = PointND::from([0,1]);

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

```
# use point_nd::PointND;
# use point_nd::{dim, dimr};
let p = PointND::from([0,1,2,3,4,5]);

// ERROR: Not implemented for PointND of 6 dimensions
//  let x = p.x();

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
let z = p[dim!(z)];
let the_rest = &p[dimr!(w..)];
```

To get **all** the values contained by a point, use the ```into_arr()``` method

```
# use point_nd::PointND;
let p = PointND::from([-10, -2, 0, 2, 10]);
assert_eq!(p.into_arr(), [-10, -2, 0, 2, 10])
```

# Querying Size

The number of dimensions can be retrieved using
the ```dims()``` method (short for _dimensions_)

```
# use point_nd::PointND;
let p: PointND<i32, 2> = PointND::from([0,1]);
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
let mut p = PointND::from([0, 1]);

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
let mut p = PointND::from([0, 1]);

// Sets x via indexing
p[0] = -100;
assert_eq!(*p.x(), -100);

// Sets y via indexing and dimension macros
p[dim!(y)] = -100;
assert_eq!(*p.x(), *p.y());
```

### Appliers

The ```apply()```, ```apply_vals()```, ```apply_dims()``` and ```apply_point()``` methods all
consume self and return a new point after calling a function or closure on all contained values

Multiple applier methods can be chained together to make complex transformations to a ```PointND```

This is probably best explained with an example:

```
# use point_nd::PointND;
// A trivial transformation more easily done other ways
//  ...but it gets the point across
let p = PointND
    ::from([0,1,2])            // Creates a new PointND
    .apply(|item| item + 2)    // Adds 2 to each item
    .apply(|item| item * 3);   // Multiplies each item by 3
assert_eq!(p.into_arr(), [6, 9, 12]);
```

Each applier has it's own subtle differences, it is recommended to read the documentation for each of them

# Iterating

Iterating over a ```PointND``` is as easy as:

```
# use point_nd::PointND;
let mut p = PointND::from([0,1]);

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
# let mut p = PointND::from([0,1]);
for _ in p.into_arr().into_iter() { /* Move stuff */ }

// ERROR: Can't access moved value
// assert_eq!(p.dims(), 2);
```

# Things (not strictly necessary) to Note

### Convenience Methods

As stated earlier, certain methods for accessing and setting the values contained by a ```PointND```
are only implemented for points within **1..=4** dimensions.

This was done to ensure that the compiler could check if out of index values were being accessed.
If it these methods were implemented for ```PointND```'s of any dimension, these errors would have
to be caught at runtime.

This was done to mirror the behaviour of arrays at compile time as closely as possible.

### Math Operations

Unlike structures in other crates, ```PointND```'s (as of ```v0.5.0```) do not implement mutating
and consuming math operations like ```Neg```, ```Add```, ```SubAssign```, _etc_.

It was decided that these functionalities and others could provided by independent crates via
functions which could be imported and passed to the apply, extend and contract methods.

```Eq``` and ```PartialEq``` are implemented though.

### Dimensional Capacity

This crate relies heavily on the ```arrayvec``` crate when applying transformations to points. Due
to the fact that ```arrayvec::ArrayVec```'s lengths are capped at ```u32::MAX```, the apply, extend
and contract methods cannot be used on ```PointND```'s with dimensions exceeding that limit.

This usually should not be a problem though (Who wants a ```u32::MAX + 1``` dimensional point anyway?
You'd need a ```16GB``` stack to hold one point containing ```i32```'s), but it is probably worth mentioning.

 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointND<T, const N: usize>([T; N]);

impl<T, const N: usize> PointND<T, N> {

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


    /**
     Consumes ```self``` and calls the ```modifier``` on each item contained
     by ```self``` to create a new ```PointND``` of the same length.

     ```
     # use point_nd::PointND;
     let p = PointND
         ::from([0,1,2])             // Creates a new PointND
         .apply(|item| item + 2)     // Adds 2 to each item
         .apply(|item| item * 3);    // Multiplies each item by 3
     assert_eq!(p.into_arr(), [6, 9, 12]);
     ```

     The return type of the ```modifier``` does not necessarily have to be
     the same as the type of the items passed to it. This means that ```apply```
     can create a new point with items of a different type, but the same length.

     ```
     # use point_nd::PointND;
     let p = PointND
         ::from([0,1,2])                // Creates a new PointND
         .apply(|item| item as f32);    // Converts items to float
     assert_eq!(p.into_arr(), [0.0, 1.0, 2.0]);
     ```

     # Panics

     - If the dimensions of ```self``` are greater than ```u32::MAX```.
     */
    pub fn apply<U>(self, modifier: ApplyFn<T, U>) -> PointND<U, N> {

        check_transform_cap!(N, "apply");

        let mut arr_v = ArrayVec::<U, N>::new();
        let mut self_ = ArrayVec::from(self.into_arr());
        // Need to reverse as we'll be popping from the back of the array
        self_.reverse();

        for _ in 0..N {
            // Items CANNOT be iterated, only popped
            let item = self_.pop().unwrap();
            let item = modifier(item);
            arr_v.push(item);
        }

        arrvec_into_inner!(arr_v, "apply")
    }

    /**
     Consumes ```self``` and calls the ```modifier``` on the items at the
     specified ```dims``` to create a new ```PointND``` of the same length.

     Any items at dimensions not specified will be passed to the new point without change

     ```
     # use point_nd::PointND;
     let p = PointND
         ::from([0,1,2,3,4])                       // Creates a PointND
         .apply_dims(&[1,3], |item| item * 2)      // Multiplies items 1 and 3 by 2
         .apply_dims(&[0,2], |item| item + 10);    // Adds 10 to items 0 and 2
     assert_eq!(p.into_arr(), [10, 2, 12, 6, 4]);
     ```

     Unlike some other apply methods, this ```apply_dims``` cannot return
     a ```PointND``` with items of a different type from the original.

     # Panics

     - If the dimensions of ```self``` are greater than ```u32::MAX```.
     */
    pub fn apply_dims(self, dims: &[usize], modifier: ApplyDimsFn<T>) -> Self {

        check_transform_cap!(N, "apply_dims");

        let mut arr_v = ArrayVec::<T, N>::new();
        let mut self_ = ArrayVec::from(self.into_arr());
        // Need to reverse as we'll be popping from the back of the array
        self_.reverse();

        for i in 0..N {
            let item = self_.pop().unwrap();
            if dims.contains(&i) {
                arr_v.push(modifier(item));
            } else {
                arr_v.push(item);
            }
        }

        arrvec_into_inner!(arr_v, "apply_dims")
    }

    /**
     Consumes ```self``` and calls the ```modifier``` on each item contained by
    ```self``` and ```values``` to create a new ```PointND``` of the same length.

     As this method may modify every value in the original point,
     the ```values``` array must be the same length as the point.

     When creating a modifier function to be used by this method, keep
     in mind that the items in ```self``` are passed to it through the
     **first arg**, and the items in ```values``` through the **second**.

     ```
     # use point_nd::PointND;
     let p = PointND
         ::from([0,1,2])                      // Creates a new PointND
         .apply_vals([1,3,5], |a, b| a + b)   // Adds items in point to items in array
         .apply_vals([2,4,6], |a, b| a * b);  // Multiplies items in point to items in array
     assert_eq!(p.into_arr(), [2, 16, 42]);
     ```

     Neither the return type of the ```modifier``` nor the type of the items contained
     by the ```values``` array necessarily have to be the same as the item type of the
     original point. This means that ```apply_vals``` can create a new point with items
     of a different type, but the same length.

     ```
     # use point_nd::PointND;
     enum Op {
        Add,
        Sub,
     }

    // Adds or subtracts 10 from 'a' depending on the
    //  operation specified in 'b', then converts to float
    let add_or_sub = |a, b| {
        match b {
            Op::Add => (a + 10) as f32,
            Op::Sub => (a - 10) as f32
        }
    };

     let p = PointND
         ::from([0,1,2])
         .apply_vals(
             [Op::Add, Op::Sub, Op::Add],
             add_or_sub
         );
     assert_eq!(p.into_arr(), [10.0, -9.0, 12.0]);
     ```

     # Panics

     - If the dimensions of ```self``` or ```values``` are greater than ```u32::MAX```.
     */
    pub fn apply_vals<U, V>(self, values: [V; N], modifier: ApplyValsFn<T, U, V>) -> PointND<U, N> {

        check_transform_cap!(N, "apply_vals");

        let mut arr_v = ArrayVec::<U, N>::new();
        let mut vals = ArrayVec::from(values);
        let mut self_ = ArrayVec::from(self.into_arr());

        // Need to reverse as we'll be popping from the back of the arrays
        vals.reverse();
        self_.reverse();

        for _ in 0..N {
            let a = self_.pop().unwrap();
            let b = vals.pop().unwrap();
            arr_v.push(modifier(a, b));
        }

        arrvec_into_inner!(arr_v, "apply_vals")
    }

    /**
     Consumes ```self``` and calls the ```modifier``` on each item contained by
     ```self``` and another ```PointND``` to create a new point of the same length.

     When creating a modifier function to be used by this method, keep
     in mind that the items in ```self``` are passed to it through the
     **first arg**, and the items in ```other``` through the **second**.

     ```
     # use point_nd::PointND;
     let p1 = PointND::from([0,9,3,1]);
     let p2 = PointND::fill(10);
     let p3 = PointND
         ::from([1,2,3,4])                // Creates a new PointND
         .apply_point(p1, |a, b| a - b)   // Subtracts items in p3 with those in p1
         .apply_point(p2, |a, b| a * b);  // Multiplies items in p3 with those in p2
     assert_eq!(p3.into_arr(), [10, -70, 0, 30]);
     ```

     Neither the return type of the ```modifier``` nor the type of the items
     contained by the ```other``` point necessarily have to be  the same as
     the type of the items in the original point. This means that ```apply_point```
     can create a new point with items of a different type, but the same length.

     # Panics

     - If the dimensions of ```self``` or ```other``` are greater than ```u32::MAX```.
     */
    pub fn apply_point<U, V>(self, other: PointND<V, N>, modifier: ApplyPointFn<T, U, V>) -> PointND<U, N> {

        check_transform_cap!(N, "apply_point");

        self.apply_vals(other.into_arr(), modifier)
    }


    /**
     Consumes ```self``` and returns a new ```PointND``` with
     items from ```values``` appended to items from the original.

     ```
     # use point_nd::PointND;
     let p = PointND
         ::from([0,1])
         .extend([2,3]);
      assert_eq!(p.into_arr(), [0,1,2,3]);
     ```

     # Panics

     - If the combined length of ```self``` and ```values``` are greater than ```u32::MAX```.

     ```should_panic
     # use point_nd::PointND;
     const N: usize = u32::MAX as usize;
     const L: usize = 1;
     const M: usize = N + L;

     let p: PointND<_, M> = PointND
         ::from([0; N])
         .extend([1; L]);
     ```
     */
    pub fn extend<const L: usize, const M: usize>(self, values: [T; L]) -> PointND<T, M> {

        check_transform_cap!(L + N, "extend");

        let mut arr_v = ArrayVec::<T, M>::new();
        let mut self_ = ArrayVec::from(self.into_arr());
        let mut vals = ArrayVec::from(values);

        // Need to reverse as we'll be popping from the back of the arrays
        self_.reverse();
        vals.reverse();

        for _ in 0..N { arr_v.push(self_.pop().unwrap()); }
        for _ in 0..L { arr_v.push(vals.pop().unwrap()); }

        arrvec_into_inner!(arr_v, "extend")
    }


    /**
     Consumes ```self``` and returns a new ```PointND``` which
     retains only the first ```dims``` items of the original.

     This method always removes the rearmost items first.

     ```
     # use point_nd::PointND;
     let p = PointND
        ::from([0,1,2,3])
        .contract(2);
     assert_eq!(p.dims(), 2);
     assert_eq!(p.into_arr(), [0,1]);
     ```

     # Panics

     - If ```dims``` is greater than the original dimensions of the point (_a.k.a_ -
       you cannot shorten the dimensions of a point to more than it had originally).

     ```should_panic
     # use point_nd::PointND;
     let p = PointND
        ::from([0,1,2])
        .contract(1_000_000);
     # // Just to silence the type error
     # let _p2 = PointND::from([0,1,2]).apply_point(p, |a, b| a + b);
     ```

     - If the dimensions of ```self``` are greater than ```u32::MAX```.
     */
    pub fn contract<const M: usize>(self, dims: usize) -> PointND<T, M> {

        if dims > N || M > N {
            panic!("Attempted to contract PointND to more dimensions than it had originally. Try \
                    passing a usize value that is less than the dimensions of the original point");
        }

        let mut arr_v = ArrayVec::<T, M>::new();
        let mut self_ = ArrayVec::from(self.into_arr());

        // Have to reverse as we'll be popping from the back
        self_.reverse();

        for _ in 0..M {
            let item = self_.pop().unwrap();
            arr_v.push(item);
        }

        arrvec_into_inner!(arr_v, "contract")
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
     let p1 = PointND::from([0,1]);       // Compiler knows this has 2 dimensions
     let p2 = PointND::from_slice(&vec![2,3]);

     // Later, p2 is applied to p1. The compiler is able to infer its dimensions
     let p3 = p1.apply_point(p2, |a, b| a + b);
     ```

     # Panics

     - If the slice passed cannot be converted into an array

    ```should_panic
    # use point_nd::PointND;
    let arr = [0,1,2];
    // ERROR: Cannot convert slice of [i32; 3] to [i32; 100]
    let p = PointND::<_, 100>::from_slice(&arr[..]);
    ```
     */
    pub fn from_slice(slice: &[T]) -> Self {
        let arr: [T; N] = slice.try_into().unwrap();
        PointND::from(arr)
    }

    /**
     Returns a new ```PointND``` with all values set as specified

     If the compiler is not able to infer the dimensions (a.k.a - length)
     of the point, it needs to be explicitly specified

     See the ```from_slice()``` function for cases when generics don't need to be explicitly specified

     ```
     # use point_nd::PointND;
     // A point with 10 dimensions with all values set to 2
     let p = PointND::<_, 10>::fill(2);

     assert_eq!(p.dims(), 10);
     assert_eq!(p.into_arr(), [2; 10]);
     ```
     */
    pub fn fill(value: T) -> Self {
        PointND::from([value; N])
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


// Convenience Getters and Setters
/// Functions for safely getting and setting the value contained by a 1D ```PointND```
impl<T> PointND<T, 1> {

    pub fn x(&self) -> &T { &self[0] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }

}
/// Functions for safely getting and setting the values contained by a 2D ```PointND```
impl<T> PointND<T, 2> {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }

}
/// Functions for safely getting and setting the values contained by a 3D ```PointND```
impl<T> PointND<T, 3>  {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }
    pub fn z(&self) -> &T { &self[2] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }
    pub fn set_z(&mut self, new_value: T) { self[2] = new_value; }

}
/// Functions for safely getting and setting the values contained by a 4D ```PointND```
impl<T> PointND<T, 4>  {

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
/// Function for safely transforming the value contained by a 1D ```PointND```
impl<T> PointND<T, 1>
    where T: AddAssign {

    pub fn shift_x(&mut self, delta: T) { self[0] += delta; }

}
/// Functions for safely transforming the values contained by a 2D ```PointND```
impl<T> PointND<T, 2>
    where T: AddAssign {

    pub fn shift_x(&mut self, delta: T) { self[0] += delta; }
    pub fn shift_y(&mut self, delta: T) { self[1] += delta; }

}
/// Functions for safely transforming the values contained by a 3D ```PointND```
impl<T> PointND<T, 3>
    where T: AddAssign {

    pub fn shift_x(&mut self, delta: T) { self[0] += delta; }
    pub fn shift_y(&mut self, delta: T) { self[1] += delta; }
    pub fn shift_z(&mut self, delta: T) { self[2] += delta; }

}
/// Functions for safely transforming the values contained by a 4D ```PointND```
impl<T> PointND<T, 4>
    where T: AddAssign {

    pub fn shift_x(&mut self, delta: T) { self[0] += delta; }
    pub fn shift_y(&mut self, delta: T) { self[1] += delta; }
    pub fn shift_z(&mut self, delta: T) { self[2] += delta; }
    pub fn shift_w(&mut self, delta: T) { self[3] += delta; }

}


impl<T, const N: usize> From<[T; N]> for PointND<T, N> {

    fn from(array: [T; N]) -> Self {
        PointND(array)
    }

}

impl<T, const N: usize> Into<[T; N]> for PointND<T, N>  {

    fn into(self) -> [T; N] {
        self.into_arr()
    }

}

impl<T, const N: usize> TryFrom<&[T]> for PointND<T, N>
    where T: Copy {

    type Error = TryFromSliceError;
    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {

        let res: Result<[T; N], _> = slice.clone().try_into();
        match res {
            Ok(arr) => Ok( PointND(arr) ),
            Err(err) => Err( err )
        }
    }

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

        // The from() constructor is under tests::from

        #[test]
        fn from_slice_works() {
            let arr = [0.0, 0.1, 0.2];
            let p = PointND::<f64, 3>::from_slice(&arr);
            for i in 0..p.dims() {
                assert_eq!(arr[i], p[i]);
            }
        }

        #[test]
        fn fill_works() {
            let fill_val = 21u8;
            let p = PointND::<u8, 5>::fill(fill_val);
            for i in p.into_iter() {
                assert_eq!(i, fill_val);
            }
        }

    }

    #[cfg(test)]
    mod appliers {
        use super::*;

        #[test]
        fn can_apply() {

            let arr = [0,1,2,3];

            let p = PointND::<u8, 4>
                ::from(arr)
                .apply(|a| a * 2);

            assert_eq!(p.into_arr(), [0, 2, 4, 6]);
        }

        #[test]
        fn can_apply_dims() {

            let p = PointND::from([-2,-1,0,1,2])
                .apply_dims(&[0, 3], |item| item - 10);
            assert_eq!(p.into_arr(), [-12,-1, 0, -9, 2]);
        }

        #[test]
        fn can_apply_vals() {

            let p = PointND::from([0,1,2])
                .apply_vals([Some(10), None, Some(20)],
                            |a, b| {
                        if let Some(i) = b {
                            a + i
                        } else {
                            a
                        }
                    });
            assert_eq!(p.into_arr(), [10, 1, 22]);
        }

        #[test]
        fn can_apply_point() {

            let p1 = PointND::from([0, 1, 2, 3]);
            let p2 = PointND::from([0, -1, -2, -3]);
            let p3 = p1.apply_point(p2, |a, b| a - b );
            assert_eq!(p3.into_arr(), [0, 2, 4, 6]);
        }

        #[test]
        fn can_apply_noclone_items() {

            #[derive(Debug, Eq, PartialEq)]
            enum X { A, B, C }

            let p = PointND
                ::from([X::A, X::B, X::C])
                .apply(|x| {
                    match x {
                        X::A => X::B,
                        X::B => X::C,
                        X::C => X::A,
                    }
                });

            assert_eq!(p.into_arr(), [X::B, X::C, X::A]);
        }

    }

    #[cfg(test)]
    mod extenders {
        use super::*;

        #[test]
        fn can_extend() {

            let zero = PointND::<i32, 0>::from([]);
            assert_eq!(zero.dims(), 0);

            let one = zero.clone().extend([0]);
            assert_eq!(one.dims(), 1);
            assert_eq!(one.into_arr(), [0]);

            let two = zero.clone().extend([0,1]);
            assert_eq!(two.dims(), 2);
            assert_eq!(two.into_arr(), [0, 1]);

            let five = PointND
                ::from([0,1,2])
                .extend([3,4]);
            assert_eq!(five.dims(), 5);
            assert_eq!(five.clone().into_arr(), [0,1,2,3,4]);

            let sum = five.apply_point(PointND::from([0,1,2,3,4]), |a, b| a + b);
            assert_eq!(sum.into_arr(), [0,2,4,6,8]);

        }

        #[test]
        fn can_extend_nothing() {
            let arr: [i32; 0] = [];
            let zero = PointND
                ::from(arr)
                .extend::<0, 0>(arr);
            assert_eq!(zero.dims(), 0);
        }

    }

    #[cfg(test)]
    mod contractors {
        use super::*;

        #[test]
        fn can_contract_to() {
            let p = PointND
                ::from([0,1,2,3])
                .contract(3);

            assert_eq!(p.dims(), 3);
            assert_eq!(p.into_arr(), [0,1,2]);
        }

        #[test]
        fn can_contract_to_zero() {
            let p = PointND
                ::from([0,1,2,3])
                .contract(0);

            assert_eq!(p.dims(), 0);
            assert_eq!(p.into_arr(), []);
        }

        #[test]
        fn can_contract_to_same() {
            let p = PointND
                ::from([0,1,2,3])
                .contract(4);

            assert_eq!(p.dims(), 4);
            assert_eq!(p.into_arr(), [0,1,2,3]);
        }

        #[test]
        #[should_panic]
        #[allow(unused_variables)]
        fn cannot_contract_to_more_dimensions() {
            let p = PointND
                ::from([0,1,2,3])
                .contract::<1000>(1000);
        }

    }

    #[cfg(test)]
    mod get {
        use super::*;

        #[test]
        fn can_get_slice_by_range_index() {
            let p = PointND::from([0,1,2,3,4]);
            let slice = &p[0..3];
            assert_eq!(slice, [0,1,2]);
        }

        #[test]
        #[should_panic]
        fn cannot_get_out_of_bounds_index() {
            let p = PointND::from([0,1,2]);
            let _x = p[p.dims() + 1];
        }


        #[test]
        fn getter_for_1d_points_work() {
            let arr = [0];
            let p = PointND::from(arr);
            assert_eq!(*p.x(), arr[0]);
        }

        #[test]
        fn getters_for_2d_points_work() {
            let arr = [0,1];
            let p = PointND::from(arr);

            assert_eq!(*p.x(), arr[0]);
            assert_eq!(*p.y(), arr[1]);
        }

        #[test]
        fn getters_for_3d_points_work() {
            let arr = [0,1,2];
            let p = PointND::from(arr);

            assert_eq!(*p.x(), arr[0]);
            assert_eq!(*p.y(), arr[1]);
            assert_eq!(*p.z(), arr[2]);
        }

        #[test]
        fn getters_for_4d_points_work() {
            let arr = [0,1,2,3];
            let p = PointND::from(arr);

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

            let mut p = PointND::from([0,1,2]);

            let new_val = 9999;
            p[1] = new_val;

            assert_eq!(p.into_arr(), [0, new_val, 2]);
        }

        #[test]
        fn setter_for_1d_points_work() {

            let old_vals = [0];
            let new_val = 4;
            let mut p = PointND::from(old_vals);

            p.set_x(new_val);
            assert_eq!(*p.x(), new_val);
        }

        #[test]
        fn setters_for_2d_points_work() {

            let old_vals = [0,1];
            let new_vals = [4,5];
            let mut p = PointND::from(old_vals);

            p.set_x(new_vals[0]);
            p.set_y(new_vals[1]);

            assert_eq!(*p.x(), new_vals[0]);
            assert_eq!(*p.y(), new_vals[1]);
        }

        #[test]
        fn setters_for_3d_points_work() {

            let old_vals = [0,1,2];
            let new_vals = [4,5,6];
            let mut p = PointND::from(old_vals);

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
            let mut p = PointND::from(old_vals);

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
            let mut p = PointND::from([0.1]);
            p.shift_x(1.23);

            assert_eq!(p.into_arr(), [1.33]);
        }

        #[test]
        fn can_shift_2d_points() {
            let mut p = PointND::from([12, 345]);
            p.shift_x(-22);
            p.shift_y(-345);

            assert_eq!(p.into_arr(), [-10, 0]);
        }

        #[test]
        fn can_shift_3d_points() {
            let mut p = PointND::from([42.4, 2.85, 75.01]);
            p.shift_x(40.6);
            p.shift_y(-2.85);
            p.shift_z(24.99);

            assert_eq!(p.into_arr(), [83.0, 0.0, 100.0]);
        }

        #[test]
        fn can_shift_4d_points() {
            let mut p = PointND::from([0,1,2,3]);
            p.shift_x(10);
            p.shift_y(-2);
            p.shift_z(5);
            p.shift_w(0);

            assert_eq!(p.into_arr(), [10, -1, 7, 3]);
        }

    }

    #[cfg(test)]
    mod from_and_into {
        use super::*;

        #[test]
        fn from_array_works() {
            let p = PointND::from([0,1,2]);
            assert_eq!(p.dims(), 3);

            let p: PointND<i32, 4> = [22; 4].into();
            let p = p.apply(|i| i / 2);
            assert_eq!(p.into_arr(), [11; 4]);
        }

        #[test]
        fn into_array_works() {
            let arr: [i32; 3] = PointND::fill(10).into();
            assert_eq!(arr, [10, 10, 10]);
        }

    }

    #[cfg(test)]
    mod try_from_and_try_into {
        use super::*;

        #[test]
        fn can_try_from_array() {
            let arr = [0,1,2,3,4,5];
            let p: Result<PointND<_, 6>, _> = arr.try_into();
            assert!(p.is_ok());
        }

        #[test]
        #[allow(unused_variables)]
        fn can_try_from_slice_of_same_len() {
            let slice = &[0,1,2,3,4][..];
            let p: Result<PointND<_, 5>, _> = slice.try_into();
            assert!(p.is_ok());
        }

        #[test]
        fn cannot_try_from_slice_of_different_length() {
            let slice = &[0,1,2,3,4][..];
            let p: Result<PointND<_, 10921>, _> = slice.try_into();
            assert!(p.is_err());
        }

    }

    #[cfg(test)]
    mod operators {
        use super::*;

        #[test]
        fn can_eq() {
            let arr = [0, -1, 2, -3];
            let p1 = PointND::from(arr);
            let p2 = PointND::from(arr);

            assert_eq!(p1, p2);
        }

        #[test]
        fn can_ne() {
            let p1 = PointND::from([0, -1, 2, -3]);
            let p2 = PointND::from([0, 1, 2, 3]);

            assert_ne!(p1, p2);
        }

    }

}