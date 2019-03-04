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
}
