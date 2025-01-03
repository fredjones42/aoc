# Advent of Code 2015 Day 03 - Python Solution Stub
from collections import defaultdict


def main():
    try:
        with open("../input.txt", "r") as f:
            data = f.read()

        grid = defaultdict(int)
        i, j = 0, 0

        grid[i, j] = 1

        for c in data:
            if c == "^":
                j += 1
            elif c == "v":
                j -= 1
            elif c == ">":
                i += 1
            elif c == "<":
                i -= 1
            grid[i, j] += 1

        print(len(grid), "houses received at least one present.")

        grid.clear()
        i, j, ir, jr = 0, 0, 0, 0
        grid[i, j] = 2

        for k, c in enumerate(data):
            if k % 2 == 0:
                if c == "^":
                    j += 1
                elif c == "v":
                    j -= 1
                elif c == ">":
                    i += 1
                elif c == "<":
                    i -= 1
                grid[i, j] += 1
            else:
                if c == "^":
                    jr += 1
                elif c == "v":
                    jr -= 1
                elif c == ">":
                    ir += 1
                elif c == "<":
                    ir -= 1
                grid[ir, jr] += 1

        print("With Robo-Santa...")
        print(len(grid), "houses received at least one present.")

    except FileNotFoundError:
        print("Input file not found")


if __name__ == "__main__":
    main()
