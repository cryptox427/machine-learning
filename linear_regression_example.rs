pub struct LinearRegression {
    pub slope: f64,
    pub intercept: f64,
}

impl LinearRegression {
    pub fn new() -> LinearRegression {
        LinearRegression {
            slope: 0.0,
            intercept: 0.0,
        }
    }

    pub fn fit(&mut self, x: &[f64], y: &[f64]) {
        let n = x.len() as f64;
        let sum_x = x.iter().sum::<f64>();
        let sum_y = y.iter().sum::<f64>();
        let sum_xx = x.iter().map(|&xi| xi * xi).sum::<f64>();
        let sum_xy = x
            .iter()
            .zip(y.iter())
            .map(|(&xi, &yi)| xi * yi)
            .sum::<f64>();

        let denominator = n * sum_xx - sum_x * sum_x;
        self.slope = (n * sum_xy - sum_x * sum_y) / denominator;
        self.intercept = (sum_y - self.slope * sum_x) / n;
    }

    pub fn predict(&self, x: f64) -> f64 {
        self.intercept + self.slope * x
    }
}

fn main() {
    let x = vec![10.0, 20.0, 30.0]; // Independent variable: Number of candies
    let y = vec![25.0, 50.0, 75.0]; // Dependent variable: Price of candies

    let mut model = LinearRegression::new();
    model.fit(&x, &y);

    let predicted_price = model.predict(47.0); // Predict the price for 47 candies
    println!("Predicted price for 47 candies: {}", predicted_price);
}
