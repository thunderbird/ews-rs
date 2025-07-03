# Exchange Web Services

This rust crate holds types that represent data structures and operations for
the Exchange Web Services API, as well as the necessary infrastructure to
serialize and deserialize them to/from XML.

## Documentation

The Cargo documentation for this repository is not currently hosted online. It
can be accessed locally after cloning this repository and generating it:

```bash
git clone https://github.com/thunderbird/ews-rs.git
cd ews-rs
cargo doc --open
```

## Report issues

The GitHub issue tracker for this repository is disabled to help us handle
EWS-related Thunderbird-adjacent bugs more easily. To report an issue or file a
feature request for this crate, please do so on Bugzilla
[here](https://bugzilla.mozilla.org/enter_bug.cgi?product=MailNews%20Core&component=Networking:%20Exchange).

## Minimum Supported Rust Version

The MSRV for `ews` is currently 1.62.1.

## License

`ews` is available under the terms of the Mozilla Public License, version 2.0.
See either our [LICENSE] file or [https://www.mozilla.org/en-US/MPL/2.0/].
