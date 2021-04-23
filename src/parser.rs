//! Parser for the Arithmetic calculator grammar.
//! ```text
//! S ::= expr
//! expr ::= expr + expr
//!        | expr - expr
//!        | expr * expr
//!        | expr / expr
//!        | - expr
//!        | ( expr )
//!        | number
//! ```
pub mod ast;
pub mod top_down_parser;
