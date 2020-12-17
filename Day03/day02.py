import sys

# input in lines
wholeInput = sys.stdin.read()
lst = wholeInput.split("\n")
length = len(lst)
width = len(lst[0])

# count number of trees
count = 0

# start at top left
x = 0
y = 0
# while not at the bottom
while (y != length):
    # check if its a tree
    if lst[y][x] == '#':
        count += 1

    # if its at the right, move back to the left
    x += 3
    x = x % width
    y += 1

print(count)