from pathlib import Path


def read_lines(p):
    for line in Path(p).read_text().split("\n"):
        yield line.strip()

