use std::{num::TryFromIntError, ops};

/// 2D coordinate represented as a two-value tuple
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord2<T> {
    pub x: T,
    pub y: T,
}
pub type Co2<T> = Coord2<T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offset2<T>
where
    // Offsets are always signed
    T: num::Signed,
{
    pub dx: T,
    pub dy: T,
}
pub type Ofs2<T> = Offset2<T>;

/* === Construct === */

#[macro_export]
macro_rules! co2 {
    ($x:expr, $y:expr) => {
        Co2::from(($x, $y))
    };
}

#[macro_export]
macro_rules! ofs2 {
    ($dx:expr, $dy:expr) => {
        Ofs2::from(($dx, $dy))
    };
}

impl<T, U> From<(U, U)> for Coord2<T>
where
    U: Into<T>,
{
    fn from(value: (U, U)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

impl<T, U> From<(U, U)> for Ofs2<T>
where
    U: Into<T>,
    T: num::Signed,
{
    fn from(value: (U, U)) -> Self {
        Self {
            dx: value.0.into(),
            dy: value.1.into(),
        }
    }
}

impl TryFrom<Coord2<usize>> for Coord2<isize> {
    type Error = TryFromIntError;

    fn try_from(value: Coord2<usize>) -> Result<Self, Self::Error> {
        Ok(Self {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
        })
    }
}

impl TryFrom<Coord2<u64>> for Coord2<i64> {
    type Error = TryFromIntError;

    fn try_from(value: Coord2<u64>) -> Result<Self, Self::Error> {
        Ok(Self {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
        })
    }
}

/* === Convert === */

impl<T> Coord2<T>
where
    T: Copy,
{
    pub fn as_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn try_as_tuple<U>(&self) -> Result<(U, U), TryFromIntError>
    where
        U: TryFrom<T>,
        TryFromIntError: From<<U as TryFrom<T>>::Error>,
    {
        Ok((self.x.try_into()?, self.y.try_into()?))
    }

    #[inline(always)]
    pub fn x(&self) -> T {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> T {
        self.y
    }
}

impl<T> Coord2<T>
where
    T: Copy + num::Signed,
{
    pub fn as_offset(&self) -> Offset2<T> {
        Offset2 {
            dx: self.x,
            dy: self.y,
        }
    }
}

impl<T> Offset2<T>
where
    T: Copy + num::Signed,
{
    pub fn as_tuple(&self) -> (T, T) {
        (self.dx, self.dy)
    }

    pub fn as_coord(&self) -> Coord2<T> {
        Coord2 {
            x: self.dx,
            y: self.dy,
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
}

impl<T> From<Coord2<T>> for (T, T)
where
    T: Copy,
{
    fn from(value: Coord2<T>) -> Self {
        value.as_tuple()
    }
}

impl<T> From<Offset2<T>> for (T, T)
where
    T: Copy + num::Signed,
{
    fn from(value: Offset2<T>) -> Self {
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
                ))
            }
        }
    };
}

impl_add!(Co2<usize> | Co2<usize> = Ofs2<isize>, isize);
impl_add!(Co2<u64> | Co2<u64> = Ofs2<i64>, i64);
impl_add!(Co2<usize> | Ofs2<isize> = Ofs2<isize>, isize);
impl_add!(Co2<u64> | Ofs2<i64> = Ofs2<i64>, i64);
impl_add!(Ofs2<isize> | Ofs2<isize> = Ofs2<isize>, isize);

macro_rules! impl_sub {
    ($lhs_t:ty | $rhs_t:ty = $out_t:ty, $out:ty) => {
        impl ops::Sub<$rhs_t> for $lhs_t {
            type Output = $out_t;

            fn sub(self, rhs: $rhs_t) -> Self::Output {
                <$out_t>::from((
                    (self.x() as $out - rhs.x() as $out),
                    (self.y() as $out - rhs.y() as $out),
                ))
            }
        }
    };
}

impl_sub!(Co2<usize> | Co2<usize> = Ofs2<isize>, isize);
impl_sub!(Co2<u64> | Co2<u64> = Ofs2<i64>, i64);
impl_sub!(Co2<usize> | Ofs2<isize> = Ofs2<isize>, isize);
impl_sub!(Co2<u64> | Ofs2<i64> = Ofs2<i64>, i64);
impl_sub!(Ofs2<isize> | Ofs2<isize> = Ofs2<isize>, isize);
