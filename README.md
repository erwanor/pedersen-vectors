# Pedersen commitment of vectors built on Ristretto

This is a first version of a vector commitment library built on the [dalek-ng](https://github.com/zkcrypto/curve25519-dalek-ng/) implementation of the [Ristretto Group](https://ristretto.group/what_is_ristretto.html). This crate hasn't been audited and should _NOT_ be used for anything other than PoCs.

## Using [bulletproofs::PedersenGens](https://docs.rs/bulletproofs/4.0.0/bulletproofs/struct.PedersenGens.html) instead

Unless you _need_ to do vector commitments, you should be able to use the Pedersen commitment implementation that lives in the bulletproof crate linked above. It's a simple to use abstraction that lets you swap the bases as you wish.

## API

```rust

use pedersen_vectors::VectorCommiter;

let mut trapdoor = VectorCommitter::new(1_000_000);
let blinding_factor = Scalar::random(&mut rng);
let empty_vector: Vec<Scalar> = Vec::new();
let commitment = trapdoor(empty_vector, blinding_factor);
```


## How are the bases generated?

For a `VectorCommitter` of size `N`, we generate `N` bases: $G_i := PRF("pedersen_domain_sep" || i)$ as well as a blinding base $H := PRF("pedersen_domain_sep:H"). This will change in future versions. Please keep in mind that this is a v0, for a PoC. There should be no expectation of stable APIs, and much less so stable internals.
