/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use serde::Deserialize;
use xml_struct::XmlSerialize;

pub trait Operation: XmlSerialize {
    type Response: OperationResponse;

    fn name() -> &'static str;
}

pub trait OperationResponse: for<'de> Deserialize<'de> {
    fn name() -> &'static str;
}
