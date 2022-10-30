use std::collections::VecDeque;

pub struct Graph {
    v: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    pub fn search(&self, s: usize) -> Vec<bool> {
        let mut seen = vec![false; self.v.len()];
        self._search(s, &mut seen);
        seen
    }

    pub fn _search(&self, s: usize, seen: &mut Vec<bool>) {
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

    pub fn search_by_recursive(&self, s: usize) -> Vec<bool> {
        let n = self.v.len();
        let mut seen = vec![false; n];
        self._search_by_resursive(s, &mut seen);
        seen
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

#[cfg(test)]
mod test {
    use super::Graph;

    #[test]
    fn test_search() {
        let mut graph = Graph::new();
        graph.v.push(vec![1, 2]);
        graph.v.push(vec![3]);
        graph.v.push(vec![3]);
        let actual = graph.search_by_recursive(0);
        let expected = vec![true, false, false];
        assert_eq!(actual, expected);
    }
}
