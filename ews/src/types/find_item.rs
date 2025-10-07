use ews_proc_macros::operation_response;
use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{BaseFolderId, ItemShape, Items, MESSAGES_NS_URI};

/// Defines a request to find items in mailbox.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem>
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
#[operation_response(FindItemResponseMessage)]
pub struct FindItem {
    #[xml_struct(attribute)]
    pub traversal: Traversal,

    pub item_shape: ItemShape,

    #[xml_struct(flatten)]
    pub view: Option<View>,

    pub parent_folder_ids: Vec<BaseFolderId>,
}

/// Defines whether the search finds items in folders or the folders' dumpsters.
/// This attribute is required.
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(text)]
pub enum Traversal {
    /// Returns only the identities of items in the folder.
    Shallow,

    /// Returns only the identities of items that are in a folder's dumpster.
    /// Note that a soft-deleted traversal combined with a search restriction
    /// will result in zero items returned even if there are items that match the search criteria.
    SoftDeleted,

    ///Returns only the identities of associated items in the folder.
    Associated,
}

#[derive(Clone, Debug, XmlSerialize)]
pub enum View {
    /// Describes how paged conversation or item information is
    /// returned for a [`FindItem`] operation or `FindConversation` operation request.
    IndexedPageItemView {
        #[xml_struct(attribute)]
        max_entries_returned: Option<usize>,

        #[xml_struct(attribute)]
        base_point: BasePoint,

        #[xml_struct(attribute)]
        offset: usize,
    },

    /// Describes where the paged view starts and the
    /// maximum number of items returned in a [`FindItem`] request.
    FractionalPageItemView {
        /// Identifies the maximum number of results to return in the FindItem response.
        /// If this attribute is not specified, the call will return all available items.
        #[xml_struct(attribute)]
        max_entries_returned: Option<usize>,

        /// Represents the numerator of the fractional offset from the start of the result set.
        /// The numerator must be equal to or less than the denominator.
        /// This attribute must represent an integral value that is equal to or greater than zero.
        #[xml_struct(attribute)]
        numerator: usize,

        /// Represents the denominator of the fractional offset from the start
        /// of the total number of items in the result set.
        /// This attribute must represent an integral value that is greater than one.
        #[xml_struct(attribute)]
        denominator: usize,
    },
}

/// Describes whether the page of items or conversations will start from the
/// beginning or the end of the set of items or conversations that are found by using
/// the search criteria.
/// Seeking from the end always searches backward.
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(text)]
pub enum BasePoint {
    Beginning,
    End,
}

/// Contains the status and result of a single [`FindItem`] operation request.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditemresponsemessage>
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct FindItemResponseMessage {
    pub items: Items,
}

#[cfg(test)]
mod tests {
    use crate::{test_utils::assert_serialized_content, BaseShape};

    use super::*;

    #[test]
    fn test_serialize_find_item_indexed_page_item_view() {
        let find_item = FindItem {
            traversal: Traversal::Shallow,
            item_shape: ItemShape {
                base_shape: BaseShape::IdOnly,
                include_mime_content: None,
                additional_properties: None,
            },
            parent_folder_ids: vec![BaseFolderId::DistinguishedFolderId {
                id: "deleteditems".to_string(),
                change_key: None,
            }],
            view: Some(View::IndexedPageItemView {
                max_entries_returned: Some(6),
                offset: 0,
                base_point: BasePoint::Beginning,
            }),
        };

        let expected = r#"<FindItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages" Traversal="Shallow"><ItemShape><t:BaseShape>IdOnly</t:BaseShape></ItemShape><IndexedPageItemView MaxEntriesReturned="6" BasePoint="Beginning" Offset="0"/><ParentFolderIds><t:DistinguishedFolderId Id="deleteditems"/></ParentFolderIds></FindItem>"#;

        assert_serialized_content(&find_item, "FindItem", expected);
    }

    #[test]
    fn test_serialize_find_item_fractional_page_item_view() {
        let finditem = FindItem {
            traversal: Traversal::Shallow,
            item_shape: ItemShape {
                base_shape: BaseShape::IdOnly,
                include_mime_content: None,
                additional_properties: None,
            },
            parent_folder_ids: vec![BaseFolderId::DistinguishedFolderId {
                id: "inbox".to_string(),
                change_key: None,
            }],
            view: Some(View::FractionalPageItemView {
                max_entries_returned: Some(12),
                numerator: 2,
                denominator: 3,
            }),
        };

        let expected = r#"<FindItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages" Traversal="Shallow"><ItemShape><t:BaseShape>IdOnly</t:BaseShape></ItemShape><FractionalPageItemView MaxEntriesReturned="12" Numerator="2" Denominator="3"/><ParentFolderIds><t:DistinguishedFolderId Id="inbox"/></ParentFolderIds></FindItem>"#;
        assert_serialized_content(&finditem, "FindItem", expected);
    }
}
