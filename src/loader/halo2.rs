pub(crate) mod loader;
pub mod poseidon_chip;

#[cfg(test)]
pub(crate) mod test;

pub use loader::{EcPoint, Halo2Loader, Scalar};
pub use util::Valuetools;

mod util {
    use halo2_proofs::circuit::Value;

    pub trait Valuetools<V>: Iterator<Item = Value<V>> {
        fn fold_zipped<B, F>(self, init: B, mut f: F) -> Value<B>
        where
            Self: Sized,
            F: FnMut(B, V) -> B,
        {
            self.into_iter().fold(Value::known(init), |acc, value| {
                acc.zip(value).map(|(acc, value)| f(acc, value))
            })
        }
    }

    impl<V, I: Iterator<Item = Value<V>>> Valuetools<V> for I {}
}
