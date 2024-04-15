/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use serde::Deserialize;
use xml_struct::XmlSerialize;

pub trait Operation: XmlSerialize + sealed::NamedStructure {
    type Response: OperationResponse;
}

pub trait OperationResponse: for<'de> Deserialize<'de> + sealed::NamedStructure {}

pub(super) mod sealed {
    pub trait NamedStructure {
        fn name() -> &'static str;
    }
}
