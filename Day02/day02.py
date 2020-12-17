

counter = 0
for i in range (0, 1000):
    inp = input()
    arr = inp.split(" ")
    num = arr[0].split("-")
    letter = arr[1][0]
    password = arr[2]

    if (password[int(num[0]) - 1] == letter and password[int(num[1]) - 1] != letter):
        counter += 1
    elif (password[int(num[0]) - 1] != letter and password[int(num[1]) - 1] == letter):
        counter += 1
print(counter)
    