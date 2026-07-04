---
name: coding-practices-monorepo
description: >
  Use when creating a new package, changing workspace configuration, setting up shared tooling,
  or making decisions about monorepo structure and package boundaries.
---

## Monorepo Guidelines

- Configure workspace-aware tooling to optimize build and test processes
- Implement clear package boundaries with explicit dependencies between packages
- Use consistent versioning strategy across all packages (independent or lockstep)
- Configure CI/CD to build and test only affected packages for efficiency
- Implement shared configurations for linting, testing, and development tooling
- Use code generators to maintain consistency across similar packages or modules
