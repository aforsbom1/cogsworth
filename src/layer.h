#include "neuron.h"

typedef struct Layer
{
    Neuron *neurons;
    int num_neurons;
} Layer;

Layer create_layer(int num_neurons, int num_inputs_per_neuron);
