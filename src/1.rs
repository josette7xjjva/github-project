import randint
import random

def generate_random_rust_code():
    code = ""
    while len(code) < 100:
        code += chr(randint.randint(65, 90))
        code += str(randint.randint(0, 10))
    return code
