#!/bin/bash
# Compile and run the Fortran solution for Day 04

gfortran -o solution -Wall -Wextra -pedantic solution.f90
if [ "$?" -eq 0 ]; then
    ./solution
else
    echo "Compilation failed!"
fi
