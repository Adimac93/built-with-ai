---
name: frontend-angular
description: >
  Use when writing Angular TypeScript or HTML template files. Covers modern Angular patterns:
  standalone components, signals, inject(), control flow syntax, and lazy loading.
---

## Angular Coding Standards

- Use standalone components, directives, and pipes instead of NgModules
- Implement signals for state management instead of traditional RxJS-based approaches
- Use the `inject()` function instead of constructor injection
- Use control flow with `@if`, `@for`, `@switch` instead of `*ngIf`, `*ngFor`, `*ngSwitch`
- Leverage functional guards and resolvers instead of class-based ones
- Use deferrable views (`@defer`) for improved loading states
- Implement `OnPush` change detection strategy for improved performance
- Use TypeScript decorators with explicit visibility modifiers (`public`, `private`)
- Leverage Angular CLI for schematics and code generation
- Implement proper lazy loading with `loadComponent` and `loadChildren`
