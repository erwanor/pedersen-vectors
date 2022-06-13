# Pedersen commitment of vectors built on Ristretto

This is a first version of a vector commitment library built on the [dalek-ng](https://github.com/zkcrypto/curve25519-dalek-ng/) implementation of the [Ristretto Group](https://ristretto.group/what_is_ristretto.html). This crate hasn't been audited and should _NOT_ be used for anything other than PoCs. There should be no expectations of consistent or stable APIs at this point. If you don't need to do vector commitments, then you should use [bulletproofs::PedersenGens](https://docs.rs/bulletproofs/4.0.0/bulletproofs/struct.PedersenGens.html).

## API

```rust

use pedersen_vectors::VectorCommiter;

let mut trapdoor = VectorCommitter::new(1_000_000);
let blinding_factor = Scalar::random(&mut rng);
let empty_vector: Vec<Scalar> = Vec::new();
// We commit to the zero vector here
let commitment_zero_vector = trapdoor(empty_vector, blinding_factor);
```

## How are the bases generated?

The recipe to generate the commitment's bases *will change* before this crate hits v1.0.0. As of v0.1.2, for a `VectorCommitter` of size `N` where $0 \leq i \lt N$, we compute $G_i := PRF(\text{"pedersen\_domain\_sep"} || i)$. As well as an additional binding base $H := PRF(\text{"pedersen\_domain\_sep:H"})$. 

## Using [bulletproofs::PedersenGens](https://docs.rs/bulletproofs/4.0.0/bulletproofs/struct.PedersenGens.html) instead

Unless you _need_ to do vector commitments, you should be able to use the Pedersen commitment implementation that lives in the bulletproof crate. It offers a similar API, and swappable bases as well.

