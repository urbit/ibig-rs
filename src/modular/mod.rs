//! Modular arithmetic.
//!
//! Modular arithmetic is performed on values `Modulo` attached to a modular ring of integers,
//! `ModuloRing`.
//!
//! Trying to mix different rings (even with the same modulus!) will cause a panic.
//!
//! # Examples
//!
//! ```
//! use ibig::{prelude::*, modular::ModuloRing};
//!
//! let ring = ModuloRing::new(&ubig!(10000));
//! let x = ring.from(12345);
//! let y = ring.from(55443);
//! assert_eq!(format!("{}", x - y), "6902 (mod 10000)");
//! ```

pub use convert::IntoModulo;
pub use modulo::Modulo;
pub use modulo_ring::ModuloRing;

mod add;
mod cmp;
pub(crate) mod convert;
mod fmt;
pub(crate) mod modulo;
pub(crate) mod modulo_ring;
mod mul;
mod pow;
