use crate::neuron::Neuron;

pub struct Layer {
    pub num_neurons: usize,
    pub neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(num_neurons: usize, num_inputs: usize) -> Self {
        let mut neurons = Vec::new();

        for _ in 0..num_neurons {
            neurons.push(Neuron::new(num_inputs));
        }

        Self {
            num_neurons,
            neurons,
        }
    }

    pub fn set_inputs(&mut self, inputs: &[f64]) {
        for neuron in &mut self.neurons {
            neuron.set_inputs(inputs);
        }
    }

    pub fn get_error(&mut self, neuron_index: usize) -> f64 {
        let mut error: f64 = 0.0;

        for neuron in self.neurons.iter() {
            error += neuron.weights[neuron_index] * neuron.error;
        }

        error
    }
}
