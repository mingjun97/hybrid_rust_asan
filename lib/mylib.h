#pragma once
#include <stdlib.h>

int array_from_rust(int* array, int n) ;
int array_in_lib(int n);
int* array_to_rust(int size);
