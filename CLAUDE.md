# CLAUDE.md — Build with AI Wrocław (hackathon)

## Co budujemy

**Uniwersalny** system dla działu R&D rozwiązujący dowolne problemy wynalazcze: strona z polem tekstowym, pod nią agentowy pipeline. Pipeline: **Problem → Kontradykcja → Kandydaci (≥3 TRIZ z matrycy kontradykcji + ≥3 SCAMPER) → Ewaluacja → Wybór**, z pełnym reasoning trailem. Problem 4 (wycieki ropy, SDG 14) to tylko scenariusz demo/eval — niczego nie hardkodować pod konkretny problem; kryteria ewaluacji generyczne + constrainty wyprowadzane z treści problemu.

**Reguła nadrzędna z zadania:** każdy krok musi działać jako realny, inspektowalny kawałek logiki — NIE jeden prompt udający strukturę. Kodem to, co policzalne (lookup w matrycy, argmax wyboru, walidacje); LLM tylko do subiektywnego (treść kandydatów, oceny jakości).

## Źródła prawdy (czytaj przed pracą, nie wymyślaj od nowa)

- `docs/task.md` — treść zadania i wymagania.
- `docs/plan.md` — plan implementacji, kamienie milowe M0–M5, kryteria sukcesu.
- `docs/graf-wiedzy.md` — grafy: pipeline, stack, mapa dokumentów, kryteria oceny.
- `docs/dzien1-produkt.md` … `dzien4-backend-llm.md` — konteksty warsztatowe (proces, UX/a11y, architektura frontu, backend/LLM/evals).
- `docs/Wskazowki do oceniania.md` — za co są punkty (Nx+warstwy, ewaluacja 50/50 inputy/metryki, full stack+deploy, pitch).

## Stack (nie zmieniać bez decyzji zespołu)

- **NX 23 monorepo, TypeScript end-to-end; Angular 21.2 (NX 23 wymusza wersję); NestJS.**
- Struktura: `apps/{api,cli,web}` + `libs/{domain,triz,methods,llm,evals}` — szczegóły w `docs/plan.md` §2.
- CLI przez `nest-commander` — te same serwisy co REST; kryterium oceny: „odpalam 1 komendą".
- Walidacja outputów LLM: `zod` na granicy każdego kroku.
- LLM: Gemini (`@google/genai`) za wrapperem w `libs/llm` (podmienialny na Claude).
- Wizualizacja trailu: `ng-diagram` + zewnętrzny auto-layout (dagre/elkjs — ng-diagram nie ma własnego).
- DB: SQLite lokalnie / Postgres produkcyjnie, Docker Compose, ORM z `synchronize=false`.

## Twarde zasady

- **Sekrety:** klucze API tylko w `.env` (nigdy w repo, promptach, logach). `.env.example` bez wartości.
- **Granice modułów:** tagi NX + depConstraints; domeny nie importują się nawzajem; `util` importuje tylko `util`; bez cykli.
- **Angular:** smart/dumb split, sygnały, OnPush, jeden centralny wrapper HTTP z interceptorami; żadnego Reacta.
- **NestJS:** DTO na granicy API (nigdy encje ORM), global ValidationPipe, Swagger, CORS whitelistowany.
- **Dostępność (minimum na hackathon):** semantyczny HTML → nawigacja klawiaturą → widoczny fokus → skip link → labelki formularzy. WCAG 2.1 AA jako cel.
- **Evals test-first:** eval powstaje razem z krokiem pipeline'u, nie po. Skala %, nie pass/fail.
- **Prompty do LLM po angielsku** (tańsze tokeny); dokumentacja i UI po polsku.
- Zmiany minimalne i chirurgiczne; najpierw najprostsze skuteczne rozwiązanie; przy niejasności — pytaj, nie zgaduj.

## Stan projektu (2026-07-04)

Faza planowania — brak kodu. Następny krok: M0 (setup workspace NX) wg `docs/plan.md` §5. Priorytet przy braku czasu: M1 silnik → M2 CLI/API → M3 evals, potem front i deploy.


<!-- nx configuration start-->
<!-- Leave the start & end comments to automatically receive updates. -->

## General Guidelines for working with Nx

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