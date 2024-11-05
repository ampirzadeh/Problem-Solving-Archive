print("AmirMohammad Pirzadeh")
print("Poole highschool")

def find_score(s: str):
    score = 0
    for letter in s:
        score += ord(letter) - 64
    return score

print(find_score("BAB"))

def generate_list(score: int) -> str:
    for i in range(1, 26):
        

print(generate_list(5))
