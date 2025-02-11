pub trait GTKTester<E> where
    E: std::fmt::Display
{
    fn steps() -> Result<(), E>;
}


