# passing-arg-iter
Add an argument to argument to `Iterator::Item` without losing `Sized`

If you find yourself mapping a capturing closure over an `Iterator` that you need to return, you'll want to use `passing_arg(self, arg: T) -> PassingArgument<I, T>`
