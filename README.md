
# PointND

A simple and flexible data structure for modelling points of 
**any** dimensions on an axis **without** the standard library.

This crate uses constant generics, it is recommended 
for use with a Rust version **>= 1.51**.

## Basic Usage

### Making a Point

```rust
// Creating a 2D point from a given array
let arr = [0,1];
let p: PointND<i32, 2> = PointND::new(arr);

// Creating a 3D point from values of a given slice
let vec = vec![0, 1, 2];
let p: PointND<i32, 3> = PointND::from_slice(&vec);

// Creating a 4D point with all values set to 5
let p: PointND<i32, 4> = PointND::fill(5);
```

### Querying Values and Properties 

If the dimensions of the point are within ```1..=4```, it is 
recommended to use the convenience getters for accessing values.

```rust
let p = PointND::new([0, 1]);

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
```

The above methods are not implemented for PointND's with more than 4 dimensions. 

We must use indexing instead.

```rust
let p = PointND::new([0,1,2,3,4,5]);

// ERROR: Not implemented for PointND of 6 dimensions
// let x = p.x();

let x: i32 = p[0];
let y = p[1];

// The dimension macros provided by this crate can make
//  direct indexing easier and more readable
// See the documentation for more info
let z = p[dim!(z)];
let the_rest = p[dimr!(w..)];
```

To get the dimensions of a point, use the ```dims()``` method.

```rust
let p = PointND::new([0, 1, 2, 3]);

let dims: usize = p.dims();
assert_eq!(dims, 4);
```

### Transforming Values

If the dimensions of the point are within ```1..=4```, it is 
recommended to use the convenience setters for setting values.

```rust
let mut p = PointND::new([0, 1]);

// As the point has 2 dimensions, we can set
//  it's values with the set_x() and set_y() methods
// There are set_z() and set_w() methods available for
//  points with 3 and 4 dimensions respectively
p.set_x(-10);
p.set_y(-20);
```

The above methods are not implemented for PointND's with more than 4 dimensions. 

We must use indexing instead.

```rust
let p = PointND::new([0,1,2,3,4]);

// ERROR: Not implemented for PointND of 5 dimensions
// p.set_x(1200);

p[0] = 1200;
// Or...
p[dim!(x)] = 1200;
```

Complex transformations can be made via functions passed to the ```apply```, 
```apply_vals```, ```apply_dims``` and ```apply_point``` methods. 

See the documentation for more info.

```rust
let add_ten = |item| -> Result<i32, ()> {
    Ok( item + 10 )
}
let sum_nums = |a, b| -> Result<i32, ()> {
    Ok( a + b )
}
let double = |item| -> Result<i32, ()> {
    Ok( item * 2 )
}

let p1 = PointND::new([0,1,2,3,4,5]);
let p2 = PointND::new([0,1,2,3,4,5])
         // Adds ten to each item
         .apply(add_ten)?
         // Doubles items at indexes 0, 1 and 2
         .apply_dims(&[0,1,2], double)?
         // Does the same thing, just more readable
         .apply_dims(&dims![x,y,z], double)?
         // Adds items in p2 to respective items in p1
         .apply_point(p1, sum_nums)?;
```

### Iterating

Iterating over a ```PointND``` is as easy as:

```rust
let mut p = PointND::new([0,1]);

for _ in p.iter()      { /* Do stuff     */ }
for _ in p.iter_mut()  { /* Change stuff */ }
for _ in p.into_iter() { /* Move stuff (unless items implement Copy) */ }
```

## Contributing

Any suggestions for the codebase, documentation, README (or anything) are more than welcome!

If there are any problems or queries, please submit an issue on our Github page.

## License

This crate uses the MIT license.
