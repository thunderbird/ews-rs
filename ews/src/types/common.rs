/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::ops::{Deref, DerefMut};

use serde::{de, Deserialize, Deserializer};
use time::format_description::well_known::Iso8601;
use time::{OffsetDateTime, PrimitiveDateTime};
use xml_struct::XmlSerialize;

pub mod response;
pub use self::response::{ResponseClass, ResponseMessages};
pub mod message_xml;
pub use self::message_xml::MessageXml;

pub(crate) const MESSAGES_NS_URI: &str =
    "http://schemas.microsoft.com/exchange/services/2006/messages";
pub(crate) const SOAP_NS_URI: &str = "http://schemas.xmlsoap.org/soap/envelope/";
pub(crate) const TYPES_NS_URI: &str = "http://schemas.microsoft.com/exchange/services/2006/types";

/// The folder properties which should be included in the response.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/foldershape>.
#[derive(Clone, Debug, Default, XmlSerialize)]
pub struct FolderShape {
    #[xml_struct(ns_prefix = "t")]
    pub base_shape: BaseShape,
}

/// The item properties which should be included in the response.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemshape>.
#[derive(Clone, Debug, Default, XmlSerialize)]
pub struct ItemShape {
    /// The base set of properties to include, which may be extended by other
    /// fields.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape>
    #[xml_struct(ns_prefix = "t")]
    pub base_shape: BaseShape,

    /// Whether the MIME content of an item should be included.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/includemimecontent>
    #[xml_struct(ns_prefix = "t")]
    pub include_mime_content: Option<bool>,

    /// A list of properties which should be included in addition to those
    /// implied by other fields.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/additionalproperties>
    #[xml_struct(ns_prefix = "t")]
    pub additional_properties: Option<Vec<PathToElement>>,
}

/// An identifier for a property on an Exchange entity.
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(variant_ns_prefix = "t")]
pub enum PathToElement {
    /// An identifier for an extended MAPI property.
    ///
    /// The full set of constraints on which properties may or must be set
    /// together are not expressed in the structure of this variant. Please see
    /// Microsoft's documentation for further details.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/extendedfielduri>
    // TODO: We can represent in a friendlier way with an enum, probably. A
    // property is fully specified by a type and either:
    // - A property set ID plus property name/ID, or
    // - A property tag.
    // https://github.com/thunderbird/ews-rs/issues/9
    ExtendedFieldURI {
        /// A well-known identifier for a property set.
        #[xml_struct(attribute)]
        distinguished_property_set_id: Option<DistinguishedPropertySet>,

        /// A GUID representing a property set.
        // TODO: This could use a strong type for representing a GUID.
        #[xml_struct(attribute)]
        property_set_id: Option<String>,

        /// Specifies a property by integer tag.
        // TODO: This should use an integer type, but it seems a hex
        // representation is preferred, and we should restrict the possible
        // values per the docs.
        #[xml_struct(attribute)]
        property_tag: Option<String>,

        /// The name of a property within a specified property set.
        #[xml_struct(attribute)]
        property_name: Option<String>,

        /// The dispatch ID of a property within a specified property set.
        #[xml_struct(attribute)]
        property_id: Option<String>,

        /// The value type of the desired property.
        #[xml_struct(attribute)]
        property_type: PropertyType,
    },

    /// An identifier for a property given by a well-known string.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/fielduri>
    #[allow(non_snake_case)]
    FieldURI {
        /// The well-known string.
        // TODO: Adjust xml_struct to support field renaming to avoid non-snake
        // case identifiers.
        // https://github.com/thunderbird/xml-struct-rs/issues/6
        // TODO: We could use an enum for this field. It's just large and not
        // worth typing out by hand.
        #[xml_struct(attribute)]
        field_URI: String,
    },

    /// An identifier for a specific element of a dictionary-based property.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/indexedfielduri>
    #[allow(non_snake_case)]
    IndexedFieldURI {
        /// The well-known string identifier of the property.
        #[xml_struct(attribute)]
        field_URI: String,

        /// The member within the dictionary to access.
        #[xml_struct(attribute)]
        field_index: String,
    },
}

/// The identifier for an extended MAPI property.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/extendedfielduri>
// N.B.: This is copied from `PathToElement::ExtendedFieldURI`,
// which follows the same structure. However, xml-struct doesn't currently
// support using a nested structure to define an element's attributes, see
// https://github.com/thunderbird/xml-struct-rs/issues/9
#[derive(Clone, Debug, Deserialize, XmlSerialize, Eq, PartialEq)]
pub struct ExtendedFieldURI {
    /// A well-known identifier for a property set.
    #[xml_struct(attribute)]
    #[serde(rename = "@DistinguishedPropertySetId")]
    pub distinguished_property_set_id: Option<DistinguishedPropertySet>,

    /// A GUID representing a property set.
    // TODO: This could use a strong type for representing a GUID.
    #[xml_struct(attribute)]
    #[serde(rename = "@PropertySetId")]
    pub property_set_id: Option<String>,

    /// Specifies a property by integer tag.
    #[xml_struct(attribute)]
    #[serde(rename = "@PropertyTag")]
    pub property_tag: Option<String>,

    /// The name of a property within a specified property set.
    #[xml_struct(attribute)]
    #[serde(rename = "@PropertyName")]
    pub property_name: Option<String>,

    /// The dispatch ID of a property within a specified property set.
    #[xml_struct(attribute)]
    #[serde(rename = "@PropertyId")]
    pub property_id: Option<String>,

    /// The value type of the desired property.
    // TODO: This is a *required* field in the ms docs, but we seem to be receiving XML without it?
    #[xml_struct(attribute)]
    #[serde(rename = "@PropertyType")]
    pub property_type: Option<PropertyType>,
}

/// A well-known MAPI property set identifier.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/extendedfielduri#distinguishedpropertysetid-attribute>
#[derive(Clone, Copy, Debug, Deserialize, XmlSerialize, Eq, PartialEq)]
#[xml_struct(text)]
pub enum DistinguishedPropertySet {
    Address,
    Appointment,
    CalendarAssistant,
    Common,
    InternetHeaders,
    Meeting,
    PublicStrings,
    Sharing,
    Task,
    UnifiedMessaging,
}

/// The action an Exchange server will take upon creating a `Message` item.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/createitem#messagedisposition-attribute>
/// and <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/updateitem#messagedisposition-attribute>
#[derive(Clone, Copy, Debug, XmlSerialize)]
#[xml_struct(text)]
pub enum MessageDisposition {
    SaveOnly,
    SendOnly,
    SendAndSaveCopy,
}

