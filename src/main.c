#include <stdio.h>
#include "layer.h"

int main()
{
    Layer layer = create_layer(3, 4);

    printf("%d\n", layer.num_neurons);

    return 0;
}