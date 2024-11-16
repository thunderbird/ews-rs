use crate::types::sealed::EnvelopeBodyContents;
use crate::{BaseItemId, Operation, OperationResponse, MESSAGES_NS_URI};
use serde::Deserialize;
use xml_struct::XmlSerialize;

/// A request to update junk status of one or more Exchange items.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/markasjunk>
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct MarkAsJunk {
    /// Specifies if the item is considered junk.
    #[xml_struct(attribute)]
    pub is_junk: bool,

    /// Specifies if the item should be moved.
    #[xml_struct(attribute)]
    pub move_item: bool,

    /// A list of item IDs to mark as junk.
    #[xml_struct(ns_prefix = "m")]
    pub item_ids: ItemIds,
}

impl Operation for MarkAsJunk {
    type Response = MarkAsJunkResponse;
}

impl EnvelopeBodyContents for MarkAsJunk {
    fn name() -> &'static str {
        "m:MarkAsJunk"
    }
}

#[derive(Clone, Debug, XmlSerialize)]
pub struct ItemIds {
    #[xml_struct(flatten, ns_prefix = "t")]
    pub items: Vec<BaseItemId>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarkAsJunkResponse {
    // Add fields as needed to match the structure of the response from EWS.
}

impl OperationResponse for MarkAsJunkResponse {}

impl EnvelopeBodyContents for MarkAsJunkResponse {
    fn name() -> &'static str {
        "MarkAsJunkResponse"
    }
}
