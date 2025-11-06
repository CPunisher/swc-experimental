mod class;
mod decl;
mod expr;
mod function;
mod ident;
mod lit;
mod module;
mod module_decl;
mod operator;
mod pat;
mod prop;
mod stmt;

pub use class::*;
pub use decl::*;
pub use expr::*;
pub use function::*;
pub use ident::*;
pub use lit::*;
pub use module::*;
pub use module_decl::*;
pub use operator::*;
pub use pat::*;
pub use prop::*;
use rspack_experimental_swc_ast_macros::ast;
pub use stmt::*;

/// Represents a invalid node.
#[ast]
pub struct Invalid {}
