#include <stdio.h>
#include "network.h"

int main()
{
    Network network = initialize_network(2, 1, 1, 3);

    printf("%d\n", network.num_layers);

    return 0;
}