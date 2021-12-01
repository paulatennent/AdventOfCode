import sys

# check if the given letter appears the right amount of times
def a(lst):
    counter = 0
    for line in lst:
        arr = line.split(" ")
        num = arr[0].split("-")
        letter = arr[1][0]
        password = arr[2]

        nLetter = 0
        for char in password:
            if (char == letter):
                nLetter += 1
        
        if (nLetter >= int(num[0]) and nLetter <= int(num[1])):
            counter += 1

    return counter

# x-y a: password
# check that at position x is an a, and y is not an a (or the other way around)
def b(lst):
    counter = 0
    for line in lst:
        arr = line.split(" ")
        num = arr[0].split("-")
        letter = arr[1][0]
        password = arr[2]

        if (password[int(num[0]) - 1] == letter and password[int(num[1]) - 1] != letter):
            counter += 1
        elif (password[int(num[0]) - 1] != letter and password[int(num[1]) - 1] == letter):
            counter += 1

    return counter
        
    
# input in lines
wholeInput = sys.stdin.read()
lst = wholeInput.split("\n")

print(a(lst))
print(b(lst))
