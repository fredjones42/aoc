# Advent of Code 2015 Day 01 - Python Solution Stub


def main():
    try:
        with open("../input.txt", "r") as f:
            data = f.read()
        print("Santa goes to floor", data.count("(") - data.count(")"))
        print("Santa enters basement at", pos_extra_rparen(data))
    except FileNotFoundError:
        print("Input file not found")


def pos_extra_rparen(data):
    n = 0
    for i, c in enumerate(data):
        if c == "(":
            n += 1
        elif c == ")":
            n -= 1
        else:
            raise Exception("unexpected char")
        if n == -1:
            return i + 1
    return 0


if __name__ == "__main__":
    main()
