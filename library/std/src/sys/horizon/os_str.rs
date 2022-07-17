/// The underlying OsString/OsStr implementation on Horizon is just a
/// wrapper around UTF-8 string types (String/str)
use crate::borrow::Cow;
use crate::collections::TryReserveError;
use crate::fmt;
use crate::mem;
use crate::rc::Rc;
use crate::sync::Arc;
use crate::string::String;
use crate::sys_common::{AsInner, FromInner, IntoInner};

#[derive(Clone, Hash)]
pub struct Buf {
    pub inner: String,
}

impl IntoInner<String> for Buf {
    fn into_inner(self) -> String {
        self.inner
    }
}

impl FromInner<String> for Buf {
    fn from_inner(inner: String) -> Self {
        Buf { inner }
    }
}

impl AsInner<str> for Buf {
    fn as_inner(&self) -> &str {
        &self.inner
    }
}

impl fmt::Debug for Buf {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), formatter)
    }
}

impl fmt::Display for Buf {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_slice(), formatter)
    }
}

#[repr(transparent)]
pub struct Slice {
    pub inner: str,
}

impl fmt::Debug for Slice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, formatter)
    }
}

impl fmt::Display for Slice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}

impl Buf {
    pub fn with_capacity(capacity: usize) -> Buf {
        Buf { inner: String::with_capacity(capacity) }
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn from_string(s: String) -> Buf {
        Buf { inner: s }
    }

    #[inline]
    pub fn as_slice(&self) -> &Slice {
        // SAFETY: Slice just wraps str,
        // and &*self.inner is &str, therefore
        // transmuting &str to &Slice is safe.
        unsafe { mem::transmute(&*self.inner) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut Slice {
        // SAFETY: Slice just wraps str,
        // and &mut *self.inner is &mut str, therefore
        // transmuting &mut str to &mut Slice is safe.
        unsafe { mem::transmute(&mut *self.inner) }
    }

    pub fn into_string(self) -> Result<String, Buf> {
        Ok(self.inner)
    }

    pub fn push_slice(&mut self, s: &Slice) {
        self.inner.push_str(&s.inner)
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional)
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity)
    }

    #[inline]
    pub fn into_box(self) -> Box<Slice> {
        unsafe { mem::transmute(self.inner.into_boxed_str()) }
    }

    #[inline]
    pub fn from_box(boxed: Box<Slice>) -> Buf {
        let inner: Box<str> = unsafe { mem::transmute(boxed) };
        Buf { inner: String::from(inner) }
    }

    #[inline]
    pub fn into_arc(&self) -> Arc<Slice> {
        self.as_slice().into_arc()
    }

    #[inline]
    pub fn into_rc(&self) -> Rc<Slice> {
        self.as_slice().into_rc()
    }
}

impl Slice {
    #[inline]
    pub fn from_str(s: &str) -> &Slice {
        unsafe { mem::transmute(s) }
    }

    pub fn to_str(&self) -> Option<&str> {
        Some(&self.inner)
    }

    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.inner)
    }

    pub fn to_owned(&self) -> Buf {
        let mut buf = String::with_capacity(self.inner.len());
        buf.push_str(&self.inner);
        Buf { inner: buf }
    }

    pub fn clone_into(&self, buf: &mut Buf) {
        self.inner.clone_into(&mut buf.inner)
    }

    #[inline]
    pub fn into_box(&self) -> Box<Slice> {
      let boxed: Box<str> = self.inner.into();
      unsafe { mem::transmute(boxed) }
    }

    pub fn empty_box() -> Box<Slice> {
      let boxed: Box<str> = Default::default();
      unsafe { mem::transmute(boxed) }
    }

    #[inline]
    pub fn into_arc(&self) -> Arc<Slice> {
        let arc: Arc<str> = Arc::from(&self.inner);
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Slice) }
    }

    #[inline]
    pub fn into_rc(&self) -> Rc<Slice> {
        let rc: Rc<str> = Rc::from(&self.inner);
        unsafe { Rc::from_raw(Rc::into_raw(rc) as *const Slice) }
    }

    #[inline]
    pub fn make_ascii_lowercase(&mut self) {
        self.inner.make_ascii_lowercase()
    }

    #[inline]
    pub fn make_ascii_uppercase(&mut self) {
        self.inner.make_ascii_uppercase()
    }

    #[inline]
    pub fn to_ascii_lowercase(&self) -> Buf {
        Buf { inner: self.inner.to_ascii_lowercase() }
    }

    #[inline]
    pub fn to_ascii_uppercase(&self) -> Buf {
        Buf { inner: self.inner.to_ascii_uppercase() }
    }

    #[inline]
    pub fn is_ascii(&self) -> bool {
        self.inner.is_ascii()
    }

    #[inline]
    pub fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.inner.eq_ignore_ascii_case(&other.inner)
    }
}
