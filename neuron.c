#include <stdlib.h>
#include "neuron.h"
#include "sigmoid.h"

Neuron createNeuron()
{
    Neuron n;
    n.weights = malloc(n.num_inputs * sizeof(double));
    n.weights[0] = 0.2;
    n.weights[1] = 0.5;
    n.weights[2] = 0.1;
    n.bias = -0.3;

    return n;
}
