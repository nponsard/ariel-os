//! Abstractions around the permissions of a particular security context.

use crate::scope::Scope;

/// Claims about a peer that are independent of the security mechanism.
///
/// A [`GeneralClaims`] instance represents processed properties of a particular security
/// connection peer.
///
/// The data is similar to a CWT's claims, but does not include ACE profile specifics (eg. the
/// confirmation data), may come from a source that does not even originally stem from ACE (eg.
/// when a raw public key is known) and also contains data not typically expressed in a CWT (eg.
/// whether these claims represent a more valuable connection for the purpose of discarding
/// connections).
pub trait GeneralClaims: core::fmt::Debug {
    /// An internal representation of a scope (which may be parsed from a CWT).
    ///
    /// Being generic, this allow both to transport claims in their original form (copied into a
    /// buffer and processed request by request) or to be preprocessed further (eg. converting
    /// paths in an AIF into an enum that indicates a resource).
    type Scope: Scope;

    /// Accesses the scope of the claim.
    ///
    /// This is used to decide whether a particular request is allowed on a particular resource.
    fn scope(&self) -> &Self::Scope;

    /// Accesses the temporal validity of the claim.
    ///
    /// This is evaluated independently of the request's content, and may be evaluated without a
    /// request when eviction of a security context is being considered.
    fn time_constraint(&self) -> crate::time::TimeConstraint;

    /// Access whether a security context is important.
    ///
    /// This is intentionally vague (importance of a security context can vary by application), but
    /// useful for keeping administrative security contexts around even when attackers can create
    /// many low-authorization contexts.
    fn is_important(&self) -> bool {
        false
    }
}

impl GeneralClaims for core::convert::Infallible {
    type Scope = core::convert::Infallible;

    fn scope(&self) -> &Self::Scope {
        self
    }

    fn time_constraint(&self) -> crate::time::TimeConstraint {
        match *self {}
    }
}

/// An implementation of [`GeneralClaims`] that puts no additional restrictions on the [`Scope`]
/// `S` it contains.
#[derive(Debug)]
pub struct Unlimited<S: Scope>(pub S);

impl<S: Scope> GeneralClaims for Unlimited<S> {
    type Scope = S;

    fn scope(&self) -> &Self::Scope {
        &self.0
    }

    fn time_constraint(&self) -> crate::time::TimeConstraint {
        crate::time::TimeConstraint::unbounded()
    }
}
