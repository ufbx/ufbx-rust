import re

def read_file(path, fn, *args):
    print(f"[R] {fn.__name__}: '{path}'")
    with open(path, "rt", encoding="utf-8") as f:
        lines = (l.rstrip("\r\n") for l in f)
        return fn(lines, *args)

def modify_file(path, fn, *args):
    print(f"[W] {fn.__name__}: '{path}'")
    with open(path, "rt", encoding="utf-8") as f:
        lines = (l.rstrip("\r\n") for l in f)
        lines = list(fn(lines, *args))

    with open(path, "wt", encoding="utf-8") as f:
        f.writelines(f"{l}\n" for l in lines)

def parse_version(lines):
    for line in lines:
        m = re.search(r"ufbx-rust (\d+\.\d+\.\d+)", line)
        if m:
            return m.group(1)
    raise RuntimeError("version not found")

def update_cargo_version(lines, version):
    it = iter(lines)

    for line in it:
        if re.match(r"version\s*=\s*\".+\"", line):
            break
        yield line

    yield f"version = \"{version}\""

    yield from it

version = read_file("misc/changelog.md", parse_version)
modify_file("cargo.toml", update_cargo_version, version)
