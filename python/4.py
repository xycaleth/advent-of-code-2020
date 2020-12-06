from collections import Counter
from pathlib import Path
import re


def get_passports():
    text = Path("../input4.txt").read_text()
    passports = []
    passport = {}
    for line in text.split("\n"):
        if not line:
            passports.append(passport)
            passport = {}

        for pair in line.split(" "):
            if pair:
                key, value = pair.split(":")
                passport[key] = value

    return passports


def has_needed_fields(passport):
    required = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"}
    remaining = required - set(passport.keys())
    return not remaining or remaining == {"cid"}


def is_valid_year(value, at_least, at_most):
    m = re.match(r"^([0-9][0-9][0-9][0-9])$", value)
    if not m:
        return False
    
    year = int(m[1])
    if year < at_least or year > at_most:
        return False

    return True


def has_valid_fields(passport):
    for field, value in passport.items():
        if field == "byr":
            if not is_valid_year(value, 1920, 2002):
                return False
        elif field == "iyr":
            if not is_valid_year(value, 2010, 2020):
                return False
        elif field == "eyr":
            if not is_valid_year(value, 2020, 2030):
                return False
        elif field == "hgt":
            if m := re.match(r"^([0-9][0-9][0-9])cm$", value):
                height = int(m[1])
                if height < 150 or height > 193:
                    return False
            elif m := re.match(r"^([0-9][0-9])in$", value):
                height = int(m[1])
                if height < 59 or height > 76:
                    return False
            else:
                return False
        elif field == "hcl":
            m = re.match(r"^#[0-9a-f]{6}$", value)
            if not m:
                return False
        elif field == "ecl":
            if value not in {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}:
                return False
        elif field == "pid":
            m = re.match(r"^[0-9]{9}$", value)
            if not m:
                return False

    return True


def is_valid_passport(passport):
    return has_needed_fields(passport) and has_valid_fields(passport)


def part1(passports):
    print(sum(1 for passport in passports if has_needed_fields(passport)))


def part2(passports):
    print(sum(1 for passport in passports if is_valid_passport(passport)))


passports = get_passports()
part1(passports)
part2(passports)
