import sys

# initialise the data
# return the number of occupied
def init():
    curr = []
    for row in range(0, len(lines)):
        for col in range(0, len(lines[row]))
            curr[row][col] = 0
            if (lines[row][col] == 'L'):
                # for the three above the current seat
                if (row != 0):
                    if (lines[row - 1][col] == 'L'):
                        curr[row][col] += 1
                    if (col != 0 and lines[row - 1][col - 1] == 'L'):
                        curr[row][col] += 1
                    if (col != len(lines[0]) - 1 and lines[row - 1][col + 1] == 'L'):
                        curr[row][col] += 1

                # for the 3 below the current seat
                if (row != len(lines) - 1):
                    if (lines[row + 1][col] == 'L'):
                        curr[row][col] += 1
                    if (col != 0 and lines[row + 1][col - 1] == 'L'):
                        curr[row][col] += 1
                    if (col != len(lines[0]) - 1 and lines[row + 1][col + 1] == 'L'):
                        curr[row][col] += 1

                # for the seat to the left of the current seat
                if (col != 0):
                    if (lines[row][col - 1] == 'L'):
                        curr[row][col] += 1

                # for the seat to the right of the current seat
                if (col != len(lines[0]) - 1):
                    if (lines[row][col + 1] == 'L'):
                        curr[row][col] += 1
    return curr

# do one iteration of the cycle. 
# return the number of seats occupied
def cycle():
    return 0

def a():
    currAdj = init()
    print(currAdj)

    prevOcc = 0
    nOcc = cycle()
    while(prevOcc != nOcc):
        prevOcc = nOcc
        nOcc = cycle()
    return nOcc

def b():
    pass


wholeInput = sys.stdin.read()
liens = wholeInput.split("\n")

print(a())
print(b())