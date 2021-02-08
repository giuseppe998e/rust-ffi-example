#include <stdio.h>
#include "testlib.h"

int main(int argc, char *argv[]) {
  int x = 50, y = 10,
      rsum = rust_sum(x, y),
      rsub = rust_sub(x, y),
      rmul = rust_mul(x, y),
      rdiv = rust_div(x, y);
      
  printf("Rust fn sum(x,y) -> z: %d + %d = %d\n", x, y, rsum);
  printf("Rust fn sub(x,y) -> z: %d + %d = %d\n", x, y, rsub);
  printf("Rust fn mul(x,y) -> z: %d + %d = %d\n", x, y, rmul);
  printf("Rust fn div(x,y) -> z: %d + %d = %d\n", x, y, rdiv);
}
