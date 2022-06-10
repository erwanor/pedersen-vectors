use curve25519_dalek_ng::{ristretto::RistrettoPoint, scalar::Scalar, traits::MultiscalarMul};

use sha2::Sha512;

//replace with const generic
pub struct VectorCommitter {
    // number of generators in the scheme
    commit_size: usize,
    // group generators
    generators: Vec<RistrettoPoint>,
    // blinding base
    base_blinding: RistrettoPoint,
}

type Commitment = RistrettoPoint;

impl VectorCommitter {
    pub fn new(size: usize) -> Self {
        let mut generators = (0..size)
            .into_iter()
            .map(|index| {
                let msg = format!("pedersen_domain_sep:{}", index);
                RistrettoPoint::hash_from_bytes::<Sha512>(msg.as_bytes())
            })
            .collect::<Vec<RistrettoPoint>>();

        let base_blinding =
            RistrettoPoint::hash_from_bytes::<Sha512>("pedersen_domain_sep:H".as_bytes());

        generators.push(base_blinding);

        Self {
            commit_size: size,
            generators,
            base_blinding,
        }
    }

    // commit to N scalars, with randomness
    pub fn commit(&mut self, mut scalars: Vec<Scalar>, randomness: Scalar) -> Commitment {
        if scalars.len() > self.commit_size {
            unimplemented!("committing too many elements")
        }

        let num_zero_elements = self.commit_size - scalars.len();

        (0..num_zero_elements)
            .into_iter()
            .for_each(|_| scalars.push(Scalar::zero()));

        scalars.push(randomness);

        RistrettoPoint::multiscalar_mul(scalars, self.generators.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    #[test]
    // Check that com_ck(A, q) \cdot com_ck(B, r) = com_ck(A+B, q+r)
    fn homomorphic_addition() {
        let mut rng = OsRng;
        let mut trapdoor = VectorCommitter::new(100);
        let r1 = Scalar::random(&mut rng);
        let r2 = Scalar::random(&mut rng);

        let mut a_scalars: Vec<Scalar> = Vec::with_capacity(100);
        let mut b_scalars: Vec<Scalar> = Vec::with_capacity(100);
        let mut sum_scalars: Vec<Scalar> = Vec::with_capacity(100);

        (0..100).into_iter().for_each(|_| {
            let a = Scalar::random(&mut rng);
            let b = Scalar::random(&mut rng);
            a_scalars.push(a);
            b_scalars.push(b);
            sum_scalars.push(a + b);
        });

        let c1 = trapdoor.commit(a_scalars, r1);
        let c2 = trapdoor.commit(b_scalars, r2);
        let sum = c1 + c2;

        let c3 = trapdoor.commit(sum_scalars, r1 + r2);

        assert_eq!(sum.compress(), c3.compress());
    }

    #[test]
    // Test vector across versions
    fn consistent_vector() {
        let blinding_scalar = Scalar::hash_from_bytes::<Sha512>(b"blinding_factor");

        let scalar_preimages = vec![b"s1", b"s2", b"s3"];
        let mut scalars = Vec::with_capacity(scalar_preimages.len());
        for preimage in scalar_preimages.iter() {
            scalars.push(Scalar::hash_from_bytes::<Sha512>(preimage.as_slice()));
        }

        let mut naive_trapdoor = VectorCommitter::new(scalar_preimages.len());
        let naive_commitment = naive_trapdoor.commit(scalars, blinding_scalar).compress();
        let compressed_commitment_test_vector = [
            20, 32, 230, 13, 11, 148, 239, 179, 111, 36, 209, 242, 200, 173, 167, 30, 25, 202, 98,
            143, 245, 64, 128, 210, 14, 1, 216, 208, 185, 226, 211, 12,
        ];
        assert_eq!(
            naive_commitment.to_bytes(),
            compressed_commitment_test_vector
        );
    }
}
