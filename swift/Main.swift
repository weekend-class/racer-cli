print("Calculator CLI")
print()

while let input = readLine() {
    guard input != "quit" else {
        break
    }
    
    print(input)
}