#[cfg(not(feature = "std"))]
extern crate alloc;

use static_regular_grammar::RegularGrammar;

/// IRI.
#[derive(RegularGrammar, PartialEq, Eq)]
#[grammar(
	file = "examples/iri.abnf",
	cache = "target/examples/iri.automaton.cbor",
	sized(IriBuf, derive(PartialEq, Eq)),
	serde
)]
pub struct Iri(str);

fn main() {
	Iri::new("https://www.rust-lang.org/foo/bar?query#frag").unwrap();
}
