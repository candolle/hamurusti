pub struct AccumulatingMean {
    count: u32,
    total: f32,
}

impl AccumulatingMean {
    pub fn new() -> AccumulatingMean {
        AccumulatingMean {
            count: 0,
            total: 0f32,
        }
    }
    pub fn push(&mut self, item: f32) {
        self.count += 1;
        self.total += item;
    }
    pub fn mean(&self) -> f32 {
        match self.count {
            0 => 0f32,
            _ => self.total / self.count as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acc_mean() {
        let mut acc_mean = AccumulatingMean::new();
        let numbers = [1f32, 2f32, 3f32, 4.5f32];
        for i in 0 .. numbers.len() {
            acc_mean.push(numbers[i]);
            let sum: f32 = numbers.iter().take(i + 1).sum();
            let mean = sum / (i + 1) as f32;
            assert_eq!(acc_mean.mean(), mean);
        }
    }
}
