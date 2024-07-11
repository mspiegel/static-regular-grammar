use std::fmt::Display;

use btree_range_map::{AnyRange, RangeMap};
use quote::quote;

use crate::{
	byteset,
	utils::{automaton, MergeRef, Sanitized},
	ByteSet,
};

use super::{Token, TokenMap, TokenRange, TokenSet};

impl Token for u8 {
	type Range = AnyRange<u8>;

	type Set = ByteSet;

	type Map<V> = RangeMap<u8, V>;

	const UNICODE: bool = false;

	fn from_u8(b: u8) -> Self {
		b
	}

	fn from_char(c: char) -> Option<Self> {
		if c.is_ascii() {
			Some(c as u8)
		} else {
			None
		}
	}

	fn from_u32(v: u32) -> Option<Self> {
		if v <= 0xff {
			Some(v as u8)
		} else {
			None
		}
	}

	fn fmt_token(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		Sanitized(*self).fmt(f)
	}

	fn rust_type() -> proc_macro2::TokenStream {
		quote!(u8)
	}

	fn rust_string_type() -> proc_macro2::TokenStream {
		quote!([u8])
	}

	fn rust_owned_string_type() -> proc_macro2::TokenStream {
		quote!(Vec<u8>)
	}

	fn rust_iterator_method() -> proc_macro2::TokenStream {
		quote!(iter().copied())
	}

	fn rust_as_inner_method() -> proc_macro2::TokenStream {
		quote!(as_bytes)
	}

	fn rust_into_inner_method() -> proc_macro2::TokenStream {
		quote!(into_bytes)
	}

	fn is_ascii(automaton: &automaton::DetAutomaton<u32, Self::Set>) -> bool {
		for transitions in automaton.transitions().values() {
			if transitions.keys().any(|set| !set.is_ascii()) {
				return false;
			}
		}

		true
	}

	fn rust_inner_as_ascii_method_body() -> Option<proc_macro2::TokenStream> {
		Some(quote!(unsafe { ::core::str::from_utf8_unchecked(&self.0) }))
	}

	fn rust_inner_into_ascii_method_body() -> Option<proc_macro2::TokenStream> {
		Some(quote!(unsafe {
			String::from_utf8_unchecked(self.0)
		}))
	}

	fn rust_empty_string() -> proc_macro2::TokenStream {
		quote! {
			b""
		}
	}
}

impl TokenRange<u8> for AnyRange<u8> {
	fn new(a: u8, b: u8) -> Self {
		(a..=b).into()
	}

	fn peek(&self) -> Option<u8> {
		self.first()
	}
}

impl TokenSet<u8> for ByteSet {
	fn singleton(token: u8, case_sensitive: bool) -> Self {
		ByteSet::from_u8(token, case_sensitive)
	}

	fn is_empty(&self) -> bool {
		self.is_empty()
	}

	fn len(&self) -> usize {
		self.len() as usize
	}

	fn peek(&self) -> Option<u8> {
		self.first()
	}

	fn intersects_range(&self, range: <u8 as Token>::Range) -> bool {
		self.intersects(range)
	}

	fn merge_with(&mut self, other: Self) {
		self.extend(other.ranges())
	}

	fn rust_set(&self) -> proc_macro2::TokenStream {
		let ranges = self.ranges().map(|range| match range.len() {
			0 => panic!("empty range"),
			1 => {
				let a = range.first().unwrap();
				quote! {
					#a
				}
			}
			_ => {
				let a = range.first().unwrap();
				let b = range.last().unwrap();
				quote! {
					#a..=#b
				}
			}
		});

		quote! { #(#ranges)|* }
	}
}

impl MergeRef for ByteSet {
	fn merge_with_ref(&mut self, other: &Self) {
		self.extend(other.ranges())
	}
}

impl automaton::DeterminizeLabel for ByteSet {
	type Range = AnyRange<u8>;

	type Ranges<'a> = byteset::Ranges<'a>;

	type RangeMap<V: Clone + PartialEq> = RangeMap<u8, V>;

	fn ranges(&self) -> Self::Ranges<'_> {
		ByteSet::ranges(self)
	}

	fn insert_range(&mut self, range: <u8 as Token>::Range) {
		self.insert(range)
	}
}

impl<V> TokenMap<u8, V> for RangeMap<u8, V> {
	type Iter<'a> = btree_range_map::generic::map::Iter<'a, u8, V, btree_range_map::DefaultMapContainer<u8, V>> where V: 'a, Self: 'a;

	fn is_empty(&self) -> bool {
		self.is_empty()
	}

	fn len(&self) -> usize {
		self.len() as usize
	}

	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}

	fn insert_range(&mut self, range: <u8 as Token>::Range, value: V)
	where
		V: PartialEq + Clone,
	{
		self.insert(range, value)
	}

	fn insert(&mut self, set: <u8 as Token>::Set, value: V)
	where
		V: PartialEq + Clone,
	{
		let mut ranges = set.ranges();

		if let Some(first) = ranges.next() {
			for range in ranges {
				self.insert_range(range, value.clone())
			}

			self.insert_range(first, value)
		}
	}

	fn update_range(&mut self, range: <u8 as Token>::Range, f: impl Fn(Option<&V>) -> Option<V>)
	where
		V: PartialEq + Clone,
	{
		self.update(range, f)
	}

	fn update(&mut self, set: &<u8 as Token>::Set, f: impl Fn(Option<&V>) -> Option<V>)
	where
		V: PartialEq + Clone,
	{
		for range in set.ranges() {
			self.update(range, &f)
		}
	}
}

impl<V: Clone + PartialEq> automaton::RangeMap<AnyRange<u8>, V> for RangeMap<u8, V> {
	fn update(&mut self, key: AnyRange<u8>, f: impl Fn(Option<&V>) -> Option<V>) {
		RangeMap::update(self, key, f)
	}
}
