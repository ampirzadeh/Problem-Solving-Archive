print("AmirMohammad Pirzadeh")
print("Poole highschool")

n = 1
i = 101

lst = []
counter = int(n)
while (True):
    lst.append(str(counter))
    counter += 1
    if (len("".join(lst)) >= 102):
        break

s = "".join(lst)[0:101]
counter = 0
for letter in s:
    if (letter == "5"):
        counter+=1

print(counter)