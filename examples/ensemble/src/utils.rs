pub trait Ignore {
    fn ignore(self);
}

impl<E> Ignore for std::result::Result<(), E> {
    fn ignore(self) {}
}
