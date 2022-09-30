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

#[cfg(test)]
mod tests {

    use super::{frog, frog_relaxsation};

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
}
