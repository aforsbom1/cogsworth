use crate::neuron::Neuron;

pub struct Layer {
    num_neurons: usize,
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(num_neurons: usize) -> Self {
        let mut neurons = Vec::new();

        for _ in 0..num_neurons {
            neurons.push(Neuron::new(num_neurons));
        }
    
        return Self {
            num_neurons,
            neurons,
        }
    }

}