#include <stdio.h>
#include "network.h"

int main()
{
    Network network;
    initialize_network(&network, 2, 1, 1, 3); // Example: 2 inputs, 1 output, 1 hidden layer with 3 neurons

    printf("%d\n", network.num_layers);

    return 0;
}