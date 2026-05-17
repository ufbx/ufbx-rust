import re
import os
from dataclasses import dataclass

def read_file(path, fn, *args):
    print(f"[R] {fn.__name__}: '{path}'")
    with open(path, "rt", encoding="utf-8") as f:
        lines = (l.rstrip("\r\n") for l in f)
        return fn(lines, *args)

def modify_file(path, fn, *args):
    print(f"[M] {fn.__name__}: '{path}'")
    with open(path, "rt", encoding="utf-8") as f:
        lines = (l.rstrip("\r\n") for l in f)
        lines = list(fn(lines, *args))

    with open(path, "wt", encoding="utf-8") as f:
        f.writelines(f"{l}\n" for l in lines)

def write_file(path, fn, *args):
    print(f"[W] {fn.__name__} '{path}'")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    lines = fn(*args)
    with open(path, "wt", encoding="utf-8") as f:
        f.writelines(f"{l}\n" for l in lines)

@dataclass
class VersionInfo:
    ufbx_version: str
    versions: dict[str, str]
    changelog: str

def parse_version(lines):
    it = iter(lines)

    for line in it:
        if line.startswith("###"):
            break

    m = re.match(r"### v(\d+\.\d+\.\d+) .*", line)
    assert m

    ufbx_version = m.group(1)

    line = next(it)
    assert line.startswith("> ")

    versions = dict(re.findall(r"([a-z\-]+) (\d+\.\d+\.\d+)", line))

    changelog = []
    for line in it:
        if line.startswith("###"):
            break
        changelog.append(line.strip())

    while changelog and not changelog[0]:
        del changelog[0]
    while changelog and not changelog[-1]:
        changelog.pop()

    return VersionInfo(
        ufbx_version=ufbx_version,
        versions=versions,
        changelog=changelog,
    )

# -- Rust specific

def release_outputs(info: VersionInfo):
    yield f"ufbx_version={info.ufbx_version}"
    for repo, version in info.versions.items():
        key = repo.replace("-", "_") + "_version"
        yield f"{key}={version}"
    yield "changelog<UFBX_CHANGELOG_EOF"
    for line in info.changelog:
        yield line
    yield "changelog<UFBX_CHANGELOG_EOF"

def update_cargo_version(lines, version):
    it = iter(lines)

    for line in it:
        if re.match(r"version\s*=\s*\".+\"", line):
            break
        yield line

    yield f"version = \"{version}\""

    yield from it

info = read_file("misc/changelog.md", parse_version)
modify_file("cargo.toml", update_cargo_version, info.versions["ufbx-rust"])
write_file("temp/release.outputs", release_outputs, info)
