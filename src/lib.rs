use curve25519_dalek_ng::{
    ristretto::{self, CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};

use rand::{CryptoRng, RngCore};
use sha2::Sha512;

// An instantiation of a Pedersen commitment schemes
pub struct Pedersen {
    // number of generators in the scheme
    commit_size: usize,
    // group generators
    generators: Vec<RistrettoPoint>,
    // base
    base: RistrettoPoint,
}

pub struct Commitment(CompressedRistretto);

impl Pedersen {
    pub fn new(size: usize) -> Self {
        let mut generators = (0..size)
            .into_iter()
            .map(|index| {
                let msg = format!("pedersen_domain_sep:{}", index);
                RistrettoPoint::hash_from_bytes::<Sha512>(msg.as_bytes())
            })
            .collect::<Vec<RistrettoPoint>>();

        Pedersen {
            commit_size: size,
            generators,
            base: RistrettoPoint::hash_from_bytes::<Sha512>(
                format!("pedersen_domain_sep:H").as_bytes(),
            ),
        }
    }

    // commit to N scalars, with randomness
    pub fn commit(&self, mut scalars: Vec<Scalar>, randomness: Scalar) -> Commitment {
        if scalars.len() > self.commit_size {
            unimplemented!("committing too many elements")
        } else {
            (0..(self.commit_size - scalars.len()))
                .into_iter()
                .map(|_| scalars.push(Scalar::zero()))
                .collect::<Vec<_>>();
        }

        let mut gens = self.generators.to_vec(); // boo-hoo
        gens.iter().zip(scalars.iter()).map(|(g, a)| g * a);
        let commitment = gens
            .iter()
            .fold(self.base * randomness, |accum, commitment| {
                commitment + accum
            });
        println!("{commitment:?}");
        Commitment(commitment.compress())
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
