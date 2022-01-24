use nbt::types::Tag;
use tui::widgets::{ListItem, ListState};

struct NestedItem {
    item: &Tag,
    folded: bool
}

struct NestedList {
    list: Vec<NestedItem>,
    state: ListState
}

impl NestedList {
    fn unfold(&mut self, i: usize) {
        let a = self.list[i];

    }
}

impl Tag {
    fn to_list_item(&self, unfold: bool) -> ListItem;
}