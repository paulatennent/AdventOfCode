import sys
wholeInput = sys.stdin.read()

def a():
    seen = set()

    acc = 0
    curr = 0
    while (curr != len(lines)):

        if curr in seen:
            return acc
        seen.add(curr)

        line = lines[curr].split(" ")

        if (line[0] == "nop"):
            curr += 1
        elif (line[0] == "acc"):
            inc = line[1][1:]
            curr += 1
            if (line[1][0] == '+'):
                acc += int(inc)
            else:
                acc -= int(inc)
        elif (line[0] == "jmp"):
            jmp = line[1][1:]
            if (line[1][0] == '+'):
                curr += int(jmp)
            else:
                curr -= int(jmp)
            

def b():
    pass

lines = wholeInput.split("\n")
print(a())