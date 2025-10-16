use ews_proc_macros::operation_response;
use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{BaseFolderId, ItemShape, Items, View, MESSAGES_NS_URI};

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
    use crate::{test_utils::assert_serialized_content, BasePoint, BaseShape};

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

    #[test]
    fn test_serialize_find_item_calendar_view() {
        let finditem = FindItem {
            traversal: Traversal::Shallow,
            item_shape: ItemShape {
                base_shape: BaseShape::IdOnly,
                include_mime_content: None,
                additional_properties: None,
            },
            parent_folder_ids: vec![BaseFolderId::DistinguishedFolderId {
                id: "calendar".to_string(),
                change_key: None,
            }],
            view: Some(View::CalendarView {
                max_entries_returned: Some(2),
                start_date: "2006-05-18T00:00:00-08:00".to_string(),
                end_date: "2006-05-19T00:00:00-08:00".to_string(),
            }),
        };

        let expected = r#"<FindItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages" Traversal="Shallow"><ItemShape><t:BaseShape>IdOnly</t:BaseShape></ItemShape><CalendarView MaxEntriesReturned="2" StartDate="2006-05-18T00:00:00-08:00" EndDate="2006-05-19T00:00:00-08:00"/><ParentFolderIds><t:DistinguishedFolderId Id="calendar"/></ParentFolderIds></FindItem>"#;

        assert_serialized_content(&finditem, "FindItem", expected);
    }

    #[test]
    fn test_serialize_find_item_contacts_view() {
        let finditem = FindItem {
            traversal: Traversal::Shallow,
            item_shape: ItemShape {
                base_shape: BaseShape::IdOnly,
                include_mime_content: None,
                additional_properties: None,
            },
            parent_folder_ids: vec![BaseFolderId::DistinguishedFolderId {
                id: "contacts".to_string(),
                change_key: None,
            }],
            view: Some(View::ContactsView {
                max_entries_returned: Some(3),
                initial_name: Some("Kelly Rollin".to_string()),
                final_name: None,
            }),
        };

        let expected = r#"<FindItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages" Traversal="Shallow"><ItemShape><t:BaseShape>IdOnly</t:BaseShape></ItemShape><ContactsView MaxEntriesReturned="3" InitialName="Kelly Rollin"/><ParentFolderIds><t:DistinguishedFolderId Id="contacts"/></ParentFolderIds></FindItem>"#;

        assert_serialized_content(&finditem, "FindItem", expected);
    }
}
