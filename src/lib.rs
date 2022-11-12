use std::ops::{Add, Sub, Mul};
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]  
pub struct Quaternion {
    i: f64,
    j: f64,
    k: f64,
    l: f64,
}

impl Quaternion {
    pub fn new(alpha: f64, beta: f64, charlie: f64, delta: f64) -> Quaternion {
        Self {
            i: alpha,
            j: beta,
            k: charlie,
            l: delta,
        }
    }
    pub fn conj(&self) -> Quaternion {
        Self {
            i: self.i,
            j: -self.j,
            k: -self.k,
            l: -self.l,
        }
    }
    pub fn grassman_product(delta: Quaternion, echo: Quaternion) -> Quaternion {
        Self {
            i: delta.i * echo.i - delta.j * echo.j - delta.k * echo.k - delta.l * echo.l,
            j: delta.i * echo.j + delta.j * echo.i + delta.k * echo.l - delta.l * echo.k,
            k: delta.i * echo.k - delta.j * echo.l + delta.k * echo.i + delta.l * echo.j,
            l: delta.i * echo.l + delta.j * echo.k - delta.k * echo.j + delta.l * echo.i,
        }
    }
    pub fn cross_product(delta: Quaternion, echo: Quaternion) -> Quaternion {
        let b = Quaternion::new(0.5, 0.0, 0.0, 0.0);
        let q = Quaternion::grassman_product(delta, echo) - Quaternion::grassman_product(echo, delta);
        Quaternion::grassman_product(b, q)
    }
    pub fn real(&self) -> f64 {
        return self.i;
    }
    pub fn imag(&self) -> Vec<f64> {
        let mut v: Vec<f64> = Vec::new();
        v.push(self.j);
        v.push(self.k);
        v.push(self.l);
        return v;
    }
    pub fn abs(&self) -> f64 {
        (self.i.powf(2.0) + self.j.powf(2.0) + self.k.powf(2.0) + self.l.powf(2.0)).powf(0.5)
    }
    pub fn exchangeable(&self, alpha: &Quaternion) -> bool {
        let q: Quaternion = Quaternion::cross_product(*self, *alpha);
        if q.i == 0.0 && q.j == 0.0 && q.k == 0.0 && q.l == 0.0 {
            return true
        }
        return false
    } 
    pub fn unit(&self) -> Quaternion {
        Quaternion::divide_elementwise(self, self.abs())
    }
    pub fn divide_elementwise(&self, alpha: f64) -> Quaternion {
        let a = alpha.abs();
        Self {
            i: self.i / a,
            j: self.j / a,
            k: self.k / a,
            l: self.l / a,
        }
    }
}

impl Add for Quaternion {
    type Output = Quaternion;
    fn add(self, alpha: Quaternion) -> Quaternion {
        Self {
            i: self.i + alpha.i,
            j: self.j + alpha.j,
            k: self.k + alpha.k,
            l: self.l + alpha.l,
        }
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;
    fn sub(self, alpha: Quaternion) -> Quaternion {
        Self {
            i: self.i - alpha.i,
            j: self.j - alpha.j,
            k: self.k - alpha.k,
            l: self.l - alpha.l,
        }
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, alpha: Quaternion) -> Quaternion {
        Self {
            i: self.i * alpha.i - self.j * alpha.j - self.k * alpha.k - self.l * alpha.l,
            j: self.i * alpha.j + self.j * alpha.i + self.k * alpha.l - self.l * alpha.k,
            k: self.i * alpha.k - self.j * alpha.l + self.k * alpha.i + self.l * alpha.j,
            l: self.i * alpha.l + self.j * alpha.k - self.k * alpha.j + self.l * alpha.i,
        }
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {}, {})", self.i, self.j, self.k, self.l)
    }
}

#[cfg(test)]
mod test {
    use super::Quaternion;

    #[test]
    fn test_basic_calculations() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(-1.0, -2.0, -3.0, -4.0);
        let q3 = Quaternion::new(28.0, -4.0, -6.0, -8.0);
        assert_eq!(q1 + q2, Quaternion::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(q1 - q2, Quaternion::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(q1.conj(), Quaternion::new(1.0, -2.0, -3.0, -4.0));
        assert_eq!(Quaternion::grassman_product(q1, q2), q3);
        assert_eq!(q1 * q2, q3);
    }
    #[test]
    fn test_abs() {
        let q1 = Quaternion::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(q1.abs(), 2.0);
    }
    #[test]
    fn test_unit() {
        let q1 = Quaternion::new(14.0, -19.0, 9.0, -3.0);
        assert_eq!(q1.unit().abs(), 1.0);
    }
}
