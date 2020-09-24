use crate::domain::entities::Keep;
use tui::widgets::ListState;

pub struct KeepItems {
    pub items: Vec<Keep>,
    pub state: ListState,
}

impl KeepItems {
    pub fn new(items: Vec<Keep>) -> KeepItems {
        KeepItems {
            items,
            state: ListState::default(),
        }
    }

    pub fn set_items(&mut self, items: Vec<Keep>) {
        self.items = items;
        self.state = ListState::default();
    }

    pub fn next(&mut self) {
        let index_selected = match self.state.selected() {
            Some(index_selected) => {
                if index_selected >= self.items.len() - 1 {
                    0
                } else {
                    index_selected + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(index_selected));
    }

    pub fn previous(&mut self) {
        let index_selected = match self.state.selected() {
            Some(index_selected) => {
                if index_selected == 0 {
                    self.items.len() - 1
                } else {
                    index_selected - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(index_selected));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
