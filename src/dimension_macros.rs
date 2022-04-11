#![cfg(feature = "dim_macros")]

/**
 Converts an identifier _x_, _y_, _z_ or _w_ to a ```usize``` value for indexing collections.

 Using any identifier apart from the above or multiple identifiers will result in a compile time error.

 It is recommended to use parentheses when calling this macro for clarity.

## Possible Variations

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
 let p = PointND::from([0,1,2]);
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
 let five_d_point = PointND::from([0,1,2,3,4]);
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

 ## Possible Variations

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dims;
 // Explicitly specify items in array
 let arr =  dims![x, y, z, w];
 assert_eq!(arr, [0, 1, 2, 3]);

 // Copy specified item N times
 let arr =  dims![w; 5];
 assert_eq!(arr, [3, 3, 3, 3, 3]);
 # }
 ```

 Using identifiers multiple times is allowed, this is only
 a more readable way to specify indexes after all

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dims;
 let index_arr =  dims![x,x, y,y, z,z];
 assert_eq!(index_arr, [0,0, 1,1, 2,2]);
 # }
 ```

 This can be especially useful with the ```apply_dims``` method available to ```PointND```'s

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dims;
 # use point_nd::PointND;
 let p = PointND
     ::from([0,1,2,3])
     .apply_dims(&dims![y,w], |item| item * 2)   // Multiplies items 1 and 3 by 2
     .apply_dims(&dims![x,z], |item| item + 10); // Adds 10 to items 0 and 2
 assert_eq!(p.into_arr(), [10, 2, 12, 6]);
 # }
 ```
 */
#[macro_export]
macro_rules! dims {

    // [x, y, y, z]
    ( $( $d:ident ), * ) => { [ $( dim!($d), )* ] };

    // [z; 3]
    ( $d:ident; $i:expr ) => { [dim!($d); $i] };

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

 // Range with expression and identifier
 //  The parentheses around the expression are compulsory
 assert_eq!(dimr!((0)..z), 0..2usize);

 // RangeInclusive with expression and identifier
 //  The parentheses around the expression are compulsory
 assert_eq!(dimr!((1)..=w), 1..=3usize);
 # }
 ```

 This is especially useful when taking slices of a ```PointND```

 ```
 # #[macro_use] extern crate point_nd; fn main() {
 # use point_nd::dimr;
 # use point_nd::PointND;
 let p = PointND::from([0,1,2,3,4,5]);
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
    //  Range z..6
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

    // Expr to Ident
    //  Range 0..z
    ( ($a:expr)..$b:ident )  => { $a..dim!($b) };
    // RangeInclusive 1..=w
    ( ($a:expr)..=$b:ident )  => { $a..=dim!($b) };

}

#[cfg(test)]
mod tests {

    #[test]
    fn dim_works() {
        assert_eq!(dim!(x), 0);
        assert_eq!(dim!(y), 1);
        assert_eq!(dim!(z), 2);
        assert_eq!(dim!(w), 3);
    }

    #[test]
    fn dims_works() {
        assert_eq!(dims![x,y,z,w], [0,1,2,3]);
        assert_eq!(dims![x,z,y],   [0,2,1]);
        assert_eq!(dims![x,y,x,y], [0,1,0,1]);
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

    #[test]
    fn dimr_expr_to_ident_works() {
        let arr = [0,1,2,3,4];
        let slice = &arr[dimr!((0)..z)];
        assert_eq!(*slice, [0,1]);

        let expr = 1usize;
        let slice = &arr[dimr!((expr)..w)];
        assert_eq!(*slice, [1, 2]);
    }

    #[test]
    fn dimr_expr_to_eq_ident_works() {
        let arr = [0,1,2,3,4];
        let slice = &arr[dimr!((0)..=z)];
        assert_eq!(*slice, [0,1,2]);

        let expr = 1usize;
        let slice = &arr[dimr!((expr)..=w)];
        assert_eq!(*slice, [1,2,3]);
    }

}