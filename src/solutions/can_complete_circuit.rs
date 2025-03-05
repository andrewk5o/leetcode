pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    if gas.iter().sum::<i32>() < cost.iter().sum::<i32>() {
        return -1;
    }

    let mut fuel = 0;
    let mut result = 0;

    for i in 0..gas.len() {
        fuel += gas[i] - cost[i];
        if fuel < 0 {
            fuel = 0;
            result = i + 1;
        }
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_complete_circuit() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(can_complete_circuit(gas, cost), 3);
    }
}
