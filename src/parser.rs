//! Parser for the Arithmetic calculator grammar.
//! ```text
//! <expr> ::= <expr> + <term>
//!          | <expr> - <term>
//!          | <term>
//! 
//! <term> ::= <term> * <factor>
//!          | <term> / <factor>
//!          | <factor>
//! 
//! <factor> ::= ( <expr> )
//!            | Num
//!            | - <factor>
//! ```
pub mod top_down_parser;
