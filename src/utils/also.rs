//! `Also` trait

/// Extends all sized types with an `also` method
pub trait Also: Sized {
    /// mutate self by `f` and return self
    #[inline]
    fn also(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }
}

impl<T: Sized> Also for T {}
