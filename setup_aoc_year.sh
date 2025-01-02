#!/bin/bash

# Script to set up Advent of Code folder structure for a specified year
# Usage: ./setup_aoc_year.sh <year>

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <year>"
    exit 1
fi

YEAR=$1
BASE_DIR="aoc/$YEAR"

# Create base directory for the year
mkdir -p "$BASE_DIR"

# Loop through days 1 to 25
for DAY in $(seq -w 1 25); do
    DAY_DIR="$BASE_DIR/day$DAY"
    RUST_DIR="$DAY_DIR/rust"
    PYTHON_DIR="$DAY_DIR/python"
    FORTRAN_DIR="$DAY_DIR/fortran"

    # Create directories
    mkdir -p "$RUST_DIR" "$PYTHON_DIR" "$FORTRAN_DIR"

    # Create blank input.txt
    touch "$DAY_DIR/input.txt"

    # Initialize Rust project
    cargo init --quiet --bin "$RUST_DIR"

    # Python stub
    PYTHON_FILE="$PYTHON_DIR/solution.py"
    cat > "$PYTHON_FILE" << EOF
# Advent of Code $YEAR Day $DAY - Python Solution Stub

def main():
    try:
        with open('../input.txt', 'r') as f:
            data = f.read()
        print("No Python solution has been written yet")
    except FileNotFoundError:
        print("Input file not found")


if __name__ == "__main__":
    main()
EOF

    # Fortran stub
    FORTRAN_FILE="$FORTRAN_DIR/solution.f90"
    cat > "$FORTRAN_FILE" << EOF
! Advent of Code $YEAR Day $DAY - Fortran Solution Stub
program day$DAY
    implicit none
    integer :: ios, lu
    character(len=16384) :: line

    open(newunit=lu, file="../input.txt", status="old", iostat=ios)
    if (ios /= 0) then
        print *, "Input file not found"
        stop
    end if

    do
        read(lu, '(a)', iostat=ios) line
        if (ios /= 0) exit
        print *, trim(line)
    end do
    close(lu)

    print *, "No Fortran solution has been written yet"
end program day$DAY
EOF

    # Fortran run script
    FRUN_FILE="$FORTRAN_DIR/frun.sh"
    cat > "$FRUN_FILE" << EOF
#!/bin/bash
# Compile and run the Fortran solution for Day $DAY

gfortran -o solution -Wall -Wextra -pedantic solution.f90
if [ "\$?" -eq 0 ]; then
    ./solution
else
    echo "Compilation failed!"
fi
EOF
    chmod +x "$FRUN_FILE"

    echo "Setup complete for Day $DAY"
done

echo "Setup complete for year $YEAR!"
