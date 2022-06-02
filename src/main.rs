use curve25519_dalek_ng::{ristretto::RistrettoPoint, scalar::Scalar};
use pedersen_toy;
use rand_core::OsRng;

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

    let mut pedersen = pedersen_toy::Pedersen::new(100 as usize);

    let mut scalars: Vec<Scalar> = Vec::with_capacity(100);

    for i in 0..100 {
        scalars.push(Scalar::random(&mut rng))
    }

    let mut empty: Vec<Scalar> = Vec::new();
    pedersen.commit(empty, Scalar::zero());

    let r = Scalar::random(&mut rng);
    pedersen.commit(scalars, r);
}
