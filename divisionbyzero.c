#include <stdio.h>

int main() {
    for( int i = 10; i >= 0; i--) { 
        printf("%f\n" , (float)10 / i );
    }
    return 0;
}
/*
gcc divisionbyzero.c -o divisionbyzero
./divisionbyzero
1.000000
1.111111
1.250000
1.428571
1.666667
2.000000
2.500000
3.333333
5.000000
10.000000
inf
*/
