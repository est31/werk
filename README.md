# Werk

A project to rewrite the C components of `libopus` in safe Rust.

For the transition, I'm taking C files and translate them to Rust, while keeping the test suite passing after each component got rewritten.

The test suite can be executed by doing `./run_tests.sh`.

This makes mistakes easier to track down as I can focus on only the most recently changed code.

The rewritten functions expose the same C API so that they can be called by code that is still in C. Some further reasons:

* Similarity with the C code is important to make it easier to add fixed point support later in the process
* Same for code that is SSE enhanced or written in assembly

This means that we sometimes are forced to use unsafe, e.g. because we are given a pointer when a slice would've been better. Once the porting process has been advanced enough that there is no more C code calling the function, and the function doesn't need any special care for fixed point support, we will be able change its ABI. That is something for the future though.

The first milestone this project targets is providing a pure Rust decoder with a "Rusty" API, with use of as little unsafe as possible (but sadly, some of it will be required due to the API compatibility noted above).

## Name

The name comes from german "Werk" and means the same as "opus", just in a different language.

## License

Licensed under the BSD 3 clause license. For details, see the [COPYING](COPYING) file.

### License of your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you, as defined in the Apache-2.0 license,
shall be BSD 3 clause licensed as above, without any additional terms or conditions.
