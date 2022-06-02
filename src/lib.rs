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

type Commitment = RistrettoPoint;

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
        gens = gens
            .iter()
            .zip(scalars.iter())
            .map(|(g, a)| g * a)
            .collect();
        let commitment = gens
            .iter()
            .fold(self.base * randomness, |accum, commitment| {
                commitment + accum
            });
        println!("{commitment:?}");
        commitment
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn scalar_addition() {
        let mut rng = OsRng;
        let mut trapdoor = Pedersen::new(100);

        let mut a_scalars: Vec<Scalar> = (0..100)
            .into_iter()
            .map(|_| Scalar::random(&mut rng))
            .collect::<Vec<Scalar>>();

        let mut b_scalars: Vec<Scalar> = (0..100)
            .into_iter()
            .map(|_| Scalar::random(&mut rng))
            .collect::<Vec<Scalar>>();

        let mut sum_scalars: Vec<Scalar> = b_scalars
            .iter()
            .zip(a_scalars.iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<Scalar>>();

        b_scalars
            .iter()
            .zip(a_scalars.iter())
            .zip(sum_scalars.iter())
            .for_each(|((&a, &b), &sum)| assert_eq!(a + b, sum));

        let elem_a = a_scalars.get(10).unwrap().clone();
        let elem_b = b_scalars.get(10).unwrap().clone();
        let elem_c = sum_scalars.get(10).unwrap().clone();
        assert_eq!(elem_a + elem_b, elem_c);
    }

    #[test]
    fn homomorphic_addition() {
        let mut rng = OsRng;
        let mut trapdoor = Pedersen::new(100);
        let r1 = Scalar::random(&mut rng);
        let r2 = Scalar::random(&mut rng);

        let mut a_scalars: Vec<Scalar> = (0..100)
            .into_iter()
            .map(|_| Scalar::random(&mut rng))
            .collect::<Vec<Scalar>>();

        let mut b_scalars: Vec<Scalar> = (0..100)
            .into_iter()
            .map(|_| Scalar::random(&mut rng))
            .collect::<Vec<Scalar>>();

        let mut sum_scalars: Vec<Scalar> = b_scalars
            .iter()
            .zip(a_scalars.iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<Scalar>>();

        let c1 = trapdoor.commit(a_scalars, r1);
        let c2 = trapdoor.commit(b_scalars, r2);
        let sum = c1 + c2;

        let c3 = trapdoor.commit(sum_scalars, r1 + r2);

        assert_eq!(sum.compress(), c3.compress());
    }
}
