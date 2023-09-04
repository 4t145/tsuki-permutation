It's a lib for represent and caculate permutation

```rust
let p = perm![1, 2, 0];
let q = perm![2, 1, 0];
// unit element of S3 group
let e3 = perm![@3];
assert_eq!(&e3 + &p, p);
assert_eq!(&p + &e3, p);
assert_eq!(&p * 3, e3);
assert_eq!(&p * 2, &e3 - &p);
dbg!(p);
assert_eq!(p.parity(), Parity::Even);
assert_eq!((&p + &p).parity(), Parity::Even);
assert_eq!((&q + &q).parity(), Parity::Even);
assert_eq!((&q + &p).parity(), Parity::Odd);
assert_eq!((&p + &q).parity(), Parity::Odd);
```
