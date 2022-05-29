CC=g++
CFLAGS=-g -std=c++17 -m64 -Wall -pedantic

test: main.cpp test.cpp
	$(CC) $(CFLAGS) -c test.cpp
	$(CC) $(CFLAGS) -c main.cpp -D TEST
	$(CC) $(CFLAGS) test.o main.o -o test
