# Dry Run Test Report

**Date**: 2026-05-28T07:43:53Z

## Summary

- **Total tests**: 100
- **Passed**: 100
- **Failed**: 0
- **Known limitations**: 0
- **Effective pass rate**: 100.00%
- **Verdict**: `consistent`

## Per-Function Results

| Function | Total | Passed | Failed | Known | Rate |
|----------|-------|--------|--------|-------|------|
| elf_end | 100 | 100 | 0 | 0 | 100.00% |

## Per-Category Results

| Category | Total | Passed | Failed | Rate |
|----------|-------|--------|--------|------|
| boundary | 22 | 22 | 0 | 100.00% |
| fuzz | 59 | 59 | 0 | 100.00% |
| integration | 4 | 4 | 0 | 100.00% |
| mutation | 5 | 5 | 0 | 100.00% |
| normal | 6 | 6 | 0 | 100.00% |
| stress | 4 | 4 | 0 | 100.00% |

## Failed Tests

No failures. All tests passed.

## Sample Passed Tests (first 10)

| # | Name | Category | C ret | Rust ret |
|---|------|----------|-------|----------|
| 1 | `elf_end_null` | `boundary` | `0` | `0` |
| 2 | `elf_end_null_repeated` | `boundary` | `0` | `0` |
| 3 | `elf_end_ar_archive` | `normal` | `0` | `0` |
| 4 | `elf_end_elf64_basic` | `normal` | `0` | `0` |
| 5 | `elf_end_elf32_basic` | `normal` | `0` | `0` |
| 6 | `elf_end_elf64_with_sections` | `normal` | `0` | `0` |
| 7 | `elf_end_elf32_with_phdr` | `normal` | `0` | `0` |
| 8 | `elf_end_elf64_big_endian` | `boundary` | `0` | `0` |
| 9 | `elf_end_elf32_big_endian` | `boundary` | `0` | `0` |
| 10 | `elf_end_shared_obj_elf64` | `normal` | `0` | `0` |

## Layer 3 Coverage

### elf_end

- **Cases executed**: 100/100
- **Passed**: 100
- **Failed**: 0
- **Coverage score**: 1.000
- **Suggestions**:
  - Missing source encodings (tested 0/20)
  - Missing target encodings (tested 0/20)
  - No boundary value tests
  - No edge case tests
  - No error path tests executed

