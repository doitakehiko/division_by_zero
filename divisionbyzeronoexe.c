#include <stdio.h>

int main() {
    return 1 / 0;
}
/*

gcc divisionbyzeronoexe.c
divisionbyzeronoexe.c: In function ‘main’:
divisionbyzeronoexe.c:4:14: warning: division by zero [-Wdiv-by-zero]
    4 |     return 1 / 0;
      |              ^
*/
