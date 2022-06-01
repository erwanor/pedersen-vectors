use curve25519_dalek_ng::{
    ristretto::{self, CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use rand::{CryptoRng, RngCore};

// An instantiation of a Pedersen commitment schemes
pub struct Pedersen {
    // number of generators in the scheme
    commit_size: usize,
    // group generators
    generators: Vec<RistrettoPoint>,
    // base
    base: RistrettoPoint,
}

pub struct Commitment {}

impl Pedersen {
    pub fn new<R: RngCore + CryptoRng>(size: usize, rng: &mut R) -> Self {
        let mut generators = (0..size)
            .into_iter()
            .map(|index| RistrettoPoint::random(rng))
            .collect::<Vec<RistrettoPoint>>();

        Pedersen {
            commit_size: size,
            generators,
            base: RistrettoPoint::random(rng),
        }
    }

    // commit to N scalars, with randomness
    pub fn commit(&self, mut scalars: Vec<Scalar>, randomness: Scalar) -> Commitment {
        if scalars.len() > self.commit_size {
            unimplemented!("committing too many elements")
        } else {
            (0..(scalars.len() - self.commit_size))
                .into_iter()
                .map(|_| scalars.push(Scalar::zero()))
                .collect::<Vec<_>>();
        }

        let mut gens = self.generators.to_vec(); // boo-hoo
        gens.iter().zip(scalars.iter()).map(|(g, a)| g * a);
        gens.iter()
            .fold(self.base * randomness, |accum, commitment| {
                commitment + accum
            });
        println!("{gens:?}");
        unimplemented!()
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
