from pathlib import Path


def read_lines(p):
    for line in Path(p).read_text().split("\n"):
        yield line.strip()


def read_paragraphs(lines):
    result = []
    for line in lines:
        if not line:
            yield result
            result = []
        else:
            result.append(line)

