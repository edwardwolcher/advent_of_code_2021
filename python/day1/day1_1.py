data_path = "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day1.txt"

test_input = [
    199,
    200,
    208,
    210,
    200,
    207,
    240,
    269,
    260,
    263,
]


def read_input(path):
    input_list = list()
    with open(path, "r") as f:
        for line in f.readlines():
            measurement = int(line)
            input_list.append(measurement)
    return input_list


def count_increases(input_list):
    increases = 0
    previous = None
    for measurement in input_list:
        if previous is not None:
            if measurement > previous:
                increases += 1
        previous = measurement
    return increases


def main():
    input_list = read_input(data_path)
    result = count_increases(input_list)
    print(f"Day 1.1: {result}")


def test():
    result = count_increases(test_input)
    if result != 7:
        print(f"Day 1.1 Test Failed - result: {result} should be 7")
    else:
        print(f"Day 1.1 Test succeeded, result: {result}")


if __name__ == "__main__":
    # test()
    main()
