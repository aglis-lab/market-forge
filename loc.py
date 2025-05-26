import os

def count_rust_loc(folder):
    total_loc = 0
    for root, _, files in os.walk(folder):
        for file in files:
            if file.endswith('.rs'):
                with open(os.path.join(root, file), 'r', encoding='utf-8') as f:
                    lines = [line for line in f if line.strip() and not line.strip().startswith('//')]
                    total_loc += len(lines)
    return total_loc

if __name__ == "__main__":
    loc = count_rust_loc(os.path.join(os.path.dirname(__file__), 'src'))
    loc += count_rust_loc(os.path.join(os.path.dirname(__file__), 'tests'))
    print(f"Total Rust LOC : {loc}")