/// The type of the value of a MAPI property.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/extendedfielduri#propertytype-attribute>
#[derive(Clone, Copy, Debug, Deserialize, XmlSerialize, Eq, PartialEq)]
#[xml_struct(text)]
pub enum PropertyType {
    ApplicationTime,
    ApplicationTimeArray,
    Binary,
    BinaryArray,
    Boolean,
    CLSID,
    CLSIDArray,
    Currency,
    CurrencyArray,
    Double,
    DoubleArray,
    Float,
    FloatArray,
    Integer,
    IntegerArray,
    Long,
    LongArray,
    Short,
    ShortArray,
    SystemTime,
    SystemTimeArray,
    String,
    StringArray,
}

/// The base set of properties to be returned in response to our request.
/// Additional properties may be specified by the parent element.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/baseshape>.
#[derive(Clone, Copy, Debug, Default, XmlSerialize)]
#[xml_struct(text)]
pub enum BaseShape {
    /// Only the IDs of any items or folders returned.
    IdOnly,

    /// The default set of properties for the relevant item or folder.
    ///
    /// The properties returned are dependent on the type of item or folder. See
    /// the EWS documentation for details.
    #[default]
    Default,

    /// All properties of an item or folder.
    AllProperties,
}

/// The common format for item move and copy operations.
#[derive(Clone, Debug, XmlSerialize)]
pub struct CopyMoveItemData {
    /// The destination folder for the copied/moved item.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/tofolderid>
    pub to_folder_id: BaseFolderId,
    /// The unique identifiers for each item to copy/move.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemids>
    pub item_ids: Vec<BaseItemId>,
    /// Whether or not to return the new item idententifers in the response.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/returnnewitemids>
    pub return_new_item_ids: Option<bool>,
}

/// The common format for folder move and copy operations.
#[derive(Clone, Debug, XmlSerialize)]
pub struct CopyMoveFolderData {
    /// The destination folder for the copied/moved folder.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/tofolderid>
    pub to_folder_id: BaseFolderId,

    /// The identifiers for each folder to copy/move.
    ///
    /// <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderids>
    pub folder_ids: Vec<BaseFolderId>,
}

/// The common format of folder response messages.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct FolderResponseMessage {
    pub folders: Folders,
}

/// The common format of item response messages.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ItemResponseMessage {
    pub items: Items,
}

/// An identifier for an Exchange folder.
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(variant_ns_prefix = "t")]
pub enum BaseFolderId {
    /// An identifier for an arbitrary folder.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid>.
    FolderId {
        #[xml_struct(attribute)]
        id: String,

        #[xml_struct(attribute)]
        change_key: Option<String>,
    },

    /// An identifier for referencing a folder by name, e.g. "inbox" or
    /// "junkemail".
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/distinguishedfolderid>.
    DistinguishedFolderId {
        #[xml_struct(attribute)]
        id: String,

        #[xml_struct(attribute)]
        change_key: Option<String>,

        /// The mailbox that owns this distinguished folder.
        ///
        /// Required when referencing a distinguished folder in a mailbox
        /// other than the one associated with the account making the
        /// request, e.g. a shared or resource mailbox.
        #[xml_struct(ns_prefix = "t")]
        mailbox: Option<Mailbox>,
    },
}

impl BaseFolderId {
    /// Creates a [`BaseFolderId::FolderId`] referencing an arbitrary folder
    /// by its identifier.
    pub fn new_folder(id: impl Into<String>) -> Self {
        BaseFolderId::FolderId {
            id: id.into(),
            change_key: None,
        }
    }

    /// Creates a [`BaseFolderId::DistinguishedFolderId`] referencing a
    /// well-known folder (e.g. `"calendar"` or `"inbox"`) in the requesting
    /// account's own mailbox.
    pub fn new_distinguished(id: impl Into<String>) -> Self {
        BaseFolderId::DistinguishedFolderId {
            id: id.into(),
            change_key: None,
            mailbox: None,
        }
    }

    /// Creates a [`BaseFolderId::DistinguishedFolderId`] referencing a
    /// well-known folder in another mailbox, e.g. a shared or resource
    /// mailbox, identified by `mailbox`.
    pub fn new_distinguished_in_mailbox(id: impl Into<String>, mailbox: Mailbox) -> Self {
        BaseFolderId::DistinguishedFolderId {
            id: id.into(),
            change_key: None,
            mailbox: Some(mailbox),
        }
    }
}

/// The unique identifier of a folder.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folderid>
#[derive(Clone, Debug, Deserialize, PartialEq, XmlSerialize, Eq)]
pub struct FolderId {
    #[serde(rename = "@Id")]
    pub id: String,

    #[serde(rename = "@ChangeKey")]
    pub change_key: Option<String>,
}

/// The manner in which items or folders are deleted.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deletetype>
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(text)]
pub enum DeleteType {
    HardDelete,
    MoveToDeletedItems,
    SoftDelete,
}

/// An identifier for an Exchange item.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemids>
// N.B.: Commented-out variants are not yet implemented.
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(variant_ns_prefix = "t")]
pub enum BaseItemId {
    /// An identifier for a standard Exchange item.
    ItemId {
        #[xml_struct(attribute)]
        id: String,

        #[xml_struct(attribute)]
        change_key: Option<String>,
    },
    // OccurrenceItemId { .. }
    /// The series an occurrence belongs to, addressed through the occurrence's
    /// own id.
    ///
    /// What makes editing or deleting a whole series possible from an
    /// occurrence in hand: the occurrence ids a `CalendarView` returns cannot
    /// reach the master any other way.
    RecurringMasterItemId {
        #[xml_struct(attribute)]
        occurrence_id: String,

        #[xml_struct(attribute)]
        change_key: Option<String>,
    },
}

/// The unique identifier of an item.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemid>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct ItemId {
    #[xml_struct(attribute)]
    #[serde(rename = "@Id")]
    pub id: String,

    #[serde(rename = "@ChangeKey")]
    #[xml_struct(attribute)]
    pub change_key: Option<String>,
}

/// The representation of a folder in an EWS operation.
#[derive(Clone, Debug, Deserialize, XmlSerialize, Eq, PartialEq)]
#[xml_struct(variant_ns_prefix = "t")]
pub enum Folder {
    /// A calendar folder in a mailbox.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/calendarfolder>
    #[serde(rename_all = "PascalCase")]
    CalendarFolder {
        #[xml_struct(ns_prefix = "t")]
        folder_id: Option<FolderId>,

        #[xml_struct(ns_prefix = "t")]
        parent_folder_id: Option<FolderId>,

        #[xml_struct(ns_prefix = "t")]
        folder_class: Option<String>,

        #[xml_struct(ns_prefix = "t")]
        display_name: Option<String>,

        #[xml_struct(ns_prefix = "t")]
        total_count: Option<u32>,

        #[xml_struct(ns_prefix = "t")]
        child_folder_count: Option<u32>,

        #[xml_struct(ns_prefix = "t")]
        extended_property: Option<Vec<ExtendedProperty>>,
    },

    /// A contacts folder in a mailbox.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contactsfolder>
    #[serde(rename_all = "PascalCase")]
    ContactsFolder {
        #[xml_struct(ns_prefix = "t")]
        folder_id: Option<FolderId>,

        #[xml_struct(ns_prefix = "t")]
        parent_folder_id: Option<FolderId>,

        #[xml_struct(ns_prefix = "t")]
        folder_class: Option<String>,

        #[xml_struct(ns_prefix = "t")]
        display_name: Option<String>,

        #[xml_struct(ns_prefix = "t")]
        total_count: Option<u32>,

        #[xml_struct(ns_prefix = "t")]
        child_folder_count: Option<u32>,

        #[xml_struct(ns_prefix = "t")]
        extended_property: Option<Vec<ExtendedProperty>>,
    },

    /// A folder in a mailbox.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folder>
    #[serde(rename_all = "PascalCase")]
    Folder {
        #[xml_struct(ns_prefix = "t")]
        folder_id: Option<FolderId>,

        #[xml_struct(ns_prefix = "t")]
        parent_folder_id: Option<FolderId>,

        #[xml_struct(ns_prefix = "t")]
        folder_class: Option<String>,

        #[xml_struct(ns_prefix = "t")]
        display_name: Option<String>,

        #[xml_struct(ns_prefix = "t")]
        total_count: Option<u32>,

        #[xml_struct(ns_prefix = "t")]
        child_folder_count: Option<u32>,

        #[xml_struct(ns_prefix = "t")]
        extended_property: Option<Vec<ExtendedProperty>>,

        #[xml_struct(ns_prefix = "t")]
        unread_count: Option<u32>,
    },

    /// A search folder in a mailbox.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/searchfolder>
    #[serde(rename_all = "PascalCase")]
    SearchFolder {
        #[xml_struct(ns_prefix = "t")]
        folder_id: Option<FolderId>,

        #[xml_struct(ns_prefix = "t")]
        parent_folder_id: Option<FolderId>,

        #[xml_struct(ns_prefix = "t")]
        folder_class: Option<String>,

        #[xml_struct(ns_prefix = "t")]
        display_name: Option<String>,

        #[xml_struct(ns_prefix = "t")]
        total_count: Option<u32>,

        #[xml_struct(ns_prefix = "t")]
        child_folder_count: Option<u32>,

        #[xml_struct(ns_prefix = "t")]
        extended_property: Option<Vec<ExtendedProperty>>,
    },

    /// A task folder in a mailbox.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/tasksfolder>
    #[serde(rename_all = "PascalCase")]
    TasksFolder {
        #[xml_struct(ns_prefix = "t")]
        folder_id: Option<FolderId>,

        #[xml_struct(ns_prefix = "t")]
        parent_folder_id: Option<FolderId>,

        #[xml_struct(ns_prefix = "t")]
        folder_class: Option<String>,

        #[xml_struct(ns_prefix = "t")]
        display_name: Option<String>,

        #[xml_struct(ns_prefix = "t")]
        total_count: Option<u32>,

        #[xml_struct(ns_prefix = "t")]
        child_folder_count: Option<u32>,

        #[xml_struct(ns_prefix = "t")]
        extended_property: Option<Vec<ExtendedProperty>>,
    },
}

/// An array of items.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Items {
    #[serde(rename = "$value", default)]
    pub inner: Vec<RealItem>,
}

/// A collection of information on Exchange folders.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/folders-ex15websvcsotherref>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Folders {
    #[serde(rename = "$value", default)]
    pub inner: Vec<Folder>,
}

/// An item which may appear as the result of a request to read or modify an
/// Exchange item.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/items>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[xml_struct(variant_ns_prefix = "t")]
#[non_exhaustive]
pub enum RealItem {
    Item(Message),
    Message(Message),
    CalendarItem(Message),
    /// A reply to a meeting request, accepting it. Created rather than
    /// updated: the answer is a message sent to the organizer, and the
    /// server's own bookkeeping follows from it.
    AcceptItem(Message),
    /// The same, answering "maybe".
    TentativelyAcceptItem(Message),
    /// The same, declining.
    DeclineItem(Message),
    Contact(Message),
    DistributionList(Message),
    MeetingMessage(Message),
    MeetingRequest(Message),
    MeetingResponse(Message),
    MeetingCancellation(Message),
    Task(Message),
    PostItem(Message),
}

impl RealItem {
    /// Return the [`Message`] object contained within this [`RealItem`].
    pub fn inner_message(&self) -> &Message {
        use RealItem::*;
        match self {
            Item(message)
            | Message(message)
            | CalendarItem(message)
            | AcceptItem(message)
            | TentativelyAcceptItem(message)
            | DeclineItem(message)
            | Contact(message)
            | DistributionList(message)
            | MeetingMessage(message)
            | MeetingRequest(message)
            | MeetingResponse(message)
            | MeetingCancellation(message)
            | Task(message)
            | PostItem(message) => message,
        }
    }

    /// Take ownership of the inner [`Message`].
    pub fn into_inner_message(self) -> Message {
        use RealItem::*;
        match self {
            Item(message)
            | Message(message)
            | CalendarItem(message)
            | AcceptItem(message)
            | TentativelyAcceptItem(message)
            | DeclineItem(message)
            | Contact(message)
            | DistributionList(message)
            | MeetingMessage(message)
            | MeetingRequest(message)
            | MeetingResponse(message)
            | MeetingCancellation(message)
            | Task(message)
            | PostItem(message) => message,
        }
    }
}

/// A date and time with second precision.
// `time` provides an `Option<OffsetDateTime>` deserializer, but it does not
// work with map fields which may be omitted, as in our case.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateTime(pub OffsetDateTime);

// While the docs say "Exchange will always include a time zone (either UTC or a
// specific time zone) in the value", there appears to be a bug where
// attachments will sometimes return a `LastModifiedTime` without a timezone.
// This custom deserializer adds a fallback to parse such strings as UTC.
impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if let Ok(date_time) = OffsetDateTime::parse(&value, &Iso8601::DEFAULT) {
            Ok(Self(date_time))
        } else {
            PrimitiveDateTime::parse(&value, &Iso8601::DEFAULT)
                .map(|date_time| Self(date_time.assume_utc()))
                .map_err(de::Error::custom)
        }
    }
}

impl XmlSerialize for DateTime {
    /// Serializes a `DateTime` as an XML text content node by formatting the
    /// inner [`time::OffsetDateTime`] as an ISO 8601-compliant string.
    fn serialize_child_nodes<W>(
        &self,
        writer: &mut quick_xml::Writer<W>,
    ) -> Result<(), xml_struct::Error>
    where
        W: std::io::Write,
    {
        let time = self
            .0
            .format(&Iso8601::DEFAULT)
            .map_err(|err| xml_struct::Error::Value(err.into()))?;

        time.serialize_child_nodes(writer)
    }
}

/// An email message.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/message-ex15websvcsotherref>
#[derive(Clone, Debug, Default, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Message {
    /// The MIME content of the item.
    #[xml_struct(ns_prefix = "t")]
    pub mime_content: Option<MimeContent>,

    /// The item's Exchange identifier.
    #[xml_struct(ns_prefix = "t")]
    pub item_id: Option<ItemId>,

    /// The identifier for the containing folder.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/parentfolderid>
    #[xml_struct(ns_prefix = "t")]
    pub parent_folder_id: Option<FolderId>,

    /// The Exchange class value of the item.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemclass>
    #[xml_struct(ns_prefix = "t")]
    pub item_class: Option<String>,

    /// The subject of the item.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/subject>
    #[xml_struct(ns_prefix = "t")]
    pub subject: Option<String>,

    #[xml_struct(ns_prefix = "t")]
    pub sensitivity: Option<Sensitivity>,

    #[xml_struct(ns_prefix = "t")]
    pub body: Option<Body>,

    #[xml_struct(ns_prefix = "t")]
    pub attachments: Option<Attachments>,

    #[xml_struct(ns_prefix = "t")]
    pub date_time_received: Option<DateTime>,

    #[xml_struct(ns_prefix = "t")]
    pub size: Option<usize>,

    /// A list of categories describing an item.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/categories-ex15websvcsotherref>
    #[xml_struct(ns_prefix = "t")]
    pub categories: Option<Vec<StringElement>>,

    // Extended MAPI properties of the message.
    #[xml_struct(ns_prefix = "t")]
    pub extended_property: Option<Vec<ExtendedProperty>>,

    #[xml_struct(ns_prefix = "t")]
    pub importance: Option<Importance>,

    #[xml_struct(ns_prefix = "t")]
    pub in_reply_to: Option<String>,

    #[xml_struct(ns_prefix = "t")]
    pub is_submitted: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub is_draft: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub is_from_me: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub is_resend: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub is_unmodified: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub internet_message_headers: Option<InternetMessageHeaders>,

    #[xml_struct(ns_prefix = "t")]
    pub date_time_sent: Option<DateTime>,

    #[xml_struct(ns_prefix = "t")]
    pub date_time_created: Option<DateTime>,

    #[xml_struct(ns_prefix = "t")]
    pub reminder_due_by: Option<DateTime>,

    #[xml_struct(ns_prefix = "t")]
    pub reminder_is_set: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub reminder_minutes_before_start: Option<usize>,

    #[xml_struct(ns_prefix = "t")]
    pub display_cc: Option<String>,

    #[xml_struct(ns_prefix = "t")]
    pub display_to: Option<String>,

    #[xml_struct(ns_prefix = "t")]
    pub has_attachments: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub culture: Option<String>,

    #[xml_struct(ns_prefix = "t")]
    pub sender: Option<Recipient>,

    #[xml_struct(ns_prefix = "t")]
    pub to_recipients: Option<ArrayOfRecipients>,

    #[xml_struct(ns_prefix = "t")]
    pub cc_recipients: Option<ArrayOfRecipients>,

    #[xml_struct(ns_prefix = "t")]
    pub bcc_recipients: Option<ArrayOfRecipients>,

    #[xml_struct(ns_prefix = "t")]
    pub is_read_receipt_requested: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub is_delivery_receipt_requested: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub conversation_index: Option<String>,

    #[xml_struct(ns_prefix = "t")]
    pub conversation_topic: Option<String>,

    #[xml_struct(ns_prefix = "t")]
    pub from: Option<Recipient>,

    #[xml_struct(ns_prefix = "t")]
    pub internet_message_id: Option<String>,

    #[xml_struct(ns_prefix = "t")]
    pub is_read: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub is_response_requested: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub reply_to: Option<ArrayOfRecipients>,

    #[xml_struct(ns_prefix = "t")]
    pub received_by: Option<Recipient>,

    #[xml_struct(ns_prefix = "t")]
    pub received_representing: Option<Recipient>,

    #[xml_struct(ns_prefix = "t")]
    pub last_modified_name: Option<String>,

    #[xml_struct(ns_prefix = "t")]
    pub last_modified_time: Option<DateTime>,

    #[xml_struct(ns_prefix = "t")]
    pub is_associated: Option<bool>,

    #[xml_struct(ns_prefix = "t")]
    pub conversation_id: Option<ItemId>,

    #[xml_struct(ns_prefix = "t")]
    pub references: Option<String>,

    /// A short preview of the first 256 characters of an item.
    ///
    /// This value isn't documented for either the `Item` or `Message`
    /// interfaces in the Exchange documentation. However, the element itself is
    /// documented at
    /// <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/preview-ex15websvcsotherref>
    ///
    /// This element was introduced in Exchange 2013.
    #[xml_struct(ns_prefix = "t")]
    pub preview: Option<String>,

    /// The flag status of the mailbox item.
    ///
    /// This value isn't documented for either the `Item` or `Message`
    /// interfaces in the Exchange documentation. However, the element itself is
    /// documented at
    /// <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/flag>
    ///
    /// This element was introduced in Exchange 2013.
    #[xml_struct(ns_prefix = "t")]
    pub flag: Option<Flag>,

    // ---- Calendar item properties -----------------------------------------
    //
    // `RealItem` represents every item type with this one struct, so the
    // properties an appointment adds to an item live here too. They are
    // populated only for `RealItem::CalendarItem` (and for meeting requests,
    // which carry the proposed appointment), and are `None` on mail.
    //
    // See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/calendaritem>
    /// The start of the appointment.
    #[xml_struct(ns_prefix = "t")]
    pub start: Option<DateTime>,

    /// The end of the appointment.
    #[xml_struct(ns_prefix = "t")]
    pub end: Option<DateTime>,

    /// Whether the appointment covers whole days, in which case `start` and
    /// `end` are midnight boundaries rather than times of day.
    #[xml_struct(ns_prefix = "t")]
    pub is_all_day_event: Option<bool>,

    /// How the appointment shows on the organizer's free/busy calendar.
    #[xml_struct(ns_prefix = "t")]
    pub legacy_free_busy_status: Option<LegacyFreeBusyStatus>,

    /// Where the appointment takes place, as free text.
    #[xml_struct(ns_prefix = "t")]
    pub location: Option<String>,

    /// Whether this item is one occurrence of a recurring series.
    ///
    /// A `CalendarView` returns occurrences already expanded from their
    /// series, so a client reading a date range does not have to interpret
    /// recurrence rules itself.
    #[xml_struct(ns_prefix = "t")]
    pub is_recurring: Option<bool>,

    /// The iCalendar UID of the appointment.
    ///
    /// Every occurrence expanded from one recurring series carries the same
    /// value, which is what lets a client group occurrences back into the
    /// series they came from without interpreting recurrence rules. Item ids,
    /// by contrast, differ per occurrence.
    // Named explicitly: the struct's PascalCase rule would look for `Uid`,
    // while the element Exchange sends is `UID`.
    #[serde(rename = "UID")]
    #[xml_struct(ns_prefix = "t")]
    pub uid: Option<String>,

    /// The meeting request a response object answers.
    ///
    /// Set only on `AcceptItem` and its siblings, which carry nothing else:
    /// the response is defined entirely by which item it refers to and which
    /// of the three it is.
    #[xml_struct(ns_prefix = "t")]
    pub reference_item_id: Option<ReferenceItemId>,

    /// The rule by which the appointment repeats, when it does.
    ///
    /// Written when creating a series; a `CalendarView` never needs it, since
    /// the server expands the series into occurrences for the range asked for.
    #[xml_struct(ns_prefix = "t")]
    pub recurrence: Option<Recurrence>,

    /// Whether the meeting has been cancelled by its organizer.
    #[xml_struct(ns_prefix = "t")]
    pub is_cancelled: Option<bool>,

    /// Whether the appointment is a meeting, i.e. has attendees.
    #[xml_struct(ns_prefix = "t")]
    pub is_meeting: Option<bool>,

    /// The mailbox that organizes the meeting.
    #[xml_struct(ns_prefix = "t")]
    pub organizer: Option<Recipient>,

    /// Attendees whose presence the organizer expects.
    #[xml_struct(ns_prefix = "t")]
    pub required_attendees: Option<ArrayOfAttendees>,

    /// Attendees invited without their presence being expected.
    #[xml_struct(ns_prefix = "t")]
    pub optional_attendees: Option<ArrayOfAttendees>,

    /// This mailbox's own response to the meeting invitation.
    #[xml_struct(ns_prefix = "t")]
    pub my_response_type: Option<ResponseType>,
}

/// How an appointment shows on a calendar's free/busy view.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/legacyfreebusystatus>
#[derive(Clone, Copy, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub enum LegacyFreeBusyStatus {
    Free,
    Tentative,
    Busy,
    #[serde(rename = "OOF")]
    OOF,
    WorkingElsewhere,
    NoData,
}

/// A mailbox's answer to a meeting invitation.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/responsetype>
#[derive(Clone, Copy, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub enum ResponseType {
    Unknown,
    Organizer,
    Tentative,
    Accept,
    Decline,
    NoResponseReceived,
}

/// A list of meeting attendees.
///
/// The XML nests each entry in an `<t:Attendee>` element, so the sequence is
/// unwrapped the same way [`ArrayOfRecipients`] unwraps its mailboxes.
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct ArrayOfAttendees(
    #[serde(deserialize_with = "deserialize_attendees")]
    #[xml_struct(flatten)]
    pub Vec<Attendee>,
);

impl Deref for ArrayOfAttendees {
    type Target = Vec<Attendee>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn deserialize_attendees<'de, D>(deserializer: D) -> Result<Vec<Attendee>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct AttendeeSequence {
        attendee: Vec<Attendee>,
    }

    let seq = AttendeeSequence::deserialize(deserializer)?;

    Ok(seq.attendee)
}

/// One invitee of a meeting, and how they answered.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/attendee>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Attendee {
    #[xml_struct(ns_prefix = "t")]
    pub mailbox: Mailbox,

    /// How this attendee answered the invitation.
    #[xml_struct(ns_prefix = "t")]
    pub response_type: Option<ResponseType>,

    /// When they answered.
    #[xml_struct(ns_prefix = "t")]
    pub last_response_time: Option<DateTime>,
}

/// An extended MAPI property of an Exchange item or folder.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/extendedproperty>
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, XmlSerialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ExtendedProperty {
    #[xml_struct(ns_prefix = "t")]
    #[serde(rename = "ExtendedFieldURI")]
    pub extended_field_URI: ExtendedFieldURI,

    #[xml_struct(ns_prefix = "t")]
    pub value: String,
}

/// A list of attachments.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/attachments-ex15websvcsotherref>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct Attachments {
    #[serde(rename = "$value")]
    #[xml_struct(flatten)]
    pub inner: Vec<Attachment>,
}

/// A newtype around a vector of `Recipient`s, that is deserialized using
/// `deserialize_recipients`.
#[derive(Clone, Debug, Default, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct ArrayOfRecipients(
    #[serde(deserialize_with = "deserialize_recipients")] pub Vec<Recipient>,
);

impl Deref for ArrayOfRecipients {
    type Target = Vec<Recipient>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ArrayOfRecipients {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A single mailbox.
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Recipient {
    #[xml_struct(ns_prefix = "t")]
    pub mailbox: Mailbox,
}

/// Deserializes a list of recipients.
///
/// `quick-xml`'s `serde` implementation requires the presence of an
/// intermediate type when dealing with lists, and this is not compatible with
/// our model for serialization.
///
/// We could directly deserialize into a `Vec<Mailbox>`, which would also
/// simplify this function a bit, but this would mean using different models
/// to represent single vs. multiple recipient(s).
fn deserialize_recipients<'de, D>(deserializer: D) -> Result<Vec<Recipient>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct MailboxSequence {
        mailbox: Vec<Mailbox>,
    }

    let seq = MailboxSequence::deserialize(deserializer)?;

    Ok(seq
        .mailbox
        .into_iter()
        .map(|mailbox| Recipient { mailbox })
        .collect())
}

/// A list of Internet Message Format headers.
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct InternetMessageHeaders {
    pub internet_message_header: Vec<InternetMessageHeader>,
}

/// A reference to a user or address which can send or receive mail.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailbox>
#[derive(Clone, Debug, Default, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Mailbox {
    /// The name of this mailbox's user.
    #[xml_struct(ns_prefix = "t")]
    pub name: Option<String>,

    /// The email address for this mailbox. This can be [`None`] in some cases,
    /// e.g. if it designates an automated system account (see
    /// <https://bugzilla.mozilla.org/show_bug.cgi?id=1994719> for an example).
    #[xml_struct(ns_prefix = "t")]
    pub email_address: Option<String>,

    /// The protocol used in routing to this mailbox.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/routingtype-emailaddress>
    ///
    /// Note: Although the documentation says that `SMTP` and `EX` are the only
    /// possible values, it also appears that `SYSTEM` is a value that sometimes
    /// occurs. Since the documentation isn't clear, this is a free-form string
    /// field.
    pub routing_type: Option<String>,

    /// The type of sender/recipient represented by this mailbox.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailboxtype>
    pub mailbox_type: Option<MailboxType>,

    /// An identifier for a contact or list of contacts corresponding to this
    /// mailbox.
    pub item_id: Option<ItemId>,
}

/// The type of sender or recipient a mailbox represents.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailboxtype>
#[derive(Clone, Copy, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[xml_struct(text)]
pub enum MailboxType {
    Mailbox,
    PublicDL,
    PrivateDL,
    Contact,
    PublicFolder,
    Unknown,
    OneOff,
    GroupMailbox,
}

/// The priority level of an item.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/importance>
#[derive(Clone, Copy, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[xml_struct(text)]
pub enum Importance {
    Low,
    Normal,
    High,
}

/// A string value.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/string>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct StringElement {
    /// The string content.
    pub string: String,
}

/// The sensitivity of the contents of an item.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/sensitivity>
#[derive(Clone, Copy, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[xml_struct(text)]
pub enum Sensitivity {
    Normal,
    Personal,
    Private,
    Confidential,
}

/// The body of an item.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/body>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct Body {
    /// The content type of the body.
    #[serde(rename = "@BodyType")]
    #[xml_struct(attribute)]
    pub body_type: BodyType,

    /// Whether the body has been truncated.
    #[serde(rename = "@IsTruncated")]
    #[xml_struct(attribute)]
    pub is_truncated: Option<bool>,

    /// The content of the body.
    // TODO: It's not immediately obvious why this tag may be empty, but it has
    // been encountered in real world responses. Needs a closer look.
    #[serde(rename = "$text")]
    #[xml_struct(flatten)]
    pub content: Option<String>,
}

/// The item a response object refers to.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/referenceitemid>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct ReferenceItemId {
    #[serde(rename = "@Id")]
    #[xml_struct(attribute)]
    pub id: String,

    #[serde(rename = "@ChangeKey")]
    #[xml_struct(attribute)]
    pub change_key: Option<String>,
}

/// How an appointment repeats: a pattern saying when, and a range saying for
/// how long.
///
/// The element requires exactly one of each, in that order, and the server
/// rejects the request outright if they arrive the other way round.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/recurrence-recurrencetype>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct Recurrence {
    pub pattern: RecurrencePattern,
    pub range: RecurrenceRange,
}

/// When an appointment repeats.
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub enum RecurrencePattern {
    /// Every N days.
    #[xml_struct(ns_prefix = "t")]
    DailyRecurrence {
        #[xml_struct(ns_prefix = "t")]
        interval: u16,
    },

    /// Every N weeks, on the named days.
    #[xml_struct(ns_prefix = "t")]
    WeeklyRecurrence {
        #[xml_struct(ns_prefix = "t")]
        interval: u16,

        /// A space-separated list of day names, e.g. "Monday Wednesday".
        #[xml_struct(ns_prefix = "t")]
        days_of_week: String,
    },

    /// Every N months, on a fixed day of the month.
    #[xml_struct(ns_prefix = "t")]
    AbsoluteMonthlyRecurrence {
        #[xml_struct(ns_prefix = "t")]
        interval: u16,

        /// The day of the month, 1 to 31.
        #[xml_struct(ns_prefix = "t")]
        day_of_month: u8,
    },

    /// Every year, on a fixed day of a fixed month.
    #[xml_struct(ns_prefix = "t")]
    AbsoluteYearlyRecurrence {
        #[xml_struct(ns_prefix = "t")]
        day_of_month: u8,

        /// The month's name, e.g. "September".
        #[xml_struct(ns_prefix = "t")]
        month: String,
    },
}

/// For how long an appointment repeats.
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub enum RecurrenceRange {
    /// Forever.
    #[xml_struct(ns_prefix = "t")]
    NoEndRecurrence {
        /// The date the series starts, as `YYYY-MM-DD`.
        #[xml_struct(ns_prefix = "t")]
        start_date: String,
    },

    /// Until a date, inclusive.
    #[xml_struct(ns_prefix = "t")]
    EndDateRecurrence {
        #[xml_struct(ns_prefix = "t")]
        start_date: String,

        #[xml_struct(ns_prefix = "t")]
        end_date: String,
    },

    /// A fixed number of occurrences.
    #[xml_struct(ns_prefix = "t")]
    NumberedRecurrence {
        #[xml_struct(ns_prefix = "t")]
        start_date: String,

        #[xml_struct(ns_prefix = "t")]
        number_of_occurrences: u16,
    },
}

/// The content type of an item's body.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/body>
#[derive(Clone, Copy, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[xml_struct(text)]
pub enum BodyType {
    HTML,
    Text,
}

/// An attachment to an Exchange item.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/attachments-ex15websvcsotherref>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub enum Attachment {
    /// An attachment containing an Exchange item.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/itemattachment>
    #[serde(rename_all = "PascalCase")]
    ItemAttachment {
        /// An identifier for the attachment.
        attachment_id: AttachmentId,

        /// The name of the attachment.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/name-attachmenttype>
        name: String,

        /// The MIME type of the attachment's content.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contenttype>
        content_type: String,

        /// An arbitrary identifier for the attachment.
        ///
        /// This field is not set by Exchange and is intended for use by
        /// external applications.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contentid>
        content_id: Option<String>,

        /// A URI representing the location of the attachment's content.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contentlocation>
        content_location: Option<String>,

        /// The size of the attachment's content in bytes.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/size>
        size: Option<usize>,

        /// The most recent modification time for the attachment.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/lastmodifiedtime>
        last_modified_time: Option<DateTime>,

        /// Whether the attachment appears inline in the item body.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/isinline>
        is_inline: Option<bool>,

        /// The attached item.
        #[serde(flatten)]
        content: Option<Box<RealItem>>,
    },

    /// An attachment containing a file.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/fileattachment>
    #[serde(rename_all = "PascalCase")]
    FileAttachment {
        /// An identifier for the attachment.
        attachment_id: AttachmentId,

        /// The name of the attachment.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/name-attachmenttype>
        name: String,

        /// The MIME type of the attachment's content.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contenttype>
        content_type: String,

        /// An arbitrary identifier for the attachment.
        ///
        /// This field is not set by Exchange and is intended for use by
        /// external applications.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contentid>
        content_id: Option<String>,

        /// A URI representing the location of the attachment's content.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contentlocation>
        content_location: Option<String>,

        /// The size of the attachment's content in bytes.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/size>
        size: Option<usize>,

        /// The most recent modification time for the attachment.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/lastmodifiedtime>
        last_modified_time: Option<DateTime>,

        /// Whether the attachment appears inline in the item body.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/isinline>
        is_inline: Option<bool>,

        /// Whether the attachment represents a contact photo.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/iscontactphoto>
        is_contact_photo: Option<bool>,

        /// The base64-encoded content of the attachment.
        ///
        /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/content>
        content: Option<String>,
    },
}

/// An identifier for an attachment.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/attachmentid>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct AttachmentId {
    /// A unique identifier for the attachment.
    #[serde(rename = "@Id")]
    #[xml_struct(attribute)]
    pub id: String,

    /// The unique identifier of the item to which it is attached.
    #[serde(rename = "@RootItemId")]
    #[xml_struct(attribute)]
    pub root_item_id: Option<String>,

    /// The change key of the item to which it is attached.
    #[serde(rename = "@RootItemChangeKey")]
    #[xml_struct(attribute)]
    pub root_item_change_key: Option<String>,
}

/// The content of an item, represented according to MIME (Multipurpose Internet
/// Mail Extensions).
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mimecontent>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct MimeContent {
    /// The character set of the MIME content if it contains [RFC 2045]-encoded
    /// text.
    ///
    /// [RFC 2045]: https://datatracker.ietf.org/doc/html/rfc2045
    #[serde(rename = "@CharacterSet")]
    #[xml_struct(attribute)]
    pub character_set: Option<String>,

    /// The item content.
    #[serde(rename = "$text")]
    #[xml_struct(flatten)]
    pub content: String,
}

/// The headers of an Exchange item's MIME content.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/internetmessageheader>
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct InternetMessageHeader {
    /// The name of the header.
    #[serde(rename = "@HeaderName")]
    #[xml_struct(attribute)]
    pub header_name: String,

    /// The value of the header.
    #[serde(rename = "$text")]
    #[xml_struct(flatten)]
    pub value: Option<String>,
}

/// View for `FindItem` and `FindConversation` operations
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/indexedpageitemview>
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/fractionalpageitemview>
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/calendarview>
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/contactsview>
#[derive(Clone, Debug, XmlSerialize)]
pub enum View {
    /// Describes how paged conversation or item information is
    /// returned for a `FindItem` operation or `FindConversation` operation request.
    IndexedPageItemView {
        #[xml_struct(attribute)]
        max_entries_returned: Option<usize>,

        #[xml_struct(attribute)]
        base_point: BasePoint,

        #[xml_struct(attribute)]
        offset: usize,
    },

    /// Describes where the paged view starts and the
    /// maximum number of items returned in a `FindItem` request.
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

    CalendarView {
        /// Describes the maximum number of results to return in the response.
        #[xml_struct(attribute)]
        max_entries_returned: Option<usize>,

        #[xml_struct(attribute)]
        start_date: String,

        #[xml_struct(attribute)]
        end_date: String,
    },

    ContactsView {
        /// Describes the maximum number of results to return in the response.
        #[xml_struct(attribute)]
        max_entries_returned: Option<usize>,

        #[xml_struct(attribute)]
        initial_name: Option<String>,

        #[xml_struct(attribute)]
        final_name: Option<String>,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Groups {
    #[serde(rename = "$value", default)]
    pub inner: Vec<GroupedItems>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct GroupedItems {
    pub group_index: Option<usize>,

    pub items: Items,
}

/// The value of the flag status for an item.
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/flagstatus>
#[derive(Clone, Debug, XmlSerialize, Deserialize, Eq, PartialEq)]
#[xml_struct(text)]
pub enum FlagStatus {
    NotFlagged,
    Flagged,
    Complete,
}

/// The flag information of the item.
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/flag>
#[derive(Clone, Debug, XmlSerialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Flag {
    pub flag_status: Option<FlagStatus>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub complete_date: Option<String>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        test_utils::{assert_deserialized_content, assert_serialized_content, minify_xml},
        Error,
    };

    /// Tests that a calendar item's own properties deserialize, which the
    /// shared [`Message`] representation did not carry before: a `CalendarView`
    /// answer would parse without error while silently dropping the start,
    /// end and location of every appointment.
    #[test]
    fn test_calendar_item_properties() -> Result<(), Error> {
        let xml = minify_xml(
            r#"
            <CalendarItem>
              <t:Subject>Weekly review</t:Subject>
              <t:Start>2026-08-26T09:00:00Z</t:Start>
              <t:End>2026-08-26T10:00:00Z</t:End>
              <t:IsAllDayEvent>false</t:IsAllDayEvent>
              <t:LegacyFreeBusyStatus>Busy</t:LegacyFreeBusyStatus>
              <t:Location>Room 3</t:Location>
              <t:IsRecurring>true</t:IsRecurring>
              <t:Organizer>
                <t:Mailbox>
                  <t:Name>Alice Test</t:Name>
                  <t:EmailAddress>alice@test.com</t:EmailAddress>
                </t:Mailbox>
              </t:Organizer>
              <t:RequiredAttendees>
                <t:Attendee>
                  <t:Mailbox>
                    <t:EmailAddress>bob@test.com</t:EmailAddress>
                  </t:Mailbox>
                  <t:ResponseType>Accept</t:ResponseType>
                </t:Attendee>
              </t:RequiredAttendees>
              <t:MyResponseType>Tentative</t:MyResponseType>
            </CalendarItem>"#,
        );

        let expected = Message {
            subject: Some("Weekly review".to_string()),
            start: Some(DateTime(
                time::OffsetDateTime::from_unix_timestamp(1787734800).unwrap(),
            )),
            end: Some(DateTime(
                time::OffsetDateTime::from_unix_timestamp(1787738400).unwrap(),
            )),
            is_all_day_event: Some(false),
            legacy_free_busy_status: Some(LegacyFreeBusyStatus::Busy),
            location: Some("Room 3".to_string()),
            is_recurring: Some(true),
            organizer: Some(Recipient {
                mailbox: Mailbox {
                    name: Some("Alice Test".to_string()),
                    email_address: Some("alice@test.com".to_string()),
                    ..Default::default()
                },
            }),
            required_attendees: Some(ArrayOfAttendees(vec![Attendee {
                mailbox: Mailbox {
                    email_address: Some("bob@test.com".to_string()),
                    ..Default::default()
                },
                response_type: Some(ResponseType::Accept),
                last_response_time: None,
            }])),
            my_response_type: Some(ResponseType::Tentative),
            ..Default::default()
        };

        assert_deserialized_content(&xml, expected);

        Ok(())
    }

    /// Tests that an [`ArrayOfRecipients`] correctly serializes into XML and
    /// back again. There should be a 1-to-1 correspondence between
    /// `<t:Mailbox>` elements and [`Recipient`]s.
    #[test]
    fn test_array_of_recipients() -> Result<(), Error> {
        let xml = minify_xml(
            r#"
            <Recipients>
              <t:Mailbox>
                <t:Name>Alice Test</t:Name>
                <t:EmailAddress>alice@test.com</t:EmailAddress>
              </t:Mailbox>
              <t:Mailbox>
                <t:Name>Bob Test</t:Name>
                <t:EmailAddress>bob@test.com</t:EmailAddress>
              </t:Mailbox>
              <t:Mailbox>
                <t:Name>Charlie Test</t:Name>
              </t:Mailbox>
            </Recipients>"#,
        );

        let data = ArrayOfRecipients(vec![
            Recipient {
                mailbox: Mailbox {
                    name: Some("Alice Test".into()),
                    email_address: Some("alice@test.com".into()),
                    ..Default::default()
                },
            },
            Recipient {
                mailbox: Mailbox {
                    name: Some("Bob Test".into()),
                    email_address: Some("bob@test.com".into()),
                    ..Default::default()
                },
            },
            Recipient {
                mailbox: Mailbox {
                    name: Some("Charlie Test".into()),
                    ..Default::default()
                },
            },
        ]);

        assert_serialized_content(&data, "Recipients", &xml);

        assert_deserialized_content(&xml, data);

        Ok(())
    }

    /// Test that [`ExtendedProperty`] correctly serializes into XML and back again.
    #[test]
    fn test_extended_property() -> Result<(), Error> {
        let data = ExtendedProperty {
            extended_field_URI: ExtendedFieldURI {
                distinguished_property_set_id: None,
                property_set_id: None,
                property_tag: Some("0x007D".into()),
                property_name: None,
                property_id: None,
                property_type: Some(PropertyType::String),
            },
            value: "data goes here".into(),
        };

        let xml = minify_xml(
            r#"
            <ExtendedProperty>
              <t:ExtendedFieldURI PropertyTag="0x007D" PropertyType="String"/>
              <t:Value>data goes here</t:Value>
            </ExtendedProperty>"#,
        );

        // Make sure data serializes into expected XML.
        assert_serialized_content(&data, "ExtendedProperty", &xml);

        // Make sure XML deserializes into expected data.
        assert_deserialized_content(&xml, data);
        Ok(())
    }

    /// Test that attachments are parsed properly
    #[test]
    fn test_attachment_parsing() {
        let item_attachment_xml = r#"
            <m:Attachments>
              <t:ItemAttachment>
                <t:AttachmentId Id="Ktum21o=" />
                <t:Name>Attached Message Item</t:Name>
                <t:ContentType>message/rfc822</t:ContentType>
                <t:Message>
                  <t:ItemId Id="AAMkAd" ChangeKey="FwAAABY" />
                </t:Message>
              </t:ItemAttachment>
            </m:Attachments>"#;

        let data = Attachments {
            inner: vec![Attachment::ItemAttachment {
                attachment_id: AttachmentId {
                    id: "Ktum21o=".to_string(),
                    root_item_id: None,
                    root_item_change_key: None,
                },
                name: "Attached Message Item".to_string(),
                content_type: "message/rfc822".to_string(),
                content_id: None,
                content_location: None,
                size: None,
                last_modified_time: None,
                is_inline: None,
                content: Some(Box::new(RealItem::Message(Message {
                    item_id: Some(ItemId {
                        id: "AAMkAd".to_string(),
                        change_key: Some("FwAAABY".to_string()),
                    }),
                    ..Default::default()
                }))),
            }],
        };

        assert_deserialized_content(item_attachment_xml, data);

        let file_attachment_xml = r#"
            <m:Attachments>
              <t:FileAttachment>
                <t:AttachmentId Id="AAAtAEFkbWluaX..."/>
                <t:Name>SomeFile</t:Name>
                <t:ContentType>message/rfc822</t:ContentType>
                <t:Content>AQIDBAU=</t:Content>
                <t:LastModifiedTime>2026-06-26T17:54:39Z</t:LastModifiedTime>
              </t:FileAttachment>
            </m:Attachments>"#;
        let data = Attachments {
            inner: vec![Attachment::FileAttachment {
                attachment_id: AttachmentId {
                    id: "AAAtAEFkbWluaX...".to_string(),
                    root_item_id: None,
                    root_item_change_key: None,
                },
                name: "SomeFile".to_string(),
                content_type: "message/rfc822".to_string(),
                content_id: None,
                content_location: None,
                size: None,
                last_modified_time: Some(DateTime(
                    OffsetDateTime::parse("2026-06-26T17:54:39Z", &Iso8601::DEFAULT).unwrap(),
                )),
                is_inline: None,
                is_contact_photo: None,
                content: Some("AQIDBAU=".to_string()),
            }],
        };

        assert_deserialized_content(file_attachment_xml, data);
    }

    #[test]
    /// Attachments sometimes have a `last_modified_time` without a timezone.
    fn test_deserialize_date_time_without_time_zone() {
        let content = r#"
            <t:Message>
              <t:DateTimeReceived>2026-06-26T17:54:39</t:DateTimeReceived>
            </t:Message>"#;

        let expected = Message {
            date_time_received: Some(DateTime(
                OffsetDateTime::parse("2026-06-26T17:54:39Z", &Iso8601::DEFAULT).unwrap(),
            )),
            ..Default::default()
        };

        assert_deserialized_content(content, expected);
    }

    #[test]
    fn test_serialize_flag_status() {
        let message = Message {
            flag: Some(Flag {
                flag_status: Some(FlagStatus::Flagged),
                start_date: None,
                due_date: None,
                complete_date: None,
            }),
            ..Default::default()
        };

        let expected = r#"<Message><t:Flag><FlagStatus>Flagged</FlagStatus></t:Flag></Message>"#;

        assert_serialized_content(&message, "Message", expected);
    }

    #[test]
    fn test_deserialize_flag_status() {
        let content = r#"
            <t:Message>
              <t:Flag>
                <t:FlagStatus>Flagged</t:FlagStatus>
                <t:StartDate>2026-01-26T23:00:00Z</t:StartDate>
                <t:DueDate>2026-01-26T23:00:00Z</t:DueDate>
              </t:Flag>
            </t:Message>"#;

        let expected = Message {
            flag: Some(Flag {
                flag_status: Some(FlagStatus::Flagged),
                start_date: Some("2026-01-26T23:00:00Z".to_string()),
                due_date: Some("2026-01-26T23:00:00Z".to_string()),
                complete_date: None,
            }),
            ..Default::default()
        };

        assert_deserialized_content(content, expected);
    }

    #[test]
    fn test_deserialize_empty_header() {
        let content = r#"
            <InternetMessageHeader HeaderName="X-Is-Empty"/>"#;
        let expected = InternetMessageHeader {
            header_name: "X-Is-Empty".to_string(),
            value: None,
        };
        assert_deserialized_content(content, expected);
    }
}
