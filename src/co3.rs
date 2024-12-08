use std::{num::TryFromIntError, ops};

/// 3D coordinate represented as a three-value tuple
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
pub type Co3<T> = Coord3<T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offset3<T>
where
    // Offsets are always signed
    T: num::Signed,
{
    pub dx: T,
    pub dy: T,
    pub dz: T,
}
pub type Ofs3<T> = Offset3<T>;

/* === Construct === */

#[macro_export]
macro_rules! co3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::Coord3::from(($x, $y, $z))
    };
}

#[macro_export]
macro_rules! ofs3 {
    ($dx:expr, $dy:expr, $dz:expr) => {
        $crate::Offset3::from(($dx, $dy, $dz))
    };
}

impl<T, U> From<(U, U, U)> for Coord3<T>
where
    U: Into<T>,
{
    fn from(value: (U, U, U)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
        }
    }
}

impl<T, U> From<(U, U, U)> for Ofs3<T>
where
    U: Into<T>,
    T: num::Signed,
{
    fn from(value: (U, U, U)) -> Self {
        Self {
            dx: value.0.into(),
            dy: value.1.into(),
            dz: value.2.into(),
        }
    }
}

impl TryFrom<Coord3<usize>> for Coord3<isize> {
    type Error = TryFromIntError;

    fn try_from(value: Coord3<usize>) -> Result<Self, Self::Error> {
        Ok(Self {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
            z: value.z.try_into()?,
        })
    }
}

impl TryFrom<Coord3<u64>> for Coord3<i64> {
    type Error = TryFromIntError;

    fn try_from(value: Coord3<u64>) -> Result<Self, Self::Error> {
        Ok(Self {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
            z: value.z.try_into()?,
        })
    }
}

/* === Convert === */

impl<T> Coord3<T>
where
    T: Copy,
{
    pub fn as_tuple(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    pub fn try_as_tuple<U>(&self) -> Result<(U, U, U), TryFromIntError>
    where
        U: TryFrom<T>,
        TryFromIntError: From<<U as TryFrom<T>>::Error>,
    {
        Ok((self.x.try_into()?, self.y.try_into()?, self.z.try_into()?))
    }

    #[inline(always)]
    pub fn x(&self) -> T {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> T {
        self.y
    }

    #[inline(always)]
    pub fn z(&self) -> T {
        self.z
    }
}

impl<T> Coord3<T>
where
    T: Copy + num::Signed,
{
    pub fn as_offset(&self) -> Offset3<T> {
        Offset3 {
            dx: self.x,
            dy: self.y,
            dz: self.z,
        }
    }
}

impl<T> Offset3<T>
where
    T: Copy + num::Signed,
{
    pub fn as_tuple(&self) -> (T, T, T) {
        (self.dx, self.dy, self.dx)
    }

    pub fn as_coord(&self) -> Coord3<T> {
        Coord3 {
            x: self.dx,
            y: self.dy,
            z: self.dz,
        }
    }

    #[inline(always)]
    pub fn x(&self) -> T {
        self.dx
    }

    #[inline(always)]
    pub fn y(&self) -> T {
        self.dy
    }

    #[inline(always)]
    pub fn z(&self) -> T {
        self.dz
    }
}

impl<T> From<Coord3<T>> for (T, T, T)
where
    T: Copy,
{
    fn from(value: Coord3<T>) -> Self {
        value.as_tuple()
    }
}

impl<T> From<Offset3<T>> for (T, T, T)
where
    T: Copy + num::Signed,
{
    fn from(value: Offset3<T>) -> Self {
        value.as_tuple()
    }
}

/* === Calculate === */

macro_rules! impl_add {
    ($lhs_t:ty | $rhs_t:ty = $out_t:ty, $out:ty) => {
        impl ops::Add<$rhs_t> for $lhs_t {
            type Output = $out_t;

            fn add(self, rhs: $rhs_t) -> Self::Output {
                <$out_t>::from((
                    (self.x() as $out + rhs.x() as $out),
                    (self.y() as $out + rhs.y() as $out),
                    (self.z() as $out + rhs.z() as $out),
                ))
            }
        }
    };
}

impl_add!(Co3<usize> | Co3<usize> = Ofs3<isize>, isize);
impl_add!(Co3<u64> | Co3<u64> = Ofs3<i64>, i64);
impl_add!(Co3<usize> | Ofs3<isize> = Ofs3<isize>, isize);
impl_add!(Co3<u64> | Ofs3<i64> = Ofs3<i64>, i64);
impl_add!(Ofs3<isize> | Ofs3<isize> = Ofs3<isize>, isize);

macro_rules! impl_sub {
    ($lhs_t:ty | $rhs_t:ty = $out_t:ty, $out:ty) => {
        impl ops::Sub<$rhs_t> for $lhs_t {
            type Output = $out_t;

            fn sub(self, rhs: $rhs_t) -> Self::Output {
                <$out_t>::from((
                    (self.x() as $out - rhs.x() as $out),
                    (self.y() as $out - rhs.y() as $out),
                    (self.z() as $out - rhs.z() as $out),
                ))
            }
        }
    };
}

impl_sub!(Co3<usize> | Co3<usize> = Ofs3<isize>, isize);
impl_sub!(Co3<u64> | Co3<u64> = Ofs3<i64>, i64);
impl_sub!(Co3<usize> | Ofs3<isize> = Ofs3<isize>, isize);
impl_sub!(Co3<u64> | Ofs3<i64> = Ofs3<i64>, i64);
impl_sub!(Ofs3<isize> | Ofs3<isize> = Ofs3<isize>, isize);
