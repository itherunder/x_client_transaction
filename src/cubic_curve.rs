pub struct Cubic {
    pub curves: Vec<f64>,
}


impl Cubic {
    pub fn new(curves: Vec<f64>) -> Cubic {
        Cubic { curves }
    }

    pub fn get_value(&self, time: f64) -> f64 {
        let (mut start_gradient, mut end_gradient) = (0.0, 0.0);
        let (mut start, mut mid) = (0.0, 0.0);
        let mut end = 1.0;

        if time <= 0.0 {
            if self.curves[0] > 0.0 {
                start_gradient = self.curves[1] / self.curves[0];
            } else if self.curves[0] == 0.0 && self.curves[2] > 0.0 {
                start_gradient = self.curves[3] / self.curves[2];
            }
            return start_gradient * time;
        }

        if time >= 1.0 {
            if self.curves[2] < 1.0 {
                end_gradient = (self.curves[3] - 1.0) / (self.curves[2] - 1.0);
            } else if self.curves[2] == 1.0 && self.curves[0] < 1.0 {
                end_gradient = (self.curves[1] - 1.0) / (self.curves[0] - 1.0);
            }
            return end_gradient * (time - 1.0) + 1.0;
        }

        while start < end {
            mid = (start + end) / 2.0;
            let x_est = self.calculate(self.curves[0], self.curves[2], mid);
            if (time - x_est).abs() < 0.00001 {
                return self.calculate(self.curves[1], self.curves[3], mid);
            }
            if x_est < time {
                start = mid;
            } else {
                end = mid;
            }
        }

        self.calculate(self.curves[1], self.curves[3], mid)
    }

    pub fn calculate(&self, a: f64, b: f64, m: f64) -> f64 {
        a * 3.0 * (1.0 - m) * (1.0 - m) * m + b * 3.0 * (1.0 - m) * m * m + m * m * m
    }
}
