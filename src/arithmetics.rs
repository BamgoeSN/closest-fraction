use std::{
    cmp::Ordering,
    ops::{Add, Mul, Shr},
};

use num::Unsigned;

pub struct Farey<T> {
    pub num: T,
    pub den: T,
}

impl<T> Clone for Farey<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            num: self.num.clone(),
            den: self.den.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.num = source.num.clone();
        self.den = source.den.clone();
    }
}

impl<T> PartialEq for Farey<T>
where
    T: Unsigned + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.den == other.den && self.num == other.num
    }
}

impl<T> PartialOrd for Farey<T>
where
    T: Unsigned + Clone + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = self.num.clone() * other.den.clone();
        let rhs = other.num.clone() * self.den.clone();
        Some(lhs.cmp(&rhs))
    }
}

fn push<T>(from: &Farey<T>, to: &Farey<T>, by: &T) -> Farey<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T>,
{
    Farey {
        num: from.num.clone() + to.num.clone() * by.clone(),
        den: from.den.clone() + to.den.clone() * by.clone(),
    }
}

fn is_beyond<T>(f1: &Farey<T>, f2: &Farey<T>, m: &T) -> bool
where
    T: Clone + Add<Output = T> + PartialOrd,
{
    f1.den.clone() + f2.den.clone() > *m
}

fn approx_right<T>(f1: &Farey<T>, f: &Farey<T>, f2: &Farey<T>, m: &T, allow_equal: bool) -> Farey<T>
where
    T: Clone + Unsigned + Ord + PartialOrd + Shr<Output = T>,
{
    if is_beyond(f1, f2, m) {
        return Farey {
            num: f2.num.clone(),
            den: f2.den.clone(),
        };
    }

    let (mut l, mut r): (T, T) = (
        T::zero(),
        (m.clone() - f2.den.clone() + f1.den.clone()) / f1.den.clone(),
    );

    while l.clone() + T::one() < r {
        let c = (l.clone() + r.clone()).shr(T::one());
        let m = push(&f2, f1, &c);

        match f.partial_cmp(&m) {
            Some(Ordering::Less) => {
                l = c;
            }
            Some(Ordering::Equal) => {
                if allow_equal {
                    l = c;
                } else {
                    r = c;
                }
            }
            Some(Ordering::Greater) => {
                r = c;
            }
            None => {
                panic!("Error with comparing two Fareys while performing binary search");
            }
        }
    }

    push(f2, f1, &l)
}

fn approx_left<T>(f1: &Farey<T>, f: &Farey<T>, f2: &Farey<T>, m: &T, allow_equal: bool) -> Farey<T>
where
    T: Clone + Unsigned + Ord + PartialOrd + Shr<Output = T>,
{
    if is_beyond(f1, f2, m) {
        return Farey {
            num: f1.num.clone(),
            den: f1.den.clone(),
        };
    }

    let (mut l, mut r): (T, T) = (
        T::zero(),
        (m.clone() - f1.den.clone() + f2.den.clone()) / f2.den.clone(),
    );

    while l.clone() + T::one() < r {
        let c = (l.clone() + r.clone()).shr(T::one());
        let m = push(&f1, f2, &c);

        match f.partial_cmp(&m) {
            Some(Ordering::Less) => {
                r = c;
            }
            Some(Ordering::Equal) => {
                if allow_equal {
                    l = c;
                } else {
                    r = c;
                }
            }
            Some(Ordering::Greater) => {
                l = c;
            }
            None => {
                panic!("Error with comparing two Fareys while performing binary search");
            }
        }
    }

    push(f1, f2, &l)
}

pub fn squeeze_right_first<T>(f1: &mut Farey<T>, f: &Farey<T>, f2: &mut Farey<T>, m: &T)
where
    T: Clone + Unsigned + Ord + Shr<Output = T>,
{
    loop {
        let nf2 = approx_right(f1, f, f2, m, true);
        let nf1 = approx_left(f1, f, &nf2, m, true);
        if f1 == &nf1 && f2 == &nf2 {
            break;
        }
        f1.clone_from(&nf1);
        f2.clone_from(&nf2);
    }
}
