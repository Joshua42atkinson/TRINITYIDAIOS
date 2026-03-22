//! Trinity Isomorphic Ontology - Material Integrity
//!
//! Defines the traits that govern the "Meaning Making" of our code.
//! Integrated into trinity-protocol as part of the core reconciliation.

/// Signifies that a type has a standardized structural definition
/// and meets the "foundry standards" for the Trinity ID OS.
pub trait MaterialIntegrity {
    /// Perform a check of the structural integrity of the type.
    fn integrity_check(&self) -> bool;

    /// Returns the foundry mark (identifier) of the plate.
    fn foundry_mark(&self) -> &'static str;
}
