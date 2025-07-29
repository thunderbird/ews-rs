/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Trait implementations for interoperability with the standard library.
//!
//! This module provides trait implementations to provide interoperability
//! between types in this crate and standard traits, notably the [`std::convert`]
//! traits.
//!
//! These are placed in a separate module from the underlying types to
//! ensure these implementations are opt-in for library clients.

use crate::{
    copy_folder::CopyFolder, copy_item::CopyItem, move_folder::MoveFolder, move_item::MoveItem,
    CopyMoveFolderData, CopyMoveItemData,
};

// Copy/Move item traits.

impl From<CopyMoveItemData> for CopyItem {
    /// Convert between [`CopyItem`] and [`CopyMoveItemData`] via the
    /// [`CopyItem::inner`] member.
    fn from(value: CopyMoveItemData) -> Self {
        Self { inner: value }
    }
}

impl From<CopyItem> for CopyMoveItemData {
    /// Convert between [`CopyItem`] and [`CopyMoveItemData`] via the
    /// [`CopyItem::inner`] member.
    fn from(value: CopyItem) -> Self {
        value.inner
    }
}

impl From<CopyMoveItemData> for MoveItem {
    /// Convert between [`MoveItem`] and [`CopyMoveItemData`] via the
    /// [`MoveItem::inner`] member.
    fn from(value: CopyMoveItemData) -> Self {
        Self { inner: value }
    }
}

impl From<MoveItem> for CopyMoveItemData {
    /// Convert between [`MoveItem`] and [`CopyMoveItemData`] via the
    /// [`MoveItem::inner`] member.
    fn from(value: MoveItem) -> Self {
        value.inner
    }
}

// Copy/Move folder traits.

impl From<CopyMoveFolderData> for CopyFolder {
    /// Convert between [`CopyFolder`] and [`CopyMoveFolderData`] via the
    /// [`CopyFolder::inner`] member.
    fn from(value: CopyMoveFolderData) -> Self {
        Self { inner: value }
    }
}

impl From<CopyFolder> for CopyMoveFolderData {
    /// Convert between [`CopyFolder`] and [`CopyMoveFolderData`] via the
    /// [`CopyFolder::inner`] member.
    fn from(value: CopyFolder) -> Self {
        value.inner
    }
}

impl From<CopyMoveFolderData> for MoveFolder {
    /// Convert between [`MoveFolder`] and [`CopyMoveFolderData`] via the
    /// [`MoveFolder::inner`] member.
    fn from(value: CopyMoveFolderData) -> Self {
        Self { inner: value }
    }
}

impl From<MoveFolder> for CopyMoveFolderData {
    /// Convert between [`MoveFolder`] and [`CopyMoveFolderData`] via the
    /// [`MoveFolder::inner`] member.
    fn from(value: MoveFolder) -> Self {
        value.inner
    }
}
