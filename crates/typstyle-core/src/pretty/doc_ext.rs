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

pub trait AllocExt<'a, A> {
    fn spaces(&'a self, n: usize) -> DocBuilder<'a, Self, A>
    where
        Self: DocAllocator<'a, A>;
}

impl<'a, T, A> AllocExt<'a, A> for T
where
    T: DocAllocator<'a, A>,
    A: 'a,
{
    fn spaces(&'a self, n: usize) -> DocBuilder<'a, Self, A> {
        static SPACES: &str =
            "                                                                                ";

        if n == 0 {
            self.nil()
        } else if n <= SPACES.len() {
            self.text(&SPACES[..n])
        } else {
            let mut doc = self.nil();
            let mut remaining = n;
            while remaining != 0 {
                let i = SPACES.len().min(remaining);
                remaining -= i;
                doc = doc.append(self.text(&SPACES[..i]))
            }
            doc
        }
    }
}
