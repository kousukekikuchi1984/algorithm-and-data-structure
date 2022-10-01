pub fn frog(costs: Vec<u32>) -> u32 {
    let mut cost_table = vec![std::u32::MAX; costs.len()];
    cost_table[0] = 0;

    for i in 1..costs.len() {
        let cost = i32::abs(costs[i] as i32 - costs[i - 1] as i32) as u32 + cost_table[i - 1];
        if cost < cost_table[i] {
            cost_table[i] = cost;
        }
        if i > 1 {
            let cost = (costs[i] as i32 - costs[i - 2] as i32).abs() as u32 + cost_table[i - 2];
            if cost < cost_table[i] {
                cost_table[i] = cost;
            }
        }
    }

    return cost_table[cost_table.len() - 1];
}

pub fn frog_relaxsation(costs: Vec<u32>) -> u32 {
    fn relaxzation(cost_table_val: &mut u32, calculated: u32) {
        if *cost_table_val > calculated {
            *cost_table_val = calculated;
        }
    }

    let mut cost_table = vec![std::u32::MAX; costs.len()];
    cost_table[0] = 0;

    for i in 1..costs.len() {
        let cost = i32::abs(costs[i] as i32 - costs[i - 1] as i32) as u32 + cost_table[i - 1];
        relaxzation(&mut cost_table[i], cost);
        if i > 1 {
            let cost = (costs[i] as i32 - costs[i - 2] as i32).abs() as u32 + cost_table[i - 2];
            relaxzation(&mut cost_table[i], cost);
        }
    }
    return cost_table[cost_table.len() - 1];
}

pub fn frog_distribute(costs: Vec<u32>) -> u32 {
    fn relaxzation(cost_table_val: &mut u32, calculated: u32) {
        if *cost_table_val > calculated {
            *cost_table_val = calculated;
        }
    }

    let mut cost_table = vec![std::u32::MAX; costs.len()];
    cost_table[0] = 0;
    let len = costs.len();

    for i in 0..costs.len() - 2 {
        if i + 1 < len {
            let cost = i32::abs(costs[i + 1] as i32 - costs[i] as i32) as u32 + cost_table[i];
            relaxzation(&mut cost_table[i + 1], cost);
        }

        if i + 2 < len {
            let cost = i32::abs(costs[i + 2] as i32 - costs[i] as i32) as u32 + cost_table[i];
            relaxzation(&mut cost_table[i + 2], cost)
        }
    }
    return cost_table[cost_table.len() - 1];
}

pub fn frog_naive_recursive(costs: Vec<u32>) -> u32 {
    fn relaxzation(cost_table_val: &mut u32, calculated: u32) {
        if *cost_table_val > calculated {
            *cost_table_val = calculated;
        }
    }

    fn _frog(i: usize, costs: &Vec<u32>) -> u32 {
        if i == 0 {
            return 0;
        }
        let mut res = std::u32::MAX;

        // from i - 1
        relaxzation(
            &mut res,
            _frog(i - 1, costs) + (costs[i] as i32 - costs[i - 1] as i32).abs() as u32,
        );

        // from i - 2
        if i > 2 {
            relaxzation(
                &mut res,
                _frog(i - 2, costs) + (costs[i] as i32 - costs[i - 2] as i32).abs() as u32,
            );
        }
        return res;
    }

    return _frog(costs.len() - 1, &costs);
}

pub fn frog_recursive(costs: Vec<u32>) -> u32 {
    fn relaxzation(cost_table_val: &mut u32, calculated: u32) {
        if *cost_table_val > calculated {
            *cost_table_val = calculated;
        }
    }
    let mut cost_table = vec![std::u32::MAX; costs.len()];
    cost_table[0] = 0;

    fn _frog(i: usize, costs: &Vec<u32>, cost_table: &mut Vec<u32>) -> u32 {
        if i == 0 {
            return 0;
        }
        if cost_table[i] < std::u32::MAX {
            return cost_table[i];
        }
        let mut res = std::u32::MAX;

        relaxzation(
            &mut res,
            _frog(i - 1, costs, cost_table) + (costs[i] as i32 - costs[i - 1] as i32).abs() as u32,
        );

        if i > 1 {
            relaxzation(
                &mut res,
                _frog(i - 2, costs, cost_table)
                    + (costs[i] as i32 - costs[i - 2] as i32).abs() as u32,
            );
        }

        cost_table[i] = res;
        return res;
    }

    return _frog(costs.len() - 1, &costs, &mut cost_table);
}

#[cfg(test)]
mod tests {

    use super::{frog, frog_distribute, frog_naive_recursive, frog_recursive, frog_relaxsation};

    #[test]
    fn test_frog() {
        assert_eq!(frog(vec![1, 2, 3, 4]), 3);
        assert_eq!(frog(vec![1, 3, 2, 4]), 3);
    }

    #[test]
    fn test_frog_relaxsation() {
        assert_eq!(frog_relaxsation(vec![1, 2, 3, 4]), 3);
        assert_eq!(frog_relaxsation(vec![1, 3, 2, 4]), 3);
    }

    #[test]
    fn test_frog_distribute() {
        assert_eq!(frog_distribute(vec![1, 2, 3, 4]), 3);
        assert_eq!(frog_distribute(vec![1, 3, 2, 4]), 3);
    }

    #[test]
    fn test_frog_naive_recursive() {
        assert_eq!(frog_naive_recursive(vec![1, 2, 3, 4]), 3);
        assert_eq!(frog_naive_recursive(vec![1, 3, 2, 4]), 3);
    }

    #[test]
    fn test_frog_recursive() {
        assert_eq!(frog_recursive(vec![1, 3, 2, 4]), 3);
        assert_eq!(frog_recursive(vec![1, 3, 2, 4]), 3);
    }
}
