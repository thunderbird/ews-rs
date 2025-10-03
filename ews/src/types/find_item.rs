use ews_proc_macros::operation_response;
use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{BaseFolderId, ItemShape, Items, MESSAGES_NS_URI};

/// Defines a request to find items in mailbox
///
/// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
#[operation_response(FindItemResponseMessage)]
pub struct FindItem {
    #[xml_struct(attribute)]
    pub traversal: Traversal,
    pub item_shape: ItemShape,
    pub parent_folder_ids: Vec<BaseFolderId>,
}

/// Defines whether the search finds items in folders or the folders' dumpsters.
/// This attribute is required
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(text)]
pub enum Traversal {
    Shallow,
    SoftDeleted,
    Associated,
}

/// Contains the status and result of a single [`FindItem`] operation request
///
/// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditemresponsemessage
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
    fn test_serialize_find_item() {
        let expected = r#"<FindItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages" Traversal="Shallow"><ItemShape><t:BaseShape>IdOnly</t:BaseShape></ItemShape><ParentFolderIds><t:DistinguishedFolderId Id="deleteditems"/></ParentFolderIds></FindItem>"#;

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
        };

        assert_serialized_content(&find_item, "FindItem", expected);
    }
}
