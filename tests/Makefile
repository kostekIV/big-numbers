

CC    = g++
CP    = cp
MKDIR = mkdir -p

all: generate

py:
	${CP} generators/gen.py build/gen.py

cpp:
	${MKDIR} build
	${CC} generators/main.cpp -o build/gen -lgmpxx -lgmp

generate: cpp py
	./build/gen
	python3 ./build/gen.py
