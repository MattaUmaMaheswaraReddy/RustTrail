from concrete import fhe

def add(x, y):
    return x + y

compiler = fhe.Compiler(add, {"x": "encrypted", "y": "encrypted"})
inputset = [(2, 3), (0, 0), (1, 6), (7, 7), (7, 1), (3, 2), (6, 1), (1, 7), (4, 5), (5, 4)]

print(f"Compiling...")
circuit = compiler.compile(inputset)

print(f"Generating keys...")
circuit.keygen()

examples = [(3, 4), (1, 2), (7, 7), (0, 0)]
for example in examples:
    encrypted_example = circuit.encrypt(*example)
    encrypted_result = circuit.run(encrypted_example)
    result = circuit.decrypt(encrypted_result)
    print(f"Evaluation of {' + '.join(map(str, example))} homomorphically = {result}")