/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use ews_proc_macros::operation_response;
use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{Mailbox, MESSAGES_NS_URI};

/// A request to find directory entries matching a name or partial address.
///
/// The operation that turns what a mailbox knows a person as — a display name,
/// or the internal directory entry an item carries instead of an address —
/// into something that can be written to.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/resolvenames>
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
#[operation_response(ResolveNamesResponseMessage)]
pub struct ResolveNames {
    /// Whether to return the full contact details of each match rather than
    /// its mailbox alone.
    #[xml_struct(attribute)]
    pub return_full_contact_data: bool,

    /// Where to look: the directory, this mailbox's contacts, or both.
    #[xml_struct(attribute)]
    pub search_scope: Option<ResolveNamesSearchScope>,

    /// The name, partial name or address to resolve.
    #[xml_struct(ns_prefix = "m")]
    pub unresolved_entry: String,
}

/// Where a [`ResolveNames`] request looks.
#[derive(Clone, Copy, Debug, XmlSerialize, PartialEq, Eq)]
#[xml_struct(text)]
pub enum ResolveNamesSearchScope {
    ActiveDirectory,
    ActiveDirectoryContacts,
    Contacts,
    ContactsActiveDirectory,
}

/// A response to a [`ResolveNames`] request.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/resolvenamesresponsemessage>
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ResolveNamesResponseMessage {
    /// The matches found, if any.
    pub resolution_set: Option<ResolutionSet>,
}

/// The matches a [`ResolveNames`] request found.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ResolutionSet {
    /// Whether the set holds every match, or was cut short.
    #[serde(rename = "@IncludesLastItemInRange")]
    pub includes_last_item_in_range: Option<bool>,

    #[serde(rename = "Resolution", default)]
    pub resolution: Vec<Resolution>,
}

/// One directory entry a name resolved to.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Resolution {
    pub mailbox: Mailbox,
}
