#!/usr/bin/env python3
"""Check Rust course organization, mastery contracts and key Markdown links.

Pure standard-library validator: no cargo dependencies, remote requests or code execution.
It validates repository-maintained content rather than learners' implementations.
"""
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
LEVELS = ("basics", "intermediate", "advanced")
FUNCTIONS = (
    "parse_port", "unicode_window", "mean_truncated", "word_frequency",
    "parse_settings", "by_initial", "normalize_ranges", "top_k_scores",
    "read_frames", "parallel_histogram", "apply_changes", "reserve_capacity",
)


def validate() -> list[str]:
    problems: list[str] = []
    for level in LEVELS:
        level_root = ROOT / level
        for folder, glob, expected in (
            (level_root, "workshop-*.md", 4),
            (level_root / "practice", "*.md", 8),
            (level_root / "projects", "*.md", 3),
        ):
            found = list(folder.glob(glob))
            if len(found) != expected:
                problems.append(f"{folder.relative_to(ROOT)}: expected {expected}, got {len(found)}")

    if len(list((ROOT / "lessons").glob("[0-9][0-9]-*.md"))) != 8:
        problems.append("expected 8 study-log lessons")

    for filename in ("mastery/starter.rs", "mastery/reference.rs"):
        text = (ROOT / filename).read_text(encoding="utf-8")
        found = re.findall(r"\bpub fn\s+([A-Za-z0-9_]+)", text)
        if len(found) != len(FUNCTIONS) or set(found) != set(FUNCTIONS):
            problems.append(f"{filename}: expected 12 matching exported mastery functions")

    test_file = (ROOT / "tests/mastery.rs").read_text(encoding="utf-8")
    if test_file.count("#[test]") != len(FUNCTIONS):
        problems.append("tests/mastery.rs: expected 12 distinct graded tests")
    for prefix in ("b", "i", "a"):
        if len(re.findall(rf"fn {prefix}\d\d_", test_file)) != 4:
            problems.append(f"tests/mastery.rs: expected four {prefix.upper()}xx tests")

    docs = [
        ROOT / name for name in ("README.md", "CURRICULUM.md", "ASSESSMENT.md", "PORTFOLIO.md")
    ]
    docs.extend(ROOT / name / "README.md" for name in LEVELS)
    docs.extend((ROOT / "mastery").glob("*.md"))
    for file in docs:
        if not file.exists():
            problems.append(f"missing {file.relative_to(ROOT)}")
            continue
        text = file.read_text(encoding="utf-8")
        for link in re.findall(r"(?<!!)\[[^\]]+\]\(([^)]+)\)", text):
            if "://" in link or link.startswith(("mailto:", "#")):
                continue
            target = link.split("#", 1)[0].split("?", 1)[0]
            if not target:
                continue
            dest = (file.parent / target).resolve()
            if not dest.is_relative_to(ROOT.resolve()) or not dest.is_file():
                problems.append(f"{file.relative_to(ROOT)}: missing link {link}")

    root_readme = (ROOT / "README.md").read_text(encoding="utf-8")
    for term in ("mastery", "168", "36", "12-week"):
        if term not in root_readme:
            problems.append(f"README.md: missing {term} summary")

    return problems


if __name__ == "__main__":
    failures = validate()
    if failures:
        print("Rust curriculum validation failed:")
        for problem in failures:
            print(" -", problem)
        sys.exit(1)
    print("Rust curriculum validated: 36 workshops, 9 projects, 8 study-log lessons,")
    print("12 mastery function contracts/tests and documented local links.")
