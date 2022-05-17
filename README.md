
# PointND

A simple and flexible no-std struct to model points on n-dimensional axes

## Compatibility

This crate was designed to be `no_std` and `wasm` compatible, and has been 
tested in those environments.

This crate uses constant generics, it is recommended for use with a Rust version 
of **at least 1.51**.

## Basic Usage

As `PointND` dereferences to a slice, all methods 
implemented for slices are available with this

### Making a Point

```rust
// Creating a 2D point from a given array
let arr = [0,1];
let p = PointND::new(arr);

// Creating a 3D point from values of a given slice
let vec = vec![0, 1, 2];
let p = PointND::<_, 3>::from_slice(&vec);

// Creating a 4D point with all values set to 5
let p = PointND::<_, 4>::fill(5);
```

### Querying Values and Properties 

If the dimensions of the point are within `1..=4`, it is 
recommended to use the convenience getters for accessing values.

```rust
let p = PointND::new([0, 1]);

// As the point has 2 dimensions, we can access
//  it's values with the x() and y() methods
let x: &i32 = p.x();
let y = p.y();

assert_eq!(*x, arr[0]);
assert_eq!(*y, arr[1]);

// If the point had 3 dimensions, we could use the above and:
//  let z = p.z();
// Or with 4 dimensions, the above and:
//  let w = p.w();
```

The above methods are not implemented for PointND's with more than 4 dimensions. 

We must use indexing instead. See the [documentation][docs] for other crates which make 
direct indexing easier

```rust
let p = PointND::new([0,1,2,3,4,5]);

// ERROR: Not implemented for PointND of 6 dimensions
// let x = p.x();

let x: i32 = p[0];
let y_to_z = p[1..3];
```

To get the dimensions of a point, use the `dims` method.

```rust
let p = PointND::new([0, 1, 2, 3]);

let dims: usize = p.dims();
assert_eq!(dims, 4);
```

### Transforming Values

If the dimensions of the point are within `1..=4`, it is 
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
```

Complex transformations can be made via functions passed to the `apply`, 
`apply_vals`, `apply_dims` and `apply_point` methods. 

See the [documentation][docs] for more info.

```rust
let add_ten = |i: i32| i + 10;
let double  = |i: i32| i * 2;
let sum     = |a: i32, b: i32| a + b;

let p1 = PointND::new([0,1,2,3,4,5]);
let p2 = PointND::new([0,1,2,3,4,5])
         // Adds ten to each item
         .apply(add_ten)
         // Doubles items at indexes 0, 1 and 2
         .apply_dims(&[0,1,2], double)
         // Adds items in p2 to respective items in p1
         .apply_point(p1, sum);
```

### Iterating

Iterating over a `PointND` is as easy as:

```rust
let mut p = PointND::new([0,1]);

for _ in p.iter()      { /* Do stuff     */ }
for _ in p.iter_mut()  { /* Change stuff */ }
for _ in p.into_iter() { /* Move stuff (unless items implement Copy) */ }
```

## Contributing

Any suggestions for the codebase, documentation, README (or anything) are more than welcome!

If there are any problems or queries, please submit an issue in our [GitHub repo][repo].

## API Changes

Breaking API changes are still a possibility in the future. However, as of `v0.5.0` this
has become far less likely and future major releases (if any) will most likely add 
functionality instead of revamping existing ones.

The [full changelog][changelog] can be found in our GitHub repo.

Our [GitHub repo][repo] is always a few steps ahead of the version available on [crates.io][crate],
so it may be worth checking for sweet new features and bugfixes.

## License

This crate is available under the [`MIT`][mit-license] 
and/or [`Apache2.0`][apache-license] licenses.


[docs]: https://docs.rs/point-nd/0.5.0/point_nd/

[repo]: https://github.com/taennan/point-nd/tree/main
[changelog]: https://github.com/taennan/point-nd/blob/main/CHANGELOG.md
[mit-license]: https://github.com/taennan/point-nd/blob/main/LICENSE-MIT
[apache-license]: https://github.com/taennan/point-nd/blob/main/LICENSE-APACHE

[crate]: https://crates.io/crates/point-nd

[axmac]: https://crates.io/crates/axmac