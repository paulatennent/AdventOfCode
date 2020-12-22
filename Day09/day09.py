import sys

def check(l, total):
    l.sort()
    length = len(l)
    #print(l, total)

    a = 0
    b = length - 1
    while (a < b):
        if (l[a] + l[b] == total):
            
            return True
        elif (l[a] + l[b] < total):
            a += 1
        else:
            b -= 1
    return False

def a():
    for i in range(preamble, len(lines)):
        #print(lines[i-preamble:i], lines[i])
        if (not check(lines[i-preamble:i], lines[i])):
            return lines[i]
        

def b():
    for i in range(0, len(lines)):
        sum = lines[i]
        curr = 1
        currMin = lines[i]
        currMax = lines[i]
        while(sum <= invalid):
            if (sum == invalid):
                return currMin + currMax
            sum += lines[i + curr]
            currMin = min(currMin, lines[i + curr])
            currMax = max(currMax, lines[i + curr])
            curr += 1

wholeInput = sys.stdin.read()
lines = wholeInput.split("\n")
lines = [int(i) for i in lines]
preamble = 25
invalid = a()
print(invalid)
print(b())
