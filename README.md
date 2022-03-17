
# PointND

A simple and flexible data structure for modelling points of **any** dimensions on an axis

This crate uses constant generics, it is recommended for use with a rust version **>= 1.51**

## Basic Usage

### Constructing a Point

This is really just a wrapper around an array with convenience methods for accessing values if it's dimensions are within ```1..=4```

```rust
// Creating a 2D point from a given array
let arr: [i32; 2] = [0,1];
let p: PointND<_, 2> = PointND::new(arr);

// Creating a 3D point from values of a given slice
let vec: Vec<i32> = vec![0, 1, 2];
let p: PointND<_, 3> = PointND::from(&vec);

// Creating a 4D point with all values set to 5
let p: PointND<i32, 4> = PointND::fill(5);

// The second generic arg is a usize constant and for the ::fill()
//  and ::from() functions, specifying it is sometimes necessary
// If you don't like writing PointND twice for type annotation,
//  use FQS (fully qualified syntax) instead:
let p = PointND::<_, 4>::fill(5);

// Trying to create a PointND with zero dimensions using the above will panic at runtime
//  ERROR: Can't create a point with zero dimensions
//  let p: PointND<_, 0> = PointND::new([]);
```

## Getting and Setting Values

If the dimensions of the point are within ```1..=4```, it is recommended to use the convenience getters and setters

```rust
let arr = [0,1];
let p: PointND<_, 2> = PointND::new(arr);

// As the point has 2 dimensions, we can access
//  it's values with the x() and y() methods
let x: &i32 = p.x();
assert_eq!(*x, arr[0]);
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

These are the only methods available for ```PointND```'s with dimensions **not** within ```1..=4```

Their use can be made easier using the dimension macros: ```dim```, ```dims``` and ```dimr``` (see the documentation for more info)

```rust
// A 5D point, cannot use any of the convenience methods
let arr = [-2, -1, 0, 1, 2];
let mut p = PointND::new(arr);

// ERROR: Not implemented for PointND<i32, 5>
// let x = p.x()
// ...
// let w = p.w();

// Instead use these deref array methods:
//  Safely getting
let x: Option<&i32> = p.get(0);
assert_eq!(*x.unwrap(), arr[0]);

// Indexing
let y: i32 = p[1];
assert_eq!(y, arr[1]);

// Slicing
let z_to_last: &[i32] = &p[2..];
assert_eq!(z_to_last, [0,1,2]);

// Setting via Indexing
p[1] = 345;
assert_eq!(p[1], 345);
```

## Querying Size

The number of dimensions can be retrieved using the ```dims()``` method (short for _dimensions_)

```rust
let p: PointND<i32, 2> = PointND::new([0,1]);
assert_eq!(p.dims(), 2);
// Alternatively, as PointND implements Deref, we can use len().
// It's name isn't as descriptive however
assert_eq!(p.len(), 2);
```

## Iterating

Iterating over a ```PointND``` is as easy as:

```rust
let mut p = PointND::new([0,1]);

for _ in p.iter()      { /* Do stuff */ }
for _ in p.iter_mut()  { /* Change stuff */ }
for _ in p.into_iter() { /* Do more stuff */ }
```

It must be noted that due to the ```Copy``` trait bounds of the items contained by a ```PointND```,
using ```into_iter()``` will not actually move the point as we are actually iterating over the contained
array via the ```Deref``` trait.

```rust
// The point 'p' is still usable after the call to into_iter()
assert_eq!(p.dims(), 2);
```

If destroying innocent points is your thing however, using ```into_arr()``` or ```into_vec()``` to
consume the point before iterating will move it out of scope

```rust
for _ in p.into_vec().into_iter() { /* Take stuff */ }

// ERROR: Can't access moved value
// assert_eq!(p.dims(), 2);
```

## Transforming Points

### Appliers

The ```apply```, ```apply_vals```, ```apply_dims``` and ```apply_point``` (henceforth referred to as _appliers_)
methods all consume self and return a new point after applying a function to all contained values

Multiple appliers can be chained together to make complex transformations to a ```PointND```

This is probably best explained with an example:

```rust
// A trivial transformation more easily done via other methods...
//  but it gets the point across
let p = PointND
    ::new([0,1,2])                      // Creates a new PointND
    .apply(|item| Ok( item + 2 ))?      // Adds 2 to each item
    .apply(|item| Ok( item * 3 ))?;     // Multiplies each item by 3
assert_eq!(p.into_arr(), [6, 9, 12]);
```

### Creating a Function to Pass to Appliers

The function or closure passed to the applier methods (henceforth referred to as _modifiers_)
accept either one or two args of type ```T``` (where ```T``` is the type of the items contained
by the point) depending on whether one or two sets of values are being modified.

Modifiers must all return a ```Result<T, ()>``` to allow graceful error handling by the applier instead of just panicking.

If an ```Err``` is returned by the modifier when called on any item, the applier returns an ```Err(())```

If all goes well, the applier returns an ```Ok``` with the new ```PointND``` as it's value

Hopefully the above wasn't confusing, but here's an example just in case:

```rust
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
```

## Contributing

Any suggestions for the codebase, documentation, README (or anything) are more than welcome! 

If you have any problems with the API

## License

This crate uses the MIT license.
