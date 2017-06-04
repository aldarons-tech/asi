// repurpose.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

//! This module is for repurposing memory allocated by rust.

use size_of::*;

#[cfg(debug_assertions)]
#[inline(always)]
fn is_safe<T, U>() {
	let sizet = size_of::<T>();
	let sizeu = size_of::<U>();
	if sizet != sizeu {
		panic!("size of u ({}) does not match size of t ({}).", sizeu,
			sizet);
	}
}

#[cfg(not(debug_assertions))]
#[inline(always)]
fn is_safe<T, U>() { }

/// Repurpose memory of type `T` as memory of type `U`.  `input` is the a
/// reference to the memory to repurpose.  Returns the repurposed memory.
#[inline(always)]
pub fn repurpose<T, U>(input: &mut T) -> &mut U {
	is_safe::<T, U>();
	// Actual repurpose.
	unsafe { &mut *(input as *mut T as *mut U) }
}
