#include "layer.h"

Layer create_layer(int num_neurons, int num_inputs_per_neuron)
{
    Layer layer = {
        .num_neurons = num_neurons,
        .neurons = malloc(num_neurons * sizeof(Neuron))};

    for (int i = 0; i < num_neurons; i++)
    {
        layer.neurons[i] = create_neuron(num_inputs_per_neuron);
    }

    return layer;
}