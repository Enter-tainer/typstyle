use pretty::{DocAllocator, DocBuilder};

pub trait DocExt<'a, D, A>
where
    A: 'a,
    D: ?Sized + DocAllocator<'a, A>,
{
    fn repeat_n(self, n: usize) -> Self;
}

impl<'a, D, A> DocExt<'a, D, A> for DocBuilder<'a, D, A>
where
    A: 'a,
    D: ?Sized + DocAllocator<'a, A>,
    DocBuilder<'a, D, A>: Clone,
{
    fn repeat_n(self, n: usize) -> Self {
        let mut doc = self.0.nil();
        for _ in 0..n {
            doc = doc.append(self.clone());
        }
        doc
    }
}
