
use point_nd::PointND;
use core::ops::Mul;

#[test]
fn can_use_external_fn_pointers_for_appliers() {

    fn square<T>(x: T) -> T
        where T: Mul<Output = T> + Copy {
        x * x
    }

    let p = PointND
        ::from([0,1,2,3])
        .apply(square);
    assert_eq!(p.into_arr(), [0, 1, 4, 9]);

}