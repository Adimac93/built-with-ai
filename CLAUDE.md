<!-- nx configuration start-->
<!-- Leave the start & end comments to automatically receive updates. -->

# General Guidelines for working with Nx

- For navigating/exploring the workspace, invoke the `nx-workspace` skill first - it has patterns for querying projects, targets, and dependencies
- When running tasks (for example build, lint, test, e2e, etc.), always prefer running the task through `nx` (i.e. `nx run`, `nx run-many`, `nx affected`) instead of using the underlying tooling directly
- Prefix nx commands with the workspace's package manager (e.g., `pnpm nx build`, `npm exec nx test`) - avoids using globally installed CLI
- You have access to the Nx MCP server and its tools, use them to help the user
- For Nx plugin best practices, check `node_modules/@nx/<plugin>/PLUGIN.md`. Not all plugins have this file - proceed without it if unavailable.
- NEVER guess CLI flags - always check nx_docs or `--help` first when unsure

## Scaffolding & Generators

- For scaffolding tasks (creating apps, libs, project structure, setup), ALWAYS invoke the `nx-generate` skill FIRST before exploring or calling MCP tools

## When to use nx_docs

- USE for: advanced config options, unfamiliar flags, migration guides, plugin configuration, edge cases
- DON'T USE for: basic generator syntax (`nx g @nx/react:app`), standard commands, things you already know
- The `nx-generate` skill handles generator discovery internally - don't call nx_docs just to look up generator syntax


<!-- nx configuration end-->

# Project Context

ng-diagram is an Angular library for creating interactive DOM-based diagrams in a monorepo managed by Turborepo.

- Diagrams are DOM-based
- Library complies with latest Angular standards
- Core and InputEventHandler are framework-agnostic and environment-agnostic (no browser dependencies)
- Core package must have no external dependencies
- Write all unit tests in Vitest

# Architecture

## ADR

- Create ADRs in `/docs/adr/{name}.md` for: major dependency changes, architectural pattern changes, new integration patterns

## Clean Architecture

- Strictly separate code into layers: entities, use cases, interfaces, and frameworks
- Dependencies point inward — inner layers have no knowledge of outer layers
- Domain entities encapsulate business rules without framework dependencies
- Use interfaces (ports) and implementations (adapters) to isolate external dependencies
- Use cases orchestrate entity interactions for specific business operations
- Mappers transform data between layers to maintain separation of concerns

# Documentation

## JSDoc

- Document all functions, classes, and methods with consistent JSDoc comments
- Use `@param`, `@returns`, `@throws` tags comprehensively
- Use `@example` tags with realistic usage scenarios for complex APIs
- Use `@typedef` for complex object structures when not using TypeScript

# Collaboration

## Expert Support Level

- Favor elegant, maintainable solutions over verbose code — assume understanding of idioms and design patterns
- Highlight potential performance implications and optimization opportunities
- Frame solutions within broader architectural contexts; suggest design alternatives when appropriate
- Focus comments on "why" not "what" — assume code readability through well-named identifiers
- Proactively address edge cases, race conditions, and security considerations
- When debugging, provide targeted diagnostic approaches rather than shotgun solutions
- Suggest comprehensive testing strategies including mocking, test organization, and coverage considerations