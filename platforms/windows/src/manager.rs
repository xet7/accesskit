// Copyright 2021 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use std::sync::Arc;

use accesskit_consumer::{Tree, TreeChange};
use accesskit_provider::InitTree;
use accesskit_schema::TreeUpdate;
use lazy_init::LazyTransform;
use windows::Win32::{
    Foundation::*,
    UI::{Accessibility::*, WindowsAndMessaging::*},
};

use crate::node::{PlatformNode, ResolvedPlatformNode};

pub struct Manager<Init: InitTree = TreeUpdate> {
    hwnd: HWND,
    tree: LazyTransform<Init, Arc<Tree>>,
}

impl<Init: InitTree> Manager<Init> {
    pub fn new(hwnd: HWND, init: Init) -> Self {
        Self {
            hwnd,
            tree: LazyTransform::new(init),
        }
    }

    fn get_or_create_tree(&self) -> &Arc<Tree> {
        self.tree
            .get_or_create(|init| Tree::new(init.init_accesskit_tree()))
    }

    pub fn update(&self, update: TreeUpdate) {
        let tree = self.get_or_create_tree();
        self.update_internal(tree, update);
    }

    pub fn update_if_active(&self, updater: impl FnOnce() -> TreeUpdate) {
        let tree = match self.tree.get() {
            Some(tree) => tree,
            None => {
                return;
            }
        };
        self.update_internal(tree, updater());
    }

    fn update_internal(&self, tree: &Arc<Tree>, update: TreeUpdate) {
        tree.update_and_process_changes(update, |change| {
            match change {
                TreeChange::FocusMoved {
                    old_node: _,
                    new_node: Some(new_node),
                } => {
                    let platform_node = PlatformNode::new(&new_node, self.hwnd);
                    let el: IRawElementProviderSimple = platform_node.into();
                    unsafe { UiaRaiseAutomationEvent(el, UIA_AutomationFocusChangedEventId) }
                        .unwrap();
                }
                TreeChange::NodeUpdated { old_node, new_node } => {
                    let old_node = ResolvedPlatformNode::new(old_node, self.hwnd);
                    let new_node = ResolvedPlatformNode::new(new_node, self.hwnd);
                    new_node.raise_property_changes(&old_node);
                }
                // TODO: handle other events (#20)
                _ => (),
            };
        });
    }

    fn root_platform_node(&self) -> PlatformNode {
        let tree = self.get_or_create_tree();
        let reader = tree.read();
        let node = reader.root();
        PlatformNode::new(&node, self.hwnd)
    }

    pub fn handle_wm_getobject(&self, wparam: WPARAM, lparam: LPARAM) -> Option<LRESULT> {
        // Don't bother with MSAA object IDs that are asking for something other
        // than the client area of the window. DefWindowProc can handle those.
        // First, cast the lparam to i32, to handle inconsistent conversion
        // behavior in senders.
        let objid: i32 = (lparam.0 & 0xFFFFFFFF) as _;
        if objid < 0 && objid != UiaRootObjectId && objid != OBJID_CLIENT.0 {
            return None;
        }

        let el: IRawElementProviderSimple = self.root_platform_node().into();
        Some(unsafe { UiaReturnRawElementProvider(self.hwnd, wparam, lparam, el) })
    }
}
