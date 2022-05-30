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
    generators: Vec<RistrettoPoint>,
    // base
    base: RistrettoPoint,
}

struct Commitment {}

impl Pedersen {
    fn new<R: RngCore + CryptoRng>(size: usize, rng: &mut R) -> Self {
        let mut generators: Vec<RistrettoPoint> = Vec::with_capacity(size);

        for i in 0..size {
            let sampled_generator = RistrettoPoint::random(rng);
            generators.push(sampled_generator);
        }

        Pedersen {
            commit_size: size,
            generators,
            base: RistrettoPoint::random(rng),
        }
    }

    // for now assume there are `commit_size` scalars
    fn commit(&self, scalars: Vec<Scalar>, randomness: Scalar) -> Commitment {
        let mut gens = self.generators.to_vec(); // boo-hoo
        gens.iter().zip(scalars.iter()).map(|(g, a)| g * a);
        gens.iter()
            .fold(self.base * randomness, |accum, commitment| {
                commitment + accum
            });
        println!("{gens:?}");
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
