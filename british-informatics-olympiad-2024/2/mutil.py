def ESet(i: int) -> int:
    return 2*i

def OSet(i: int) -> int:
    return 1 + 2*(i-1)

T = []
for i in range(2*60):
    for j in range(i):
        T.append(i)

def TSet(i: int) -> int:
    return T[i - 1]


def Combine(left, right):
    def GetI(i):
        return right(left(right(i)))
    
    return GetI