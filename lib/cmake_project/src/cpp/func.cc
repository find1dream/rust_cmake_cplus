#include <iostream>
#include <cstdint>

using std::cout;
extern void foo2();

void foo(){
   cout << "hello world from cmake_project\r\n";
   foo2();
}

int32_t add(int32_t a, int32_t b){
    return a + b;
}
