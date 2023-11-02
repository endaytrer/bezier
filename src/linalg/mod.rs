use std::ops::{Add, Mul, Sub};

use num::{traits::Float, Zero, One};

pub trait Linear<T: Float>: Clone + Copy + Mul<T, Output = Self> + Add<Self, Output = Self> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Null {}
impl <T: Float> Linear<T> for Null {}

impl Add for Null {
    type Output = Null;

    fn add(self, _rhs: Self) -> Self::Output {
        Null {}
    }
}
impl <T: Float> Mul<T> for Null {
    type Output = Null;

    fn mul(self, _rhs: T) -> Self::Output {
        Null {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BVec<T: Float, const N: usize> {
    pub v: [T; N]
}

pub type Vec2 = BVec<f32, 2>;
pub type Vec3 = BVec<f32, 3>;
pub type Vec4 = BVec<f32, 4>;

impl Linear<f32> for f32 {}
impl Linear<f64> for f64 {}

impl <T: Float, const N: usize> Linear<T> for BVec<T, N> {}
// initializer
impl <T: Float> BVec<T, 2> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            v: [x, y]
        }
    }
}
impl <T: Float> BVec<T, 3> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            v: [x, y, z]
        }
    }
}
impl <T: Float> BVec<T, 4> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            v: [x, y, z, w]
        }
    }
}

// x, y, z, w, norm, normalize, bitwise multiplication
impl <T: Float, const N: usize> BVec<T, N> {
    pub fn x(&self) -> T {
        self.v[0]
    }
    pub fn y(&self) -> T {
        self.v[1]
    }
    pub fn z(&self) -> T {
        self.v[2]
    }
    pub fn w(&self) -> T {
        self.v[3]
    }
    pub fn xy(&self) -> BVec<T, 2> {
        BVec { v: [self.x(), self.y()] }
    }
    pub fn xz(&self) -> BVec<T, 2> {
        BVec { v: [self.x(), self.z()] }
    }
    pub fn xw(&self) -> BVec<T, 2> {
        BVec { v: [self.x(), self.w()] }
    }
    pub fn yz(&self) -> BVec<T, 2> {
        BVec { v: [self.y(), self.z()] }
    }
    pub fn yw(&self) -> BVec<T, 2> {
        BVec { v: [self.y(), self.w()] }
    }
    pub fn zw(&self) -> BVec<T, 2> {
        BVec { v: [self.z(), self.w()] }
    }
    pub fn xyz(&self) -> BVec<T, 3> {
        BVec { v: [self.x(), self.y(), self.z()] }
    }
    pub fn xyw(&self) -> BVec<T, 3> {
        BVec { v: [self.x(), self.y(), self.w()] }
    }
    pub fn xzw(&self) -> BVec<T, 3> {
        BVec { v: [self.x(), self.z(), self.w()] }
    }
    pub fn yzw(&self) -> BVec<T, 3> {
        BVec { v: [self.y(), self.z(), self.w()] }
    }
    pub fn xyzw(&self) -> BVec<T, 4> {
        BVec { v: [self.x(), self.y(), self.z(), self.w()] }
    }

    pub fn norm(&self) -> T {
        let mut ans = T::zero();
        for i in 0..N {
            ans = ans + self.v[i] * self.v[i];
        }
        ans.sqrt()
    }
    pub fn normalize(&self) -> BVec<T, N> {
        return self.clone() * (T::one() / self.norm())
    }
    pub fn star(&self, rhs: &Self) -> BVec<T, N> {
        let mut ans = BVec::zero();
        for i in 0..N {
            ans.v[i] = self.v[i] * rhs.v[i];
        }
        ans
    }
}

impl <T: Float, const N: usize> Zero for BVec<T, N> {
    fn zero() -> Self {
        BVec { v: [T::zero(); N] }
    }

    fn is_zero(&self) -> bool {
        for i in 0..N {
            if !self.v[i].is_zero() {
                return false;
            }
        }
        true
    }
}

impl <T: Float, const N: usize> Add for BVec<T, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut ans: BVec<T, N> = BVec::zero();
        for i in 0..N {
            ans.v[i] = self.v[i] + rhs.v[i];
        }
        ans
    }
}
impl <T: Float, const N: usize> Sub for BVec<T, N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut ans: BVec<T, N> = BVec::zero();
        for i in 0..N {
            ans.v[i] = self.v[i] - rhs.v[i];
        }
        ans
    }
}
impl <T: Float, const N: usize> Mul<T> for BVec<T, N> {
    type Output = BVec<T, N>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut ans = BVec::zero();
        for i in 0..N {
            ans.v[i] = self.v[i] * rhs;
        }
        ans
    }
}
// dot product, for consistency of matrix products
impl <T: Float, const N: usize> Mul for BVec<T, N> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        
        let mut ans = T::zero();
        for i in 0..N {
            ans = ans + self.v[i] + rhs.v[i];
        }
        ans
    }
}
// cross product
impl <T: Float> BVec<T, 3> {
    pub fn cross(&self, other: BVec<T, 3>) -> BVec<T, 3> {

        BVec::<T, 3> { v: [
            self.v[1] * other.v[2] - self.v[2] * other.v[1],
            self.v[2] * other.v[0] - self.v[0] * other.v[2],
            self.v[0] * other.v[1] - self.v[1] * other.v[0],
        ]}
    }
}
// friends
impl <const N: usize> Mul<BVec<f32, N>> for f32 {
    type Output = BVec<f32, N>;

    fn mul(self, rhs: BVec<f32, N>) -> Self::Output {
        rhs * self
    }
}
impl <const N: usize> Mul<BVec<f64, N>> for f64 {
    type Output = BVec<f64, N>;

    fn mul(self, rhs: BVec<f64, N>) -> Self::Output {
        rhs * self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BMatrix<T: Float, const M: usize, const N: usize> {
    pub v: [[T; N]; M]
}

pub type Matrix2 = BMatrix<f32, 2, 2>;
pub type Matrix22 = BMatrix<f32, 2, 2>;
pub type Matrix23 = BMatrix<f32, 2, 3>;
pub type Matrix24 = BMatrix<f32, 2, 4>;
pub type Matrix32 = BMatrix<f32, 3, 2>;
pub type Matrix3 = BMatrix<f32, 3, 3>;
pub type Matrix33 = BMatrix<f32, 3, 3>;
pub type Matrix34 = BMatrix<f32, 3, 4>;
pub type Matrix42 = BMatrix<f32, 4, 2>;
pub type Matrix43 = BMatrix<f32, 4, 3>;
pub type Matrix4 = BMatrix<f32, 4, 4>;
pub type Matrix44 = BMatrix<f32, 4, 4>;
// transpose
impl <T: Float, const M: usize, const N: usize> BMatrix<T, M, N> {
    pub fn transpose(&self) -> BMatrix<T, N, M> {
        let mut ans: BMatrix<T, N, M> = BMatrix::zero();
        for i in 0..M {
            for j in 0..N {
                ans.v[j][i] = self.v[i][j];
            }
        }
        ans
    }
}
pub trait Det<T: Float> {
    fn det(&self) -> T;
}
// determinant
impl <T: Float> Det<T> for BMatrix<T, 1, 1> {
    fn det(&self) -> T {
        self.v[0][0]
    }
}
impl <T: Float> Det<T> for BMatrix<T, 2, 2> {
    fn det(&self) -> T {
        self.v[0][0] * self.v[1][1] - self.v[0][1] * self.v[1][0]
    }
}
impl <T: Float> Det<T> for BMatrix<T, 3, 3> {
    fn det(&self) -> T {
          self.v[0][0] * self.v[1][1] * self.v[2][2]
        - self.v[0][0] * self.v[1][2] * self.v[2][1]
        + self.v[0][1] * self.v[1][2] * self.v[2][0]
        - self.v[0][1] * self.v[1][0] * self.v[2][2]
        + self.v[0][2] * self.v[1][0] * self.v[2][1]
        - self.v[0][2] * self.v[1][1] * self.v[2][0]
    }
}

impl <T: Float, const M: usize, const N: usize> Zero for BMatrix<T, M, N> {
    fn zero() -> Self {
        BMatrix { v: [[T::zero(); N]; M] }
    }

    fn is_zero(&self) -> bool {
        for i in 0..M {
            for j in 0..N {
                if !self.v[i][j].is_zero() {
                    return false;
                }
            }
        }
        true
    }
}
impl <T: Float, const N: usize> One for BMatrix<T, N, N> {
    fn one() -> Self {
        let mut ans: BMatrix<T, N, N> = BMatrix::zero();
        for i in 0..N {
            ans.v[i][i] = T::one();
        }
        ans
    }
}
impl <T: Float, const M: usize, const N: usize> Add for BMatrix<T, M, N> {
    type Output = BMatrix<T, M, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ans: BMatrix<T, M, N> = BMatrix::zero();
        for i in 0..M {
            for j in 0..N {
                ans.v[i][j] = self.v[i][j] + rhs.v[i][j];
            }
        }
        ans
    }
}
impl <T: Float, const M: usize, const N: usize> Sub for BMatrix<T, M, N> {
    type Output = BMatrix<T, M, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ans: BMatrix<T, M, N> = BMatrix::zero();
        for i in 0..M {
            for j in 0..N {
                ans.v[i][j] = self.v[i][j] - rhs.v[i][j];
            }
        }
        ans
    }
}
// matrix * scalar
impl <T: Float, const M: usize, const N: usize> Mul<T> for BMatrix<T, M, N> {
    type Output = BMatrix<T, M, N>;
    fn mul(self, rhs: T) -> Self::Output {
        let mut ans: BMatrix<T, M, N> = BMatrix::zero();
        for i in 0..M {
            for j in 0..N {
                ans.v[i][j] = self.v[i][j] * rhs;
            }
        }
        ans
    }
}
// matrix * vector
impl <T: Float, const M: usize, const N: usize> Mul<BVec<T, N>> for BMatrix<T, M, N> {
    type Output = BVec<T, M>;
    fn mul(self, rhs: BVec<T, N>) -> Self::Output {
        let mut ans: BVec<T, M> = BVec::zero();
        for i in 0..M {
            for j in 0..N {
                ans.v[i] = ans.v[i] + self.v[i][j] * rhs.v[j];
            }
        }
        ans
    }
}
// matrix * matrix
impl <T: Float, const M: usize, const N: usize, const P: usize> Mul<BMatrix<T, N, P>> for BMatrix<T, M, N> {
    type Output = BMatrix<T, M, P>;

    fn mul(self, rhs: BMatrix<T, N, P>) -> Self::Output {
        let mut ans: BMatrix<T, M, P> = BMatrix::zero();
        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    ans.v[i][j] = ans.v[i][j] + self.v[i][k] * rhs.v[k][j];
                }
            }
        }
        ans
    }
}
// friends
// scalar - matrix
impl <const M: usize, const N: usize> Mul<BMatrix<f32, M, N>> for f32 {
    type Output = BMatrix<f32, M, N>;

    fn mul(self, rhs: BMatrix<f32, M, N>) -> Self::Output {
        rhs * self
    }
}
impl <const M: usize, const N: usize> Mul<BMatrix<f64, M, N>> for f64 {
    type Output = BMatrix<f64, M, N>;

    fn mul(self, rhs: BMatrix<f64, M, N>) -> Self::Output {
        rhs * self
    }
}
// vector(Transposed) - matrix
impl <T: Float, const M: usize, const N: usize> Mul<BMatrix<T, M, N>> for BVec<T, M> {
    type Output = BVec<T, N>;
    fn mul(self, rhs: BMatrix<T, M, N>) -> Self::Output {
        let mut ans: BVec<T, N> = BVec::zero();
        for i in 0..N {
            for j in 0..M {
                ans.v[i] = ans.v[i] + self.v[j] * rhs.v[j][i];
            }
        }
        ans
    }
}