mod arithmetics;

use std::ops::{DivAssign, Rem, Shr};

use arithmetics::*;
use num::{Num, Unsigned};

fn gcd<T>(x: T, y: T) -> T
where
    T: Num + Clone + PartialEq + Rem<Output = T>,
{
    let (mut a, mut b) = (x.clone(), y.clone());
    while b != T::zero() {
        let t = b.clone();
        b = a % b;
        a = t.clone();
    }
    a
}

pub fn get_closest_fraction<T>(num: &T, den: &T, max_den: &T) -> (Farey<T>, Farey<T>)
where
    T: Clone + Unsigned + Ord + PartialOrd + Shr<Output = T> + DivAssign,
{
    let (mut x, mut y) = (num.clone(), den.clone());
    let g = gcd(x.clone(), y.clone());
    x /= g.clone();
    y /= g.clone();
    let mut f1 = Farey {
        num: T::zero(),
        den: T::one(),
    };
    let mut f2 = Farey {
        num: T::one(),
        den: T::one(),
    };
    let f = Farey {
        num: x.clone(),
        den: y.clone(),
    };
    squeeze_right_first(&mut f1, &f, &mut f2, max_den);
    (f1, f2)
}
