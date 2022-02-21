#include <stdio.h>

#ifdef DYNAMIC
#include <dlfcn.h>
#else
#include "mylib.h"
#endif
int main(int argc, char* argv[]){

#ifdef DYNAMIC
    void* handle = dlopen('./mylib.so', RTLD_LAZY);
    if (!handle){
        printf("Libarry not found");
        return 1;
    }
#else
    int array[5];
    int * ret;
    switch (atoi(argv[1])) {
        case 1:
            printf("Array from rust\n");
            array_from_rust(array, 6);
            break;
        case 2:
            printf("Array in lib\n");
            array_in_lib(101);
            break;
        case 3:
            printf("Array to rust\n");
            ret = array_to_rust(5);
            ret[6] += 5;
    }

#endif

}