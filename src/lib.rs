use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PointND<T, const N: usize>([T; N])
    where T: Clone + Copy;

impl<T, const N: usize> PointND<T, N>
    where T: Clone + Copy {

    pub fn from(slice: &[T]) -> Self {
        if slice.len() == 0 {
            panic!("Cannot construct Point with zero dimensions");
        }
        let arr: [T; N] = slice.try_into().unwrap();
        PointND(arr)
    }

    pub fn fill(value: T) -> Self {
        PointND::<T, N>::from(&[value; N])
    }

    pub fn dims(&self) -> usize {
        self.len()
    }

    pub fn apply<F>(self, modifier: F) -> Result<Self, ()>
        where F: Fn(T) -> Result<T, ()> {

        let mut vec = Vec::<T>::with_capacity(N);
        for item in self.into_iter() {
            let fn_result = modifier(item)?;
            vec.push(fn_result);
        }

        Ok( PointND::<T, N>::from(&vec) )
    }

    pub fn apply_dims<F>(self, dims: &[usize], modifier: F) -> Result<Self, ()>
        where F: Fn(T) -> Result<T, ()> {

        let mut vec = Vec::<T>::with_capacity(N);
        for (i, item) in self.into_iter().enumerate() {
            if dims.contains(&i) {
                vec.push(modifier(item)?);
            } else {
                vec.push(item);
            }
        }

        Ok( PointND::<T, N>::from(&vec) )
    }

    pub fn apply_vals<F>(self, values: [T; N], modifier: F) -> Result<Self, ()>
        where F: Fn(T, T) -> Result<T, ()> {

        let mut vec = Vec::<T>::with_capacity(N);
        for (a, b) in self.into_iter().zip(values) {
            vec.push(modifier(a, b)?);
        }

        Ok( PointND::<T, N>::from(&vec) )
    }

    pub fn apply_with<F>(self, other: PointND<T, N>, modifier: F) -> Result<Self, ()>
        where F: Fn(T, T) -> Result<T, ()> {

        self.apply_vals(other.into_arr(), modifier)
    }


    pub fn into_arr(self) -> [T; N] {
        self.0.clone()
    }

    pub fn into_vec(self) -> Vec<T> {
        Vec::from(&self[..])
    }

}

impl<T, const N: usize> Deref for PointND<T, N>
    where T: Clone + Copy {

    type Target = [T; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }

}
impl<T, const N: usize> DerefMut for PointND<T, N>
    where T: Clone + Copy {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }

}


impl<T> PointND<T, 1>
    where T: Clone + Copy  {

    pub fn x(&self) -> &T { &self[0] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }

}
impl<T> PointND<T, 2>
    where T: Clone + Copy  {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }

}
impl<T> PointND<T, 3>
    where T: Clone + Copy  {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }
    pub fn z(&self) -> &T { &self[2] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }
    pub fn set_z(&mut self, new_value: T) { self[2] = new_value; }

}
impl<T> PointND<T, 4>
    where T: Clone + Copy  {

    pub fn x(&self) -> &T { &self[0] }
    pub fn y(&self) -> &T { &self[1] }
    pub fn z(&self) -> &T { &self[2] }
    pub fn w(&self) -> &T { &self[3] }

    pub fn set_x(&mut self, new_value: T) { self[0] = new_value; }
    pub fn set_y(&mut self, new_value: T) { self[1] = new_value; }
    pub fn set_z(&mut self, new_value: T) { self[2] = new_value; }
    pub fn set_w(&mut self, new_value: T) { self[3] = new_value; }

}


#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn blank() {
    }

    #[test]
    fn can_iter() {

        let arr = [0, 1, 2, 3];

        let p = PointND::<u8, 4>::from(&arr);
        for (i, item) in p.iter().enumerate() {
            assert_eq!(arr[i], *item);
        }

        let mut p = PointND::<u8, 4>::from(&arr);
        for item in p.iter_mut() {
            *item = 10;
        }

        for i in p.into_iter() {
            assert_eq!(i, 10u8);
        }

    }

}