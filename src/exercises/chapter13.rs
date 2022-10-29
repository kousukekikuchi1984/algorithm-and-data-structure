use std::collections::VecDeque;

pub struct Graph {
    v: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    pub fn search(&self, s: usize) {
        let n = self.v.len();
        let mut seen = vec![false; n];
        let mut todo: VecDeque<usize> = VecDeque::new();

        seen[s] = true;
        todo.push_back(s);

        while !todo.is_empty() {
            let v = todo.pop_front().unwrap();
            for x in &self.v[v] {
                match seen.get(*x) {
                    Some(_) => continue,
                    None => {
                        seen[*x] = true;
                        todo.push_back(*x);
                    }
                }
            }
        }
    }

    pub fn search_by_recursive(&self, s: usize) {
        let n = self.v.len();
        let mut seen = vec![false; n];
        self._search_by_resursive(s, &mut seen)
    }

    fn _search_by_resursive(&self, v: usize, seen: &mut Vec<bool>) {
        seen[v] = true;
        for x in &self.v[v] {
            match seen.get(*x) {
                Some(_) => continue,
                None => self._search_by_resursive(*x, seen),
            }
        }
    }
}
