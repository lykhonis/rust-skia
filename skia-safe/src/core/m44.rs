use crate::prelude::*;
use crate::{scalar, Matrix, Matrix44};
use bitflags::_core::ops::{AddAssign, MulAssign};
use skia_bindings as sb;
use skia_bindings::{Sk3LookAt, Sk3Perspective, SkM44, SkV2, SkV3, SkV4};
use std::f32;
use std::ops::{Add, Mul, Neg, Sub, SubAssign};
use std::slice;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct V2 {
    pub x: f32,
    pub y: f32,
}

impl NativeTransmutable<SkV2> for V2 {}

#[test]
fn test_v2_layout() {
    V2::test_layout();
}

impl V2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(self, b: Self) -> scalar {
        self.x * b.x + self.y * b.y
    }

    pub fn cross(self, b: Self) -> scalar {
        self.x * b.y - self.y * b.x
    }

    pub fn normalize(self) -> Self {
        self * (1.0 / self.length())
    }

    pub fn length_squared(self) -> scalar {
        Self::dot(self, self)
    }

    pub fn length(self) -> scalar {
        self.length_squared().sqrt()
    }

    const COMPONENTS: usize = 2;

    pub fn as_array(&self) -> &[f32; Self::COMPONENTS] {
        unsafe { slice::from_raw_parts(&self.x, Self::COMPONENTS) }
            .try_into()
            .unwrap()
    }

    pub fn as_mut_array(&mut self) -> &mut [f32; Self::COMPONENTS] {
        unsafe { slice::from_raw_parts_mut(&mut self.x, Self::COMPONENTS) }
            .try_into()
            .unwrap()
    }
}

impl Neg for V2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Add for V2 {
    type Output = Self;

    fn add(self, v: Self) -> Self::Output {
        Self::new(self.x + v.x, self.y + v.y)
    }
}

impl Sub for V2 {
    type Output = Self;

    fn sub(self, v: Self) -> Self::Output {
        Self::new(self.x - v.x, self.y - v.y)
    }
}

impl Mul for V2 {
    type Output = Self;

    fn mul(self, v: Self) -> Self::Output {
        Self::new(self.x * v.x, self.y * v.y)
    }
}

impl Mul<scalar> for V2 {
    type Output = Self;

    fn mul(self, s: scalar) -> Self::Output {
        Self::new(self.x * s, self.y * s)
    }
}

impl Mul<V2> for scalar {
    type Output = V2;

    fn mul(self, v: V2) -> Self::Output {
        V2::new(v.x * self, v.y * self)
    }
}

impl AddAssign for V2 {
    fn add_assign(&mut self, v: Self) {
        *self = *self + v
    }
}

impl SubAssign for V2 {
    fn sub_assign(&mut self, v: Self) {
        *self = *self - v
    }
}

impl MulAssign for V2 {
    fn mul_assign(&mut self, v: Self) {
        *self = *self * v
    }
}

impl MulAssign<scalar> for V2 {
    fn mul_assign(&mut self, s: scalar) {
        *self = *self * s
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl NativeTransmutable<SkV3> for V3 {}

#[test]
fn test_v3_layout() {
    V3::test_layout();
}

impl V3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, b: &Self) -> scalar {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn cross(&self, b: &Self) -> Self {
        Self::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }

    pub fn normalize(&self) -> Self {
        *self * (1.0 / self.length())
    }

    pub fn length_squared(&self) -> scalar {
        Self::dot(self, self)
    }

    pub fn length(&self) -> scalar {
        Self::dot(self, self).sqrt()
    }

    const COMPONENTS: usize = 3;

    pub fn as_array(&self) -> &[f32; Self::COMPONENTS] {
        unsafe { slice::from_raw_parts(&self.x, Self::COMPONENTS) }
            .try_into()
            .unwrap()
    }

    pub fn as_mut_array(&mut self) -> &mut [f32; Self::COMPONENTS] {
        unsafe { slice::from_raw_parts_mut(&mut self.x, Self::COMPONENTS) }
            .try_into()
            .unwrap()
    }
}

impl Neg for V3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add for V3 {
    type Output = Self;

    fn add(self, v: Self) -> Self::Output {
        Self::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Sub for V3 {
    type Output = Self;

    fn sub(self, v: Self) -> Self::Output {
        Self::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Mul for V3 {
    type Output = Self;

    fn mul(self, v: Self) -> Self::Output {
        Self::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Mul<scalar> for V3 {
    type Output = Self;

    fn mul(self, s: scalar) -> Self::Output {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}

impl Mul<V3> for scalar {
    type Output = V3;

    fn mul(self, v: V3) -> Self::Output {
        V3::new(v.x * self, v.y * self, v.z * self)
    }
}

impl AddAssign for V3 {
    fn add_assign(&mut self, v: Self) {
        *self = *self + v
    }
}

impl SubAssign for V3 {
    fn sub_assign(&mut self, v: Self) {
        *self = *self - v
    }
}

impl MulAssign for V3 {
    fn mul_assign(&mut self, v: Self) {
        *self = *self * v
    }
}

impl MulAssign<scalar> for V3 {
    fn mul_assign(&mut self, s: scalar) {
        *self = *self * s
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct V4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl NativeTransmutable<SkV4> for V4 {}

#[test]
fn test_v4_layout() {
    V4::test_layout();
}

impl V4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    const COMPONENTS: usize = 4;

    pub fn as_array(&self) -> &[f32; Self::COMPONENTS] {
        unsafe { slice::from_raw_parts(&self.x, Self::COMPONENTS) }
            .try_into()
            .unwrap()
    }

    pub fn as_mut_array(&mut self) -> &mut [f32; Self::COMPONENTS] {
        unsafe { slice::from_raw_parts_mut(&mut self.x, Self::COMPONENTS) }
            .try_into()
            .unwrap()
    }
}

impl Neg for V4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Add for V4 {
    type Output = Self;

    fn add(self, v: Self) -> Self::Output {
        Self::new(self.x + v.x, self.y + v.y, self.z + v.z, self.w + v.w)
    }
}

impl Sub for V4 {
    type Output = Self;

    fn sub(self, v: Self) -> Self::Output {
        Self::new(self.x - v.x, self.y - v.y, self.z - v.z, self.w - v.w)
    }
}

impl Mul for V4 {
    type Output = Self;

    fn mul(self, v: Self) -> Self::Output {
        Self::new(self.x * v.x, self.y * v.y, self.z * v.z, self.w * v.w)
    }
}

impl Mul<scalar> for V4 {
    type Output = Self;

    fn mul(self, s: scalar) -> Self::Output {
        Self::new(self.x * s, self.y * s, self.z * s, self.w * s)
    }
}

impl Mul<V4> for scalar {
    type Output = V4;

    fn mul(self, v: V4) -> Self::Output {
        V4::new(v.x * self, v.y * self, v.z * self, v.w * self)
    }
}

impl AddAssign for V4 {
    fn add_assign(&mut self, v: Self) {
        *self = *self + v
    }
}

impl SubAssign for V4 {
    fn sub_assign(&mut self, v: Self) {
        *self = *self - v
    }
}

impl MulAssign for V4 {
    fn mul_assign(&mut self, v: Self) {
        *self = *self * v
    }
}

impl MulAssign<scalar> for V4 {
    fn mul_assign(&mut self, s: scalar) {
        *self = *self * s
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct M44 {
    mat: [f32; Self::COMPONENTS],
}

impl NativeTransmutable<SkM44> for M44 {}

#[test]
fn test_m44_layout() {
    M44::test_layout()
}

impl Default for M44 {
    fn default() -> Self {
        Self {
            mat: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

impl PartialEq for M44 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sb::C_M44_equals(self.native(), other.native()) }
    }
}

impl M44 {
    const COMPONENTS: usize = 16;

    pub fn new_identity() -> Self {
        Self::default()
    }

    pub fn concat(a: &Self, b: &Self) -> Self {
        let mut m = Self::default();
        m.set_concat(a, b);
        m
    }

    pub fn nan() -> Self {
        Self {
            mat: [f32::NAN; Self::COMPONENTS],
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        m0: scalar,
        m1: scalar,
        m2: scalar,
        m3: scalar,
        m4: scalar,
        m5: scalar,
        m6: scalar,
        m7: scalar,
        m8: scalar,
        m9: scalar,
        m10: scalar,
        m11: scalar,
        m12: scalar,
        m13: scalar,
        m14: scalar,
        m15: scalar,
    ) -> Self {
        Self {
            mat: [
                m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15,
            ],
        }
    }

    pub fn rows(r0: &V4, r1: &V4, r2: &V4, r3: &V4) -> Self {
        let mut m = Self::default();
        m.set_row(0, r0);
        m.set_row(1, r1);
        m.set_row(2, r2);
        m.set_row(3, r3);
        m
    }

    pub fn cols(c0: &V4, c1: &V4, c2: &V4, c3: &V4) -> Self {
        let mut m = Self::default();
        m.set_col(0, c0);
        m.set_col(1, c1);
        m.set_col(2, c2);
        m.set_col(3, c3);
        m
    }

    pub fn translate(x: scalar, y: scalar, z: scalar) -> Self {
        Self::new(
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn scale(x: scalar, y: scalar, z: scalar) -> Self {
        Self::new(
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotate(axis: V3, radians: scalar) -> Self {
        let mut m = Self::default();
        m.set_rotate(axis, radians);
        m
    }

    pub fn get_col_major(&self, v: &mut [scalar; Self::COMPONENTS]) {
        v.copy_from_slice(&self.mat)
    }

    pub fn get_row_major(&self, v: &mut [scalar; Self::COMPONENTS]) {
        unsafe { self.native().getRowMajor(v.as_mut_ptr()) }
    }

    pub fn set_col_major(&mut self, v: &[scalar; Self::COMPONENTS]) -> &mut Self {
        self.mat.copy_from_slice(v);
        self
    }

    pub fn set_row_major(&mut self, v: &[scalar; Self::COMPONENTS]) -> &mut Self {
        unsafe { self.native_mut().setRowMajor(v.as_ptr()) };
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_44(
        &mut self,
        m0: scalar,
        m1: scalar,
        m2: scalar,
        m3: scalar,
        m4: scalar,
        m5: scalar,
        m6: scalar,
        m7: scalar,
        m8: scalar,
        m9: scalar,
        m10: scalar,
        m11: scalar,
        m12: scalar,
        m13: scalar,
        m14: scalar,
        m15: scalar,
    ) -> &mut Self {
        self.mat[0] = m0;
        self.mat[1] = m1;
        self.mat[2] = m2;
        self.mat[3] = m3;
        self.mat[4] = m4;
        self.mat[5] = m5;
        self.mat[6] = m6;
        self.mat[7] = m7;
        self.mat[8] = m8;
        self.mat[9] = m9;
        self.mat[10] = m10;
        self.mat[11] = m11;
        self.mat[12] = m12;
        self.mat[13] = m13;
        self.mat[14] = m14;
        self.mat[15] = m15;
        self
    }

    pub fn rc(&self, r: usize, c: usize) -> scalar {
        assert!(r <= 3);
        assert!(c <= 3);
        self.mat[c * 4 + r]
    }

    pub fn set_rc(&mut self, r: usize, c: usize, value: scalar) {
        assert!(r <= 3);
        assert!(c <= 3);
        self.mat[c * 4 + r] = value;
    }

    pub fn row(&self, i: usize) -> V4 {
        assert!(i <= 3);
        V4::new(
            self.mat[i],
            self.mat[i + 4],
            self.mat[i + 8],
            self.mat[i + 12],
        )
    }

    pub fn col(&self, i: usize) -> V4 {
        assert!(i <= 3);
        V4::new(
            self.mat[i * 4],
            self.mat[i * 4 + 1],
            self.mat[i * 4 + 2],
            self.mat[i * 4 + 3],
        )
    }

    pub fn set_row(&mut self, i: usize, v: &V4) {
        assert!(i <= 3);
        self.mat[i] = v.x;
        self.mat[i + 4] = v.y;
        self.mat[i + 8] = v.z;
        self.mat[i + 12] = v.w;
    }

    pub fn set_col(&mut self, i: usize, v: &V4) {
        assert!(i <= 3);
        self.mat[(i * 4)..(i * 4 + V4::COMPONENTS)].copy_from_slice(v.as_array());
    }

    pub fn set_identity(&mut self) -> &mut Self {
        self.set_44(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn set_translate(&mut self, x: scalar, y: scalar, z: scalar) -> &mut Self {
        self.set_44(
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn set_scale(&mut self, x: scalar, y: scalar, z: scalar) -> &mut Self {
        self.set_44(
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn set_rotate_unit_sin_cos(
        &mut self,
        axis: V3,
        sin_angle: scalar,
        cos_angle: scalar,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .setRotateUnitSinCos(axis.into_native(), sin_angle, cos_angle)
        };
        self
    }

    pub fn set_rotate_unit(&mut self, axis: V3, radians: scalar) -> &mut Self {
        self.set_rotate_unit_sin_cos(axis, radians.sin(), radians.cos())
    }

    pub fn set_rotate(&mut self, axis: V3, radians: scalar) -> &mut Self {
        unsafe { self.native_mut().setRotate(axis.into_native(), radians) };
        self
    }

    pub fn set_concat_16(&mut self, a: &M44, col_major: &[scalar; Self::COMPONENTS]) -> &mut Self {
        unsafe {
            self.native_mut()
                .setConcat16(a.native(), col_major.as_ptr())
        };
        self
    }

    pub fn set_concat(&mut self, a: &M44, b: &M44) -> &mut Self {
        self.set_concat_16(a, &b.mat)
    }

    pub fn pre_concat_16(&mut self, col_major: &[scalar; Self::COMPONENTS]) -> &mut Self {
        self.set_concat_16(&self.clone(), col_major)
    }

    pub fn invert(&self) -> Option<M44> {
        let mut m = Self::default();
        unsafe { self.native().invert(m.native_mut()) }.if_true_some(m)
    }

    pub fn transpose(&self) -> M44 {
        Self::from_native(unsafe { self.native().transpose() })
    }

    pub fn dump(&self) {
        unsafe { self.native().dump() }
    }

    pub fn map(&self, x: f32, y: f32, z: f32, w: f32) -> V4 {
        V4::from_native(unsafe { self.native().map(x, y, z, w) })
    }

    pub fn to_m33(&self) -> Matrix {
        let m = &self.mat;
        Matrix::new_all(m[0], m[4], m[12], m[1], m[5], m[13], m[3], m[7], m[15])
    }

    pub fn pre_translate(&mut self, x: scalar, y: scalar) -> &mut Self {
        unsafe { self.native_mut().preTranslate(x, y) };
        self
    }

    pub fn pre_scale(&mut self, x: scalar, y: scalar) -> &mut Self {
        unsafe { self.native_mut().preScale(x, y) };
        self
    }

    pub fn pre_concat(&mut self, m: &Matrix) -> &mut Self {
        unsafe { self.native_mut().preConcat(m.native()) };
        self
    }

    pub fn look_at(eye: &V3, center: &V3, up: &V3) -> Self {
        Self::from_native(unsafe { Sk3LookAt(eye.native(), center.native(), up.native()) })
    }

    pub fn perspective(near: f32, far: f32, angle: f32) -> Self {
        Self::from_native(unsafe { Sk3Perspective(near, far, angle) })
    }

    // helper

    pub fn to_matrix44(&self) -> Matrix44 {
        let mut m = Matrix44::default();
        m.set_col_major(&self.mat);
        m
    }
}

impl Mul for &M44 {
    type Output = M44;

    fn mul(self, m: Self) -> Self::Output {
        M44::concat(self, &m)
    }
}

impl Mul<V4> for &M44 {
    type Output = V4;

    fn mul(self, v: V4) -> Self::Output {
        self.map(v.x, v.y, v.z, v.w)
    }
}

impl Mul<V3> for &M44 {
    type Output = V3;

    fn mul(self, v: V3) -> Self::Output {
        let v4 = self.map(v.x, v.y, v.z, 0.0);
        V3::new(v4.x, v4.y, v4.z)
    }
}

impl From<&Matrix> for M44 {
    fn from(src: &Matrix) -> Self {
        use crate::matrix::Member::*;

        Self::new(
            src[ScaleX],
            src[SkewX],
            0.0,
            src[TransX],
            src[SkewY],
            src[ScaleY],
            0.0,
            src[TransY],
            0.0,
            0.0,
            1.0,
            0.0,
            src[Persp0],
            src[Persp1],
            0.0,
            src[Persp2],
        )
    }
}

impl From<Matrix> for M44 {
    fn from(m: Matrix) -> Self {
        M44::from(&m)
    }
}

impl From<&Matrix44> for M44 {
    fn from(m: &Matrix44) -> Self {
        let mut rm: [f32; 16] = Default::default();
        m.as_col_major(&mut rm);
        let mut m = M44::default();
        m.set_col_major(&rm);
        m
    }
}
