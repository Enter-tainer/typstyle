#!/usr/bin/env python
import os


def process_files(directory):
    import_list = []
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith(".typ"):
                file_path = os.path.join(root, file)
                with open(file_path, "r", encoding="utf-8") as f:
                    for line in f:
                        line = line.strip()
                        if line.startswith('#import "@preview'):
                            import_list.append(line)
    with open("output.typ", "w", encoding="utf-8") as f:
        for line in import_list:
            f.write(line + "\n")


if __name__ == "__main__":
    directory = "./"  # replace with your directory
    process_files(directory)
