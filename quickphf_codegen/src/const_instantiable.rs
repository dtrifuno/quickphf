use core::fmt;

/// Provides a way to generate code which instantiates values of the
/// implementing type in a `const` context.
pub trait ConstInstantiable {
    /// Print a `const` expression that can be used to instantiate this value.
    #[allow(clippy::missing_errors_doc)]
    fn fmt_const_new(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

/// Provides blanket implementation of [`ConstInstantiable`] which defers to
/// [`Debug`](core::fmt::Debug) representation of type.
pub trait DebugInstantiable: std::fmt::Debug {}

impl<T: DebugInstantiable> ConstInstantiable for T {
    fn fmt_const_new(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DebugInstantiable for () {}
impl DebugInstantiable for bool {}
impl DebugInstantiable for &str {}
impl DebugInstantiable for char {}
impl DebugInstantiable for i8 {}
impl DebugInstantiable for i16 {}
impl DebugInstantiable for i32 {}
impl DebugInstantiable for i64 {}
impl DebugInstantiable for i128 {}
impl DebugInstantiable for isize {}
impl DebugInstantiable for u8 {}
impl DebugInstantiable for u16 {}
impl DebugInstantiable for u32 {}
impl DebugInstantiable for u64 {}
impl DebugInstantiable for u128 {}
impl DebugInstantiable for usize {}

impl<T1, T2> ConstInstantiable for (T1, T2)
where
    T1: ConstInstantiable,
    T2: ConstInstantiable,
{
    fn fmt_const_new(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        self.0.fmt_const_new(f)?;
        write!(f, ", ")?;
        self.1.fmt_const_new(f)?;
        write!(f, ")")
    }
}

impl<T1, T2, T3> ConstInstantiable for (T1, T2, T3)
where
    T1: ConstInstantiable,
    T2: ConstInstantiable,
    T3: ConstInstantiable,
{
    fn fmt_const_new(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        self.0.fmt_const_new(f)?;
        write!(f, ", ")?;
        self.1.fmt_const_new(f)?;
        write!(f, ", ")?;
        self.2.fmt_const_new(f)?;
        write!(f, ")")
    }
}

impl<T1, T2, T3, T4> ConstInstantiable for (T1, T2, T3, T4)
where
    T1: ConstInstantiable,
    T2: ConstInstantiable,
    T3: ConstInstantiable,
    T4: ConstInstantiable,
{
    fn fmt_const_new(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        self.0.fmt_const_new(f)?;
        write!(f, ", ")?;
        self.1.fmt_const_new(f)?;
        write!(f, ", ")?;
        self.2.fmt_const_new(f)?;
        write!(f, ", ")?;
        self.3.fmt_const_new(f)?;
        write!(f, ")")
    }
}

impl<T: ConstInstantiable> ConstInstantiable for Option<T> {
    fn fmt_const_new(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Some(x) => {
                write!(f, "Some(")?;
                x.fmt_const_new(f)?;
                write!(f, ")")
            }
            None => write!(f, "None"),
        }
    }
}

impl<T1, T2> ConstInstantiable for Result<T1, T2>
where
    T1: ConstInstantiable,
    T2: ConstInstantiable,
{
    fn fmt_const_new(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ok(x) => {
                write!(f, "Ok(")?;
                x.fmt_const_new(f)?;
                write!(f, ")")
            }
            Err(e) => {
                write!(f, "Err(")?;
                e.fmt_const_new(f)?;
                write!(f, ")")
            }
        }
    }
}
