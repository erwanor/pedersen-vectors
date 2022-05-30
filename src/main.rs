use curve25519_dalek_ng::scalar::Scalar;
use pedersen_toy;
use rand_core::OsRng;

fn main() {
    let mut rng = OsRng;
    let mut pedersen = pedersen_toy::Pedersen::new(100 as usize, &mut rng);

    let mut scalars: Vec<Scalar> = Vec::with_capacity(100);

    for i in 0..100 {
        scalars.push(Scalar::random(&mut rng))
    }

    let r = Scalar::random(&mut rng);
    pedersen.commit(scalars, r);
}
