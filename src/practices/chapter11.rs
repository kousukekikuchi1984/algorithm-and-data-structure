use petgraph::unionfind::UnionFind;

pub struct Edge {
    from: u32,
    to: u32,
}

pub fn find_bridges(edge: Vec<Edge>, nodes: usize) -> u32 {
    let mut result = 0;
    for e in 0..edge.len() {
        let mut ufind: UnionFind<u32> = UnionFind::new(nodes);
        for i in 0..edge.len() {
            if i == e {
                continue;
            }
            ufind.union(edge[i].from, edge[i].to);
        }

        let mut count = 0;
        for v in 0..nodes as u32 {
            if ufind.find(v) == v {
                count += 1;
            }
        }
        if count > 1 {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::{find_bridges, Edge};

    #[test]
    fn test_find_num_of_bridges() {
        let bridges = vec![
            Edge { from: 0, to: 1 },
            Edge { from: 1, to: 2 },
            Edge { from: 2, to: 3 },
            Edge { from: 3, to: 4 },
            Edge { from: 4, to: 5 },
        ];
        assert_eq!(find_bridges(bridges, 6), 5);
    }
}
