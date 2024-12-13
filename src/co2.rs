use std::{num::TryFromIntError, ops};

/// 2D coordinate represented as a two-value tuple
///
/// Row-major.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord2<T> {
    /// Row
    pub row: T,
    /// Column
    pub col: T,
}
pub type Co2<T> = Coord2<T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offset2<T>
where
    // Offsets are always signed
    T: num::Signed,
{
    pub dy: T,
    pub dx: T,
}
pub type Ofs2<T> = Offset2<T>;

/* === Construct === */

#[macro_export]
macro_rules! co2 {
    ($row:expr, $col:expr) => {
        $crate::Coord2::from(($row, $col))
    };
    {y: $row:expr, x: $col:expr} => {
        $crate::Coord2::from(($row, $col))
    };
}

#[macro_export]
macro_rules! ofs2 {
    ($dy:expr, $dx:expr) => {
        $crate::Offset2::from(($dy, $dx))
    };
}

impl<T, U> From<(U, U)> for Coord2<T>
where
    U: Into<T>,
{
    fn from(value: (U, U)) -> Self {
        Self {
            row: value.0.into(),
            col: value.1.into(),
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
            dy: value.0.into(),
            dx: value.1.into(),
        }
    }
}

impl TryFrom<Coord2<usize>> for Coord2<isize> {
    type Error = TryFromIntError;

    fn try_from(value: Coord2<usize>) -> Result<Self, Self::Error> {
        Ok(Self {
            row: value.row.try_into()?,
            col: value.col.try_into()?,
        })
    }
}

impl TryFrom<Coord2<u64>> for Coord2<i64> {
    type Error = TryFromIntError;

    fn try_from(value: Coord2<u64>) -> Result<Self, Self::Error> {
        Ok(Self {
            row: value.row.try_into()?,
            col: value.col.try_into()?,
        })
    }
}

/* === Convert === */

impl<T> Coord2<T>
where
    T: Copy,
{
    pub fn try_into_tuple<U>(&self) -> Result<(U, U), TryFromIntError>
    where
        U: TryFrom<T>,
        TryFromIntError: From<<U as TryFrom<T>>::Error>,
    {
        Ok((self.row.try_into()?, self.col.try_into()?))
    }

    #[inline(always)]
    pub fn row(&self) -> T {
        self.row
    }

    #[inline(always)]
    pub fn col(&self) -> T {
        self.col
    }

    #[inline(always)]
    pub fn y(&self) -> T {
        self.row
    }

    #[inline(always)]
    pub fn x(&self) -> T {
        self.col
    }
}

impl<T> Offset2<T>
where
    T: Copy + num::Signed,
{
    #[inline(always)]
    pub fn row(&self) -> T {
        self.dy
    }

    #[inline(always)]
    pub fn col(&self) -> T {
        self.dx
    }
}

impl<T> From<Coord2<T>> for (T, T)
where
    T: Copy,
{
    fn from(value: Coord2<T>) -> Self {
        (value.row, value.col)
    }
}

impl<T> From<Offset2<T>> for (T, T)
where
    T: Copy + num::Signed,
{
    fn from(value: Offset2<T>) -> Self {
        (value.dy, value.dx)
    }
}

impl<T> From<Coord2<T>> for Offset2<T>
where
    T: num::Signed,
{
    fn from(value: Coord2<T>) -> Self {
        Offset2 {
            dy: value.row,
            dx: value.col,
        }
    }
}

impl<T> From<Offset2<T>> for Coord2<T>
where
    T: num::Signed,
{
    fn from(value: Offset2<T>) -> Self {
        Coord2 {
            row: value.dy,
            col: value.dx,
        }
    }
}

/* === Calculate === */

macro_rules! impl_add {
    ($lhs_t:ty | $rhs_t:ty = $out_t:ty, $out:ty) => {
        impl ops::Add<$rhs_t> for $lhs_t {
            type Output = $out_t;

            fn add(self, rhs: $rhs_t) -> Self::Output {
                <$out_t>::from((
                    (self.row() as $out + rhs.row() as $out),
                    (self.col() as $out + rhs.col() as $out),
                ))
            }
        }
    };
}

impl_add!(Co2<usize> | Co2<usize> = Ofs2<isize>, isize);
impl_add!(Co2<u64> | Co2<u64> = Ofs2<i64>, i64);
impl_add!(Co2<i64> | Co2<i64> = Ofs2<i64>, i64);
impl_add!(Co2<usize> | Ofs2<isize> = Ofs2<isize>, isize);
impl_add!(Co2<u64> | Ofs2<i64> = Ofs2<i64>, i64);

impl_add!(Ofs2<isize> | Ofs2<isize> = Ofs2<isize>, isize);
impl_add!(Ofs2<i64> | Ofs2<i64> = Ofs2<i64>, i64);

macro_rules! impl_sub {
    ($lhs_t:ty | $rhs_t:ty = $out_t:ty, $out:ty) => {
        impl ops::Sub<$rhs_t> for $lhs_t {
            type Output = $out_t;

            fn sub(self, rhs: $rhs_t) -> Self::Output {
                <$out_t>::from((
                    (self.row() as $out - rhs.row() as $out),
                    (self.col() as $out - rhs.col() as $out),
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
impl_sub!(Ofs2<i64> | Ofs2<i64> = Ofs2<i64>, i64);

macro_rules! impl_mul {
    ($lhs_t:ty | $rhs_t:ty = $out_t:ty, $out:ty) => {
        impl ops::Mul<$rhs_t> for $lhs_t {
            type Output = $out_t;

            fn mul(self, rhs: $rhs_t) -> Self::Output {
                <$out_t>::from((
                    (self.row() as $out * rhs as $out),
                    (self.col() as $out * rhs as $out),
                ))
            }
        }
    };
}

impl_mul!(Co2<u64> | u64 = Co2<u64>, u64);
impl_mul!(Co2<i64> | i64 = Co2<i64>, i64);
