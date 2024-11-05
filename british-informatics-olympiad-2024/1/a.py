print("AmirMohammad Pirzadeh")
print("Poole highschool")

inp = input()
n, i = inp.split(" ")
n = int(n)
i = int(i)

lst = []
counter = int(n)
while (True):
    lst.append(str(counter))
    counter += 1
    if (len("".join(lst)) >= i):
        print("".join(lst)[i - 1])
        break