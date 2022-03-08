
use std::{
    ops::{Add, Sub, Mul, Div},
    convert::TryInto,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PointAD<T, const N: usize>
    where T: Clone + Copy + Default {
    arr: [T; N],
}

impl<T, const N: usize>  PointAD<T, N>
    where T: Clone + Copy + Default {

    pub fn from(slice: &[T]) -> Self {
        if slice.len() == 0 {
            panic!("Cannot construct Point with zero dimensions");
        }
        let arr: [T; N] = slice.try_into().unwrap();
        PointAD{ arr }
    }

    pub fn of_dimes(d: usize) -> Self {
        let arr = vec![T::default(); d];
        PointAD::from(&arr)
    }

    pub fn dimes(&self) -> usize {
        self.arr.len()
    }

    pub fn try_get(&self, i: usize) -> Option<&T> {
        self.arr.get(i)
    }

    pub fn get(&self, i: usize) -> &T {
        self.try_get(i).unwrap()
    }

    pub fn as_arr(&self) -> [T; N] {
        self.arr
    }

    pub fn as_vec(&self) -> Vec<T> {
        Vec::from(&self.arr[..])
    }

}

impl<T, const N: usize> Add for PointAD<T, N> where T: Add<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if &self.dimes() != &rhs.dimes() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values= [T::default(); N];
        for i in 0..ret_values.len() {
            ret_values[i] = values_left[i] + values_right[i];
        }

        PointAD::<T, N>::from(&ret_values)
    }

}
impl<T, const N: usize> Sub for PointAD<T, N> where T: Sub<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if &self.dimes() != &rhs.dimes() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values= [T::default(); N];
        for i in 0..ret_values.len() {
            ret_values[i] = values_left[i] - values_right[i];
        }

        PointAD::<T, N>::from(&ret_values)
    }

}
impl<T, const N: usize> Mul for PointAD<T, N> where T: Mul<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if &self.dimes() != &rhs.dimes() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values= [T::default(); N];
        for i in 0..ret_values.len() {
            ret_values[i] = values_left[i] * values_right[i];
        }

        PointAD::<T, N>::from(&ret_values)
    }

}
impl<T, const N: usize> Div for PointAD<T, N> where T: Div<Output = T> + Clone + Copy + Default {

    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if &self.dimes() != &rhs.dimes() { panic!("Tried to add two PointND's of unequal length"); }

        let values_left= self.as_arr();
        let values_right = rhs.as_arr();

        let mut ret_values= [T::default(); N];
        for i in 0..ret_values.len() {
            ret_values[i] = values_left[i] / values_right[i];
        }

        PointAD::<T, N>::from(&ret_values)
    }

}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod constructor {

        use crate::*;

        #[test]
        fn constructable_with_from_function() {
            let vec = vec![1,2,3,4];

            let _p = PointAD::<_, 4>::from(&vec);
            let _p = PointAD::<_, 3>::from(&vec[..3]);
        }

        #[test]
        fn constructable_with_of_d_function() {
            let _p = PointAD::<i32, 2>::of_dimes(2);
        }

        #[test]
        #[should_panic]
        fn cant_construct_with_0_dimensions() {
            let _p = PointAD::<u8, 0>::from(&[]);
        }

    }

    #[cfg(test)]
    mod dimensions {

        use crate::*;

        #[test]
        fn returns_correct_dimensions() {
            let vec = vec![0,1,2,3];
            let p = PointAD::<_, 4>::from(&vec);

            assert_eq!(p.dimes(), vec.len());
        }

    }

    #[cfg(test)]
    mod values {

        use crate::*;

        #[test]
        fn returns_value_on_get() {
            let vec = vec![0,1,2,3];
            let p = PointAD::<_, 4>::from(&vec);

            for i in 0..vec.len() {
                assert_eq!(p.get(i), &vec[i]);
            }
        }

        #[test]
        fn changing_input_vec_doesnt_change_arr_value() {
            let mut vec = vec![0,1,2,3];
            let p = PointAD::<_, 4>::from(&vec);

            for i in 0..vec.len() {
                vec[i] = (vec[i] + 1) * 2;
                assert_ne!(p.get(i), &vec[i]);
            }
        }

    }

    #[cfg(test)]
    mod operators {

        use crate::*;

        #[test]
        fn can_add_two_points() {
            let vec = vec![0,1,2,3];
            let p1 = PointAD::<_, 4>::from(&vec);
            let p2 = PointAD::from(&vec);

            let p3 = p1 + p2;
            for (a, b) in p3.as_arr().into_iter().zip(vec){
                assert_eq!(a, b + b);
            }
        }

        #[test]
        fn can_subtract_two_points() {
            let vec = vec![0,1,2,3];
            let p1 = PointAD::<_, 4>::from(&vec);
            let p2 = PointAD::from(&vec);

            let p3 = p1 - p2;
            for (a, b) in p3.as_arr().into_iter().zip(vec){
                assert_eq!(a, b - b);
            }
        }

        #[test]
        fn can_multiply_two_points() {
            let vec = vec![0,1,2,3];
            let p1 = PointAD::<_, 4>::from(&vec);
            let p2 = PointAD::from(&vec);

            let p3 = p1 * p2;
            for (a, b) in p3.as_arr().into_iter().zip(vec){
                assert_eq!(a, b * b);
            }
        }

        #[test]
        fn can_divide_two_points() {
            let vec = vec![1,2,3,4];
            let p1 = PointAD::<_, 4>::from(&vec);
            let p2 = PointAD::from(&vec);

            let p3 = p1 / p2;
            for (a, b) in p3.as_arr().into_iter().zip(vec){
                assert_eq!(a, b / b);
            }
        }

        #[test]
        #[should_panic]
        fn cannot_divide_two_points_if_one_item_is_zero() {
            let vec = vec![0, 1,2,3,4];
            let p1 = PointAD::<_, 5>::from(&vec);
            let p2 = PointAD::from(&vec);

            let p3 = p1 / p2;
            for (a, b) in p3.as_arr().into_iter().zip(vec){
                assert_eq!(a, b / b);
            }
        }

    }

}