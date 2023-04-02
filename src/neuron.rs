pub struct Neuron {
    pub output: f64,
    pub error: f64,
    pub weights: Vec<f64>,
    pub bias: f64,
}

impl Neuron {
    pub fn new(num_inputs: usize) -> Neuron {
        let mut weights = Vec::with_capacity(num_inputs);
        for _ in 0..num_inputs {
            weights.push(rand::random::<f64>());
        }
        let bias = rand::random::<f64>();
        Neuron {
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
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
