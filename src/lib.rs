#[derive(Clone)]
pub struct PassingArgument<I, T> {
    iter: I,
    argument: T,
}

pub trait PassingArg<I, T> {
    /// Add an argument to a `Iterator::Item`
    /// Default implementation requires `T: Clone`
    fn passing_arg(self, arg: T) -> PassingArgument<I, T>;
}

impl<I, T> PassingArg<I, T> for I
where
    I: Iterator,
{
    fn passing_arg(self, arg: T) -> PassingArgument<I, T> {
        PassingArgument {
            iter: self,
            argument: arg,
        }
    }
}

impl<I, T> Iterator for PassingArgument<I, T>
where
    I: Iterator,
    T: Clone,
{
    type Item = (T, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => Some((self.argument.clone(), item)),
            None => None,
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|nth| (self.argument.clone(), nth))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, T> DoubleEndedIterator for PassingArgument<I, T>
where
    I: DoubleEndedIterator,
    T: Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|nb| (self.argument.clone(), nb))
    }
}

impl<I, T> ExactSizeIterator for PassingArgument<I, T>
where
    I: ExactSizeIterator,
    T: Clone,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}
