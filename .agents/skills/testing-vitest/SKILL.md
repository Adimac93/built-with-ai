---
name: testing-vitest
description: >
  Use when writing or configuring Vitest unit tests in TypeScript or TSX files.
  Covers mocking patterns, setup files, snapshots, coverage, and DOM testing.
---

## Vitest Guidelines

- Use `vi.fn()` for function mocks, `vi.spyOn()` to monitor existing functions, `vi.stubGlobal()` for global mocks
- Prefer spies over mocks when you only need to verify interactions without changing behavior
- Place `vi.mock()` factory functions at the top level — the factory runs before imports are processed
- Use `mockImplementation()` or `mockReturnValue()` for dynamic control during tests
- Define global mocks, custom matchers, and environment setup in setup files referenced in `vitest.config.ts`
- Use inline snapshots (`toMatchInlineSnapshot()`) for readable assertions visible in code review
- Configure coverage thresholds in `vitest.config.ts` only when asked; focus on meaningful tests over percentages
- Run `vitest --watch` during development for instant feedback; use `-t` to filter specific tests
- Use `vitest --ui` to navigate large test suites visually during development
- Set `environment: 'jsdom'` for frontend component tests; combine with testing-library for user interaction simulation
- Group related tests with `describe` blocks; follow Arrange-Act-Assert pattern
- Use `expectTypeOf()` for type-level assertions; ensure mocks preserve original type signatures
