#![feature(stmt_expr_attributes)]

use curve25519_dalek_ng::{ristretto::RistrettoPoint, scalar::Scalar};
use pedersen_toy::VectorCommitter;
use rand_core::OsRng;

use bulletproofs::PedersenGens;
use sha2::{Digest, Sha512};

fn main() {
    let mut rng = OsRng;

    let mut s = Scalar::random(&mut rng);

    println!("scalar: {s:?}");

    let mut z = Scalar::zero();
    println!("zero: {z:?}");

    let mut prod = s * z;
    println!("prod: {prod:?}");

    let mut point = RistrettoPoint::random(&mut rng);
    println!("point: {point:?}");

    let mut prod_point = z * point;
    println!("prod_point: {prod_point:?}");

    let mut point2 = RistrettoPoint::random(&mut rng);
    let mut prod_point2 = z * point2;

    println!("prod_point2: {prod_point2:?}");

    println!("canonical:");

    let canon_point = prod_point.compress();
    println!("prod_point_compress: {canon_point:?}");

    let canon_point2 = prod_point2.compress();
    println!("prod_point_compress2: {canon_point2:?}");

    println!("rest of code!");

    let mut pedersen = pedersen_toy::VectorCommitter::new(100 as usize);

    let mut scalars: Vec<Scalar> = Vec::with_capacity(100);

    for i in 0..100 {
        scalars.push(Scalar::random(&mut rng))
    }

    let mut empty: Vec<Scalar> = Vec::new();
    pedersen.commit(empty, Scalar::zero());

    let mut rng = OsRng;
    // CompressedRistretto: [64, 149, 245, 9, 225, 129, 49, 23, 244, 236, 33, 7, 240, 236, 169, 254, 180, 2, 186, 136, 10, 147, 152, 15, 9, 68, 179, 163, 8, 124, 255, 85]

    let B = RistrettoPoint::hash_from_bytes::<Sha512>("pedersen_domain_sep:0".as_ref());
    let B_blinding = RistrettoPoint::hash_from_bytes::<Sha512>("pedersen_domain_sep:H".as_ref());
    let mut dalek_pedersen = PedersenGens { B, B_blinding };

    let blinding_scalar = Scalar::hash_from_bytes::<Sha512>(b"blinding_factor");

    let preimages = vec![b"s1", b"s2", b"s3"];
    let len = preimages.len();
    let mut scalars = Vec::with_capacity(len);
    for preimage in preimages {
        scalars.push(Scalar::hash_from_bytes::<Sha512>(preimage));
    }

    let mut naive_trapdoor = VectorCommitter::new(len);
    let naive_commitment = naive_trapdoor.commit(scalars, blinding_scalar);
    let canonical_naive_commitment = naive_commitment.compress();
    println!("canonical naive: {canonical_naive_commitment:?}");

    let mut rng = OsRng;
    let mut trapdoor = VectorCommitter::new(100);
    let r1 = Scalar::random(&mut rng);
    let r2 = Scalar::random(&mut rng);
    let a_scalars: Vec<Scalar> = (0..100)
        .into_iter()
        .map(|_| Scalar::random(&mut rng))
        .collect::<Vec<Scalar>>();

    let b_scalars: Vec<Scalar> = (0..100)
        .into_iter()
        .map(|_| Scalar::random(&mut rng))
        .collect::<Vec<Scalar>>();

    let sum_scalars: Vec<Scalar> = b_scalars
        .iter()
        .zip(a_scalars.iter())
        .map(|(a, b)| a + b)
        .collect::<Vec<Scalar>>();
    println!("0test..");

    let c1 = trapdoor.commit(a_scalars, r1);
    let c2 = trapdoor.commit(b_scalars, r2);
    let sum = c1 + c2;
    println!("test..");

    let c3 = trapdoor.commit(sum_scalars, r1 + r2);
    println!("checking..");
    println!("{c3:?}");
    println!("{sum:?}");
}
