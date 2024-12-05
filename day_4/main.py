def get_diags(data: list[str]) -> list[str]:
    wordsearch_size = len(data[0])
    diags = []
    for start in range(wordsearch_size * 2 - 1):
        this_line = ''
        for row in range(wordsearch_size):
            col = start - row
            if 0 <= col < wordsearch_size:
                this_line += data[row][col]
        diags.append(this_line)
    return diags

def part_1():
    
    # Build up a master list of strings to test
    all_search_strings = []
    count = 0
    
    # Get raw data
    data = []
    with open('./data_1.txt', 'r') as data_file:
        data = data_file.readlines()
    data = [line.replace('\n', '') for line in data]

    # forwards and backwards horiizontally
    for line in data:
        all_search_strings.append(line)
        all_search_strings.append(line[::-1])

    # Transpose rows in to cols to get the vertical possibilities
    transposed = [''.join(row) for row in zip(*data)]
    for line in transposed:
        all_search_strings.append(line)
        all_search_strings.append(line[::-1])

    all_search_strings += get_diags(data)  # south-west diags
    all_search_strings += get_diags([r[::-1] for r in data])  # south-east diags
    all_search_strings += get_diags(data[::-1])  # north-east diags
    all_search_strings += get_diags([r[::-1] for r in data[::-1]])  # north-west diags
    
    # Count
    for row in all_search_strings:
        count += row.count('XMAS')

    print(count)

def part_2():
    count = 0
    
    # Get raw data
    data = []
    with open('./data_2.txt', 'r') as data_file:
        data = data_file.readlines()
    data = [line.replace('\n', '') for line in data]

    for row_index, row in enumerate(data):
        for col_index, char in enumerate(row):
            if char == 'A':
                if row_index-1 < 0 or col_index-1 < 0:
                    continue
                try:
                    top_left = data[row_index-1][col_index-1]
                    top_right = data[row_index-1][col_index+1]
                    bottom_left = data[row_index+1][col_index-1]
                    bottom_right = data[row_index+1][col_index+1]
                    if (
                        (top_left == 'M' and bottom_right == 'S' and top_right == 'S' and bottom_left == 'M') or
                        (top_left == 'M' and bottom_right == 'S' and top_right == 'M' and bottom_left == 'S') or
                        (top_left == 'S' and bottom_right == 'M' and top_right == 'S' and bottom_left == 'M') or
                        (top_left == 'S' and bottom_right == 'M' and top_right == 'M' and bottom_left == 'S')
                    ):
                        count += 1
                except IndexError:
                    continue
    
    print(count)


if __name__ == "__main__":
    part_1()
    part_2()