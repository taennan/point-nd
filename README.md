
# PointND

A simple and flexible data structure for modelling points of **any** dimensions on an axis

## Basic Usage

### Constructing a Point

No matter how a PointND is constructed, the second generic arg must be filled with the number of dimensions it needs to have

If a point of zero dimensions is constructed, it will panic

```rust
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

### Accessing Values

It is recommended to use the convenience getters if the dimensions of the point are from ```1..=4```

```rust
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

```rust
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

### Querying Size

The number of dimensions can be retrieved using the ```dims()``` method (short for _dimensions_)

```rust
use point_nd::PointND;

let p: PointND<i32, 2> = PointND::fill(10);
assert_eq!(p.dims(), 2);
```

## Contributing

Any suggestions for the codebase, documentation, README (or anything) are more than welcome!

