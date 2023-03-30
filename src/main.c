#include <stdio.h>
#include "neuron.h"

int main()
{
    Neuron n = create_neuron(10);

    printf("%d\n", n.num_inputs);

    return n.num_inputs;
}