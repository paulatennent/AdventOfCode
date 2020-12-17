import sys
import re 


def dfs(curr, bagGra):
    
    # set the current bag to be seen
    total = 1
    #print("\ncurr: ", curr)

    if curr[0] in bagGra:
        for bag in bagGra[curr[0]]:
            add = dfs(bag, bagGra) * int(bag[1])
            total += add
            #print(bag)
            #print(add, bag[1], total)

    #print("\nendcurr: ", curr, "return: ", total)
    return total

# create graph
bagGra = {}

# insert in data
wholeInput = sys.stdin.read()

# split up by line
lines = wholeInput.split("\n")

# find first bag and list of bags inside
for line in lines:
    fstBag = re.findall("\w* \w*", line)
    if (len(fstBag) > 1):
        fstBag = fstBag[0]
    else:
        continue
    #print(fstBag)

    oth = re.sub(".*contain ", "", line)
    oth = re.sub(".*no other.*", "", oth)
    oth = re.sub("\.", "", oth)
    oth = re.sub("bags*", "", oth)
    oth = re.findall("\d+ \w+ \w+", oth)
    #print(oth)

    # connect first bag to all bags in list (going backwards)
    for bag in oth:
        num = re.findall("\d+", bag)[0]
        bag = re.sub("\d ?", "", bag)
        #print(num, bag)
        item = [bag, num]
        if fstBag in bagGra:
            bagGra[fstBag].append(item)
        else:
            bagGra[fstBag] = [item]

    #print(bagGra)

# go from shiny gold bag and do a bfs to find all bags inside
seen = []
total = dfs(["shiny gold", 1], bagGra)
print(total - 1)