// Copyright 2019 Amar Singh
// This file is part of Sunshine, licensed with the MIT License

// high performance Rust required with this one
trait Metagov {
	// `Asynchronize`
	// generic function that takes a list of functions and returns their variants as future (`Poll<T>`)
	// this would be useful for gradually upgrading the current `RageQuit` function
	// PROCEDURAL MACRO DEFINITION
	// @input a synchronous and fallible function
	// @return an asynchronous and fallible function
	fn Asynchronize<F, T, O, E, G>(f: F, i: I, o: O, e: E) -> impl Async fn G 
		where
			F: fn(T) -> Result<I, E>,
			G: Async fn(T) -> Poll<I, E>,
    {	
        unimplemented!();
        // return G;
    }

	// Define Voting on Parameterizations
	// @input a list of functions as well as 
	// @input corresponding parameters that can be voted on
	// AND
	// @input a list of the voting algorithms for voting on each parameter (take some enum variant)
	// --> make it simple enough to say {pick_one, this_one} instead of a yes, no, for every one
	// ----> optional parameters with `PhantomData<T>` 
	// --> define the set of voters (maybe as associated items)
}