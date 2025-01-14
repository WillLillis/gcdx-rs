use std::fmt::Display;
use std::ops::Rem;

pub trait Zero {
    const ZERO: Self;
}
impl Zero for i8 {
    const ZERO: Self = 0;
}
impl Zero for i16 {
    const ZERO: Self = 0;
}
impl Zero for i32 {
    const ZERO: Self = 0;
}
impl Zero for i128 {
    const ZERO: Self = 0;
}
impl Zero for u8 {
    const ZERO: Self = 0;
}
impl Zero for u16 {
    const ZERO: Self = 0;
}
impl Zero for u32 {
    const ZERO: Self = 0;
}
impl Zero for u64 {
    const ZERO: Self = 0;
}
impl Zero for u128 {
    const ZERO: Self = 0;
}

fn gcd<T>(a: T, b: T) -> T
where
    T: Rem<Output = T> + PartialOrd + Zero + Copy + Display,
{
    let mut ac = a;
    let mut bc = b;
    let mut tmp = b;
    loop {
        let check = ac % bc;
        // println!("{} {} {}", ac, bc, check);
        if check != T::ZERO {
            tmp = check;
            ac = bc;
            bc = tmp;
        } else {
            break;
        }
    }
    tmp
}

pub fn gcdx<T>(values: &[T]) -> Option<T>
where
    T: Rem<Output = T> + PartialOrd + Zero + Copy + Display + Clone,
{
    if values.len() > 0 {
        let mut m = values[0];
        for i in 1..values.len() {
            m = gcd(m, values[i]);
            // println!("{}", m);
        }
        Some(m)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run() {
        let v = vec![10];
        let g = gcdx(&v).unwrap();
        println!("{}", g);
        assert_eq!(g, 10);

        let v = vec![10, 9];
        let g = gcdx(&v).unwrap();
        println!("{}", g);
        assert_eq!(g, 1);

        let v = vec![10, 9, 8, 7];
        let g = gcdx(&v).unwrap();
        println!("{}", g);
        assert_eq!(g, 1);

        let v: Vec<u32> = vec![120, 168, 328, 624, 320];
        let g = gcdx(&v).unwrap();
        println!("{}", g);
        assert_eq!(g, 8);

        let v: Vec<u32> = vec![1566429, 1566930, 1570805, 1563941, 1566387];
        let g = gcdx(&v).unwrap();
        println!("{}", g);
        assert_eq!(g, 1);

        let v: Vec<u32> = vec![1774071, 1360754, 1571542, 1830161, 1302721];
        let g = gcdx(&v).unwrap();
        println!("{}", g);
        assert_eq!(g, 1);

        let v: Vec<u32> = vec![2228668932, 825805579, 1955783521, 1173124319, 1234171242];
        let g = gcdx(&v).unwrap();
        println!("{}", g);
        assert_eq!(g, 1);
    }
}
