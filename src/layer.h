#include "neuron.h"

typedef struct
{
    Neuron *neurons;
    int num_neurons;
} Layer;

Layer create_layer(int num_neurons, int num_inputs_per_neuron);