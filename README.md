# Werk

Opus decoder. WIP.

## Roadmap

- [ ] Create a directory dev which contains a separate crate that uses this one
- [ ] Make that crate use the ogg crate to decode a passed ogg file
- [ ] Decode ID and comment headers from [RFC 7845](https://tools.ietf.org/html/rfc7845)
- [ ] Implement the frame packing
- [ ] Implement the range decoder
- [ ] In separate repo: Create FFI for the reference decoder and try to modularize its silk and celt decoders
- [ ] Use the silk and celt decoders from the FFI to test the range decoder
- [ ] Implement the SILK part
- [ ] Implement the CELT part
- [ ] ??
- [ ] Profit!

## Name

The name comes from german "Werk" and means the same as "opus".

## License

Licensed under Apache 2 or MIT (at your option). For details, see the [LICENSE](LICENSE) file.

All examples inside the `examples/` folder are licensed under the
[CC-0](https://creativecommons.org/publicdomain/zero/1.0/) license.

### License of your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed / CC-0 licensed as above, without any additional terms or conditions.
