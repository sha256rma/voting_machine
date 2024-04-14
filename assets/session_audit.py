import numpy as np
import json


def create_A():
    # Creating a random 8x8 matrix
    A = np.random.randint(0, 2, size=(8, 8))

    while np.linalg.det(A) == 0:
        A = np.random.randint(0, 2, size=(8, 8))

    return A


def G(s, A):
    return np.dot(A, s) % 2


A = create_A()
# print("Session Id Generator:\n")
generator = "".join(["".join(map(str, row)) for row in A])

# Creating a random binary vector s with 8 entries
s = np.random.randint(2, size=8)

Gs = G(s, A)


print("\nSession Id:", end=" ")
s_id = "".join(map(str, Gs))
binary_string = generator + s_id
# Convert binary string to integer
binary_integer = int(binary_string, 2)

# Convert integer to hexadecimal
hexadecimal_string = hex(binary_integer)
print(hexadecimal_string)


def return_s(s):
    return s.astype(int).tolist()


result = return_s(s)
print(json.dumps(result))
