use curve25519_dalek_ng::{
    ristretto::{self, CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use rand::{CryptoRng, RngCore};

// An instantiation of a Pedersen commitment schemes
struct Pedersen {
    // number of generators in the scheme
    commit_size: usize,
    // group generators
    generators: Vec<CompressedRistretto>,
}

struct Commitment {}

impl Pedersen {
    fn new<R: RngCore + CryptoRng>(size: usize, rng: &mut R) -> Self {
        let mut generators: Vec<CompressedRistretto> = Vec::with_capacity(size);

        for i in 0..size {
            let sampled_generator = RistrettoPoint::random(rng).compress();
            generators.push(sampled_generator);
        }

        Pedersen {
            commit_size: size,
            generators,
        }
    }

    fn commit(scalars: Vec<Scalar>, randomness: Scalar) -> Commitment {
        Commitment {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
