use std::collections::LinkedList;

pub trait LinkdListExtensions<T, F> {
    fn pop_where(
        &mut self,
        f: F,
    ) -> Option<T>
    where
        F: Fn(&T) -> bool;
}

impl<T, F> LinkdListExtensions<T, F> for LinkedList<T> {
    fn pop_where(
        &mut self,
        f: F,
    ) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        if self.is_empty() {
            return None;
        }

        let item_pos = if let Some(ind) = self.iter().position(f) {
            ind
        } else {
            return None;
        };

        if item_pos == self.len() - 1 {
            return self.pop_back();
        }

        if item_pos == 0 {
            return self.pop_front();
        }

        let mut tail = self.split_off(item_pos);
        let item = self.pop_back();
        self.append(&mut tail);
        item
    }
}
