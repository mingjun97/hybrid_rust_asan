#include "mylib.h" 

int array_from_rust(int* array, int n) {
    return array[n]; // Buggy access.
}

int array_in_lib(int n){
    int* array = malloc(100);
    int i;
    for (i = 0; i < 100; i++){
        array[i] = i + 10;
    }
    return array[n];
}

int* array_to_rust(int size){
    int *array = malloc(size * sizeof(int));
    int i;
        for (i = 0; i < size; i++){
        array[i] = i + 10;
    }
    return array;
}