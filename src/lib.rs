use std::ops::{Add, Sub, Mul, Neg};

#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug)]  
pub struct Quaternion<T> {
    i: T,
    j: T,
    k: T,
    l: T,
}

impl<T: Copy + Sub<Output=T> + Neg<Output=T> + Add<Output=T> + Mul<Output=T> + std::cmp::PartialEq> Quaternion<T> {
    pub fn new(alpha: T, beta: T, charlie: T, delta: T) -> Quaternion<T> {
        Self {
            i: alpha,
            j: beta,
            k: charlie,
            l: delta,
        }
    }
    pub fn conj(&self) -> Quaternion<T> {
        Self {
            i: self.i,
            j: -self.j,
            k: -self.k,
            l: -self.l,
        }
    }
    pub fn grassman_product(delta: Quaternion<T>, echo: Quaternion<T>) -> Quaternion<T> {
        Self {
            i: delta.i * echo.i - delta.j * echo.j - delta.k * echo.k - delta.l * echo.l,
            j: delta.i * echo.j + delta.j * echo.i + delta.k * echo.l - delta.l * echo.k,
            k: delta.i * echo.k - delta.j * echo.l + delta.k * echo.i + delta.l * echo.j,
            l: delta.i * echo.l + delta.j * echo.k - delta.k * echo.j + delta.l * echo.i,
        }
    }
    pub fn cross_product(delta: Quaternion<T>, echo: Quaternion<T>) -> Quaternion<T> {
        // can I use non generic values in generic implementation
        let b = Quaternion::new(0.5, 0.0, 0.0, 0.0);
        let q = Quaternion::grassman_product(delta, echo) - Quaternion::grassman_product(echo, delta);
        Quaternion::grassman_product(b, q)
    }
    pub fn real(&self) -> T {
        return self.i;
    }
    pub fn imag(&self) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        v.push(self.j);
        v.push(self.k);
        v.push(self.l);
        return v;
    }
    pub fn exchangeable(&self, alpha: Quaternion<T>) -> bool {
        let q = Quaternion::new(0, 0, 0, 0);
        if Quaternion::cross_product(*self, alpha) == q {
            return true;
        } else {
            return false;
        }
    } 
}

impl<T: Add<Output=T>> Add<Quaternion<T>> for Quaternion<T> {
    type Output = Quaternion<T>;
    fn add(self, alpha: Quaternion<T>) -> Quaternion<T> {
        Self {
            i: self.i + alpha.i,
            j: self.j + alpha.j,
            k: self.k + alpha.k,
            l: self.l + alpha.l,
        }
    }
}

impl<T: Sub<Output=T>> Sub<Quaternion<T>> for Quaternion<T> {
    type Output = Quaternion<T>;
    fn sub(self, alpha: Quaternion<T>) -> Quaternion<T> {
        Self {
            i: self.i - alpha.i,
            j: self.j - alpha.j,
            k: self.k - alpha.k,
            l: self.l - alpha.l,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Quaternion;

    #[test]
    fn test_basic_calculations() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(-1.0, -2.0, -3.0, -4.0);
        let q4 = Quaternion::new(28.0, -4.0, -6.0, -8.0);
        assert_eq!(q1 + q2, Quaternion::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(q1 - q2, Quaternion::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(q1.conj(), Quaternion::new(1.0, -2.0, -3.0, -4.0));
        assert_eq!(Quaternion::grassman_product(q1, q2), q4);
    }
}

