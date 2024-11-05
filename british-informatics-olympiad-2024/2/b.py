print("AmirMohammad Pirzadeh")
print("Poole highschool")

from mutil import Combine, TSet

two_counter = 0
counter = 1
while (True):
    if (Combine(TSet, TSet)(counter) == 2):
        two_counter += 1
    if (Combine(TSet, TSet)(counter) == 3):
        print(two_counter)
        break
    counter += 1
