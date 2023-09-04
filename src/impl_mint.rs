use mint;
use crate::{Real, Vec2, Vec3, Vec4, Quat};

impl From<mint::Vector2<Real>> for Vec2 {
    fn from(v: mint::Vector2<Real>) -> Self {
        Self{ x: v.x, y: v.y }
    }
}
impl From<Vec2> for mint::Vector2<Real> {
    fn from(v: Vec2) -> Self {
        Self{ x: v.x, y: v.y }
    }
}
impl mint::IntoMint for Vec2 {
    type MintType = mint::Vector2<Real>;
}


impl From<mint::Vector3<Real>> for Vec3 {
    fn from(v: mint::Vector3<Real>) -> Self {
        Self{ x: v.x, y: v.y, z: v.z }
    }
}
impl From<Vec3> for mint::Vector3<Real> {
    fn from(v: Vec3) -> Self {
        Self{ x: v.x, y: v.y, z: v.z }
    }
}
impl mint::IntoMint for Vec3 {
    type MintType = mint::Vector3<Real>;
}

impl From<mint::Vector4<Real>> for Vec4 {
    fn from(v: mint::Vector4<Real>) -> Self {
        Self{ x: v.x, y: v.y, z: v.z, w: v.w }
    }
}
impl From<Vec4> for mint::Vector4<Real> {
    fn from(v: Vec4) -> Self {
        Self{ x: v.x, y: v.y, z: v.z, w: v.w }
    }
}
impl mint::IntoMint for Vec4 {
    type MintType = mint::Vector4<Real>;
}

impl From<mint::Quaternion<Real>> for Quat {
    fn from(v: mint::Quaternion<Real>) -> Self {
        Self{ x: v.v.x, y: v.v.y, z: v.v.z, w: v.s }
    }
}
impl From<Quat> for mint::Quaternion<Real> {
    fn from(v: Quat) -> Self {
        Self{ v: mint::Vector3::<Real>{ x: v.x, y: v.y, z: v.z }, s: v.w }
    }
}
impl mint::IntoMint for Quat {
    type MintType = mint::Quaternion<Real>;
}
