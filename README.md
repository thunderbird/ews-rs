# Exchange Web Services

`ews` is a crate providing data structures and APIs for working with Microsoft's
[Exchange Web Services API](https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-reference-for-exchange).

## Status
At present, the focus of this crate is providing Rust equivalents of the data
structures specified in the EWS reference and API for serializing to and
deserializing from XML.
No client is provided for handling HTTP requests or responses, and as such,
this crate does not provide handling of authentication, server-side throttling,
or other error conditions. Additionally, it does not include an implementation
of the Autodiscover protocol or any other means of discovering EWS endpoints.
This crate was built to support the work-in-progress EWS protocol for
[Thunderbird](https://thunderbird.net). If you have an interest in using this
crate and it is missing functionality you need or have ideas for improved
ergonomics, please get in touch with us.

## Minimum Supported Rust Version
The MSRV for `ews` is currently 1.62.1.

## License
`ews` is available under the terms of the Mozilla Public License, version 2.0.
See either our [LICENSE] file or [https://www.mozilla.org/en-US/MPL/2.0/].
