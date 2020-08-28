// Copyright (c) DUSK NETWORK. All rights reserved.
// Licensed under the MPL 2.0 license. See LICENSE file in the project root for details.”
//! Gadget Errors Module.
//!
//! Includes the definitions of all of the possible errors that the gadgets
//! might encounter with toghether with it's display message implementations.
use thiserror::Error;

/// Represents an error during the execution of one of the library gagets.
#[derive(Error, Debug)]
pub enum GadgetErrors {
    /// Error returned when we try to compute the inverse of a number which is
    /// non-QR (doesn't have an inverse inside of the field)
    #[error("error on the computation of an inverse")]
    NonExistingInverse,
}
