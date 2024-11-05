print("AmirMohammad Pirzadeh")
print("Poole highschool")

n = 1
i = 101

lst = []
counter = int(n)
while (True):
    lst.append(str(counter))
    counter += 1
    if ("11111" in "".join(lst)):
        print("".join(lst).find("11111") + 1)
        break
