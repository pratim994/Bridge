
# Codex Development Workflow

## General Rule

Work incrementally.

Do not rewrite large portions of the frontend when a targeted change is sufficient.

## Before Coding

For every non-trivial task:

1. Inspect the relevant source.
2. Read the relevant documentation.
3. Identify existing abstractions.
4. Determine affected files.
5. Check whether tests already exist.

## During Coding

Prefer:

- small changes;
- existing patterns;
- explicit types;
- deterministic logic;
- reusable components.

Avoid:

- unnecessary dependencies;
- speculative architecture;
- duplicated state;
- unrelated formatting changes;
- unrelated refactors.

## Testing

Implement or update tests when changing game-state behavior.

Run the relevant validation commands before declaring completion.

## When Tests Fail

Do not simply remove or weaken a failing test.

Instead:

1. inspect the failure;
2. determine whether the implementation or test is wrong;
3. fix the underlying issue;
4. rerun the test.

## When Requirements Conflict

Stop and report the conflict if:

- documentation conflicts with code;
- a requested feature requires backend changes outside scope;
- a protocol is undefined;
- hidden information would need to be exposed;
- the requested architecture conflicts with an existing accepted ADR.

## Final Response

Report:

### Summary

What was implemented.

### Files

Files added/modified.

### Validation

Commands run and their results.

### Notes

Important assumptions or remaining work.

### Documentation

Any documentation updated.
