pub struct Averager {
    count: u32,
    tree: Vec<Option<f64>>,
}

impl Averager {
    pub fn new() -> Averager {
        Averager { count: 0, tree: Vec::new() }
    }

    pub fn update(&mut self, mut value: f64) {
        self.count = self.count.checked_add(1).unwrap();

        for elem in &mut self.tree {
            match elem.take() {
                None => {
                    *elem = Some(value);
                    return;
                }
                Some(e) => value = (e + value) / 2.0,
            }
        }

        self.tree.push(Some(value));
    }

    pub fn average(&self) -> f64 {
        let mut average = 0f64;
        for (i, v) in self.tree.iter().enumerate() {
            let weight = 2f64.powi(i as i32) / f64::from(self.count);
            average += v.unwrap_or(0.0) * weight;
        }
        average
    }
}
