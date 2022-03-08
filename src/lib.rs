

use std::convert::TryInto;
use std::ops::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PointND<T> where T: Clone + Copy + Default {
    vec: Vec<T>
}

impl<T> PointND<T> where T: Clone + Copy + Default {

    pub fn from(vec: Vec<T>) -> Self {
        if vec.len() == 0 { panic!("") }
        PointND{ vec }
    }

    pub fn of_dimes(d: usize) -> Self {
        if d == 0 { panic!(""); }
        PointND{ vec: vec![T::default(); d] }
    }


    pub fn dimes(&self) -> usize {
        self.vec.len()
    }

    pub fn has_same_dimes(&self, other: &Self) -> bool {
        self.dimes() == other.dimes()
    }


    pub fn try_get(&self, i: usize) -> Option<&T> {
        self.vec.get(i)
    }

    pub fn get(&self, i: usize) -> &T {
        self.try_get(i)
            .expect(&format!("Tried to access value in dimension {} when PointND has only {} dimensions", i, self.dimes()))
    }

    pub fn as_vec(&self) -> Vec<T> {
        self.vec.clone()
    }

    pub fn as_arr<const N: usize>(&self) -> Result<[T; N], ()> {
        let res: Result<[T; N], Vec<T>> = self.vec.clone().try_into();
        match res {
            Ok(arr) => Ok(arr),
            _ => Err(())
        }
    }

}


impl<T> Add for PointND<T> where T: Add<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if !&self.has_same_dimes(&rhs) { panic!("Tried to add two PointND's of unequal length"); }

        PointND::from(
            self.vec
                .iter()
                .zip(rhs.as_vec().iter())
                .map(|(l, r)| *l + *r)
                .collect()
        )
    }

}
impl<T> Sub for PointND<T> where T: Sub<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if !&self.has_same_dimes(&rhs) { panic!("Tried to subtract two PointND's of unequal length"); }

        PointND::from(
            self.vec
                .iter()
                .zip(rhs.as_vec().iter())
                .map(|(l, r)| *l - *r)
                .collect()
        )
    }

}
impl<T> Div for PointND<T> where T: Div<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if !&self.has_same_dimes(&rhs) { panic!("Tried to divide two PointND's of unequal length"); }

        PointND::from(
            self.vec
                .iter()
                .zip(rhs.as_vec().iter())
                .map(|(l, r)| *l / *r)
                .collect()
        )
    }

}
impl<T> Mul for PointND<T> where T: Mul<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if !&self.has_same_dimes(&rhs) { panic!("Tried to multiply two PointND's of unequal length"); }

        PointND::from(
            self.vec
                .iter()
                .zip(rhs.as_vec().iter())
                .map(|(l, r)| *l * *r)
                .collect()
        )
    }

}

#[cfg(test)]
mod tests {

    use crate::PointND;

    #[test]
    fn returns_correct_dimensions_from_of_dimes_constructor() {
        let vec = vec![1,2,3,4];
        let dimes = vec.len();

        let p = PointND::<i32>::of_dimes(dimes);
        assert_eq!(p.dimes(), dimes);
    }

    #[test]
    fn returns_correct_dimensions_from_from_constructor() {
        let vec = vec![1,2,3,4];
        let dimes = vec.len();

        let p = PointND::from(vec);
        assert_eq!(p.dimes(), dimes);
    }

    #[test]
    fn returns_vector() {
        let vec = vec![0,1,2,3];
        let p1 = PointND::from(vec.clone());
        assert_eq!(p1.as_vec(), vec);
    }

    #[test]
    fn adds() {
        let v1 = vec![23, 45, 2];
        let p1 = PointND::from(v1.clone());

        let v2 = vec![34, 78, 1];
        let p2 = PointND::from(v2.clone());

        let p3 = p1 + p2;

        assert_eq!(p3.as_vec(), vec![v1[0] + v2[0], v1[1] + v2[1],  v1[2] + v2[2]])
    }

    #[test]
    #[should_panic]
    fn cant_add_points_of_unequal_length() {
        let v1 = vec![23, 45, 2];
        let p1 = PointND::from(v1.clone());

        let v2 = vec![34, 78, 1, 234];
        let p2 = PointND::from(v2.clone());

        let _p3 = p1 + p2;
    }

    #[test]
    fn can_be_compared() {
        let v = vec![0,1,2];
        let p1 = PointND::from(v);
        let p2 = p1.clone();
        let p3 = PointND::from(vec![2,1,0]);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn returns_a_primitive_nd_array() {
        let arr: [f64; 7] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let p = PointND::from(Vec::from(&arr[..]));

        assert_eq!(p.as_arr().unwrap(), arr);
    }

    #[test]
    fn returns_a_non_primitive_nd_array() {

        #[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
        struct Filler {}
        impl Filler { pub fn new() -> Self { Filler{} } }

        let arr = [Filler::new(); 10];
        let p = PointND::from(Vec::from(&arr[..]));

        assert_eq!(p.as_arr().unwrap(), arr);
    }

}
