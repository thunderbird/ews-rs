/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use xml_struct::XmlSerialize;

use super::{BaseFolderId, BaseItemId, MESSAGES_NS_URI};

#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
pub struct MoveItem {
    pub to_folder_id: BaseFolderId,
    pub item_ids: Vec<BaseItemId>,
}

#[cfg(test)]
mod test {
    use crate::{test_support::assert_serialized_content, BaseFolderId, BaseItemId};

    use super::MoveItem;

    #[test]
    fn test_serialize_move_item() {
        let move_item = MoveItem {
            to_folder_id: BaseFolderId::DistinguishedFolderId {
                id: "drafts".to_string(),
                change_key: None,
            },
            item_ids: vec![BaseItemId::ItemId {
                id: "AAAtAEF/swbAAA=".to_string(),
                change_key: Some("EwAAABYA/s4b".to_string()),
            }],
        };

        let expected = r#"<MoveItem xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"><ToFolderId><t:DistinguishedFolderId Id="drafts"/></ToFolderId><ItemIds><t:ItemId Id="AAAtAEF/swbAAA=" ChangeKey="EwAAABYA/s4b"/></ItemIds></MoveItem>"#;

        assert_serialized_content(&move_item, "MoveItem", expected);
    }
}
