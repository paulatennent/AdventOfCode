import sys

def a():
    lines.sort()
    prev = 0
    nOneDiff = 0
    nThreeDiff = 1
    for adaptor in lines:
        if (adaptor - prev == 1):
            nOneDiff += 1
        elif (adaptor - prev == 3):
            nThreeDiff += 1
        prev = adaptor
    return nOneDiff * nThreeDiff

def b():
    dp = {}
    dp[0] = 1
    dp[lines[0]] = 1
    for i in range(1, len(lines)):
        if (i > 0 and lines[i] - lines[i - 1] <= 3):
            dp[lines[i]] = dp[lines[i - 1]]
        if (i > 1 and lines[i] - lines[i - 2] <= 3):
            dp[lines[i]] += dp[lines[i - 2]]
        if (i > 2 and lines[i] - lines[i - 3] <= 3):
            dp[lines[i]] += dp[lines[i - 3]]
        # add one for connecting straight to the outlet
        if (lines[i] <= 3):
            dp[lines[i]] += 1
    #print(lines)
    #for item in dp:
    #    print(item, dp[item])
    #print(dp)
    return dp[lines[i]]

wholeInput = sys.stdin.read()
lines = wholeInput.split("\n")
lines = [int(i) for i in lines]

print(a())
print(b())

# any adaptor can take input of 1, 2, or 3 lower than rating and product output
# you have an inbult adaptor of 3 higher than highest adaptor
# charging output has joltage of rating 0
# using ALL adaptors at once, what is the distribution of joltage differences between the charging outlet