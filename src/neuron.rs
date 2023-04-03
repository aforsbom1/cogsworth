pub struct Neuron {
    pub output: f64,
    pub error: f64,
    pub weights: Vec<f64>,
    pub bias: f64,
}

impl Neuron {
    pub fn new(num_inputs: usize) -> Self {
        let mut weights = Vec::with_capacity(num_inputs);
        for _ in 0..num_inputs {
            weights.push(rand::random::<f64>());
        }
        let bias = rand::random::<f64>();
        Self {
            output: 0.0,
            error: 0.0,
            weights,
            bias,
        }
    }

    pub fn set_inputs(&mut self, inputs: &[f64]) {
        let sum = self
            .weights
            .iter()
            .zip(inputs.iter())
            .map(|(&w, &i)| w * i)
            .sum::<f64>()
            + self.bias;
        self.output = sigmoid(sum);
    }

    pub fn update_weights(&mut self) {
        for i in 0..self.weights.len() {
            self.weights[i] += self.error * self.output * (1.0 - self.output) * self.output;
        }
        self.bias += self.error * self.output * (1.0 - self.output) * self.output;
    }
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
