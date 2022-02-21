CC := clang
SAN_FLAG := -fno-omit-frame-pointer -fsanitize=address
MYLIB := mylib.c -o libmy.so -shared -fPIC -g3

default: all

test.o:
	$(CC) $(SAN_FLAG) -c test.c -o test.o -g3

test: test.o
	$(CC) $(SAN_FLAG) -o test test.o -L. -lmy

mylib:
	$(CC) $(SAN_FLAG) $(MYLIB)

all: mylib 	test

run:
	LD_LIBRARY_PATH=. ASAN_OPTIONS=symbolize=1 ./test 1
	LD_LIBRARY_PATH=. ASAN_OPTIONS=symbolize=1 ./test 2
	LD_LIBRARY_PATH=. ASAN_OPTIONS=symbolize=1 ./test 3

clean:
	rm -f *.so *.o test