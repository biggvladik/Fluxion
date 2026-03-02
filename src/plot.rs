pub struct PlotData {
    pub points: Vec<[f64; 2]>,
    pub t_otgona: f64,
}

impl PlotData {
    pub fn new() -> Self {
        Self {
            points: vec![],
            t_otgona: 0.0,
        }
    }

    pub fn add_point(&mut self, value: f64) {
        let x = self.points.len() as f64;
        self.points.push([x, value]);
        self.t_otgona = value;

        if self.points.len() > 1000 {
            self.points.remove(0);
        }
    }
}