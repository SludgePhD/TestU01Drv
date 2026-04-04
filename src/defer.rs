pub(crate) fn defer(f: impl FnOnce()) -> impl Drop {
    struct Defer<F: FnOnce()>(Option<F>);

    impl<F: FnOnce()> Drop for Defer<F> {
        fn drop(&mut self) {
            (self.0.take().unwrap())();
        }
    }

    Defer(Some(f))
}
