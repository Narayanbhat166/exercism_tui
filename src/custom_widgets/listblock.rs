use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub selected: usize,
}

impl<T: Clone> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        let mut list = Self {
            state: ListState::default(),
            items: vec![],
            selected: 0,
        };

        list.state.select(None);
        list
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut list = StatefulList {
            state: ListState::default(),
            items,
            selected: 0,
        };

        list.state.select(Some(list.selected));
        list
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected = i;
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected = i;
        self.state.select(Some(i));
    }

    pub fn get_current_item(&self) -> Option<T> {
        self.items.get(self.selected).map(|item| item.clone())
    }

    pub fn unselect(&mut self) {
        self.state.select(None)
    }

    pub fn add_items(&mut self, items: Vec<T>) {
        self.items.extend(items)
    }

    pub fn nop(&mut self) {}
}
