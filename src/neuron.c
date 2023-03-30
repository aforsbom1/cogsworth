#include <stdlib.h>
#include "neuron.h"
#include "sigmoid.h"

Neuron create_neuron(int num_inputs)
{
    Neuron neuron = {
        .activation_function = &sigmoid,
        .bias = 0,
        .error = 0,
        .num_inputs = num_inputs,
        .weights = calloc(num_inputs, sizeof(double)),
        .output = 0};

    return neuron;
}
