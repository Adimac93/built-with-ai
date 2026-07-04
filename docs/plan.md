# Plan implementacji — Inventive Problem Solving System (uniwersalny)

> System jest **uniwersalny**: wejściem jest dowolny problem wynalazczy wpisany w pole tekstowe (UI) lub podany plikiem (CLI). Problem 4 (Oil Spills, SDG 14) to tylko scenariusz demo/eval — nic w silniku nie może być pod niego zahardkodowane.

> Plan przed kodem (zasada z dnia 2: plan developmentu w repo _przed_ implementacją).
> Graf zależności i stacku: `docs/graf-wiedzy.md`. Zadanie: `docs/task.md`.

---

## 0. Założenia (jawne — do weryfikacji z zespołem)

1. **Metody generowania (decyzja z 2026-07-04): dokładnie dwie — TRIZ + SCAMPER, obie zawsze w jednym przebiegu** (zadanie wymaga min. 2, jedną MUSI być TRIZ). Analiza morfologiczna usunięta z zakresu. SCAMPER: 7 deterministycznych operatorów → łatwo pokazać „realny, inspektowalny krok logiki" zamiast jednego prompta; wymienny moduł w `apps/api/src/pipeline`.
2. **LLM = Gemini** (`@google/genai`) za cienkim wrapperem providera — event Google, darmowe tokeny. Podmiana na Claude to zmiana jednego adaptera.
3. **Silnik żyje w NestJS**, CLI (`nest-commander`) i REST wywołują ten sam serwis pipeline'u — zero duplikacji, a kryterium D4 („odpalam 1 komendą") spełnione od razu.
4. **Matryca kontradykcji TRIZ (39×39 → zasady 1–40) jako statyczny JSON w repo.** Lookup to czysty kod z unit testami — dokładnie to, co ocenianie D4 nazywa „kod tam, gdzie odpowiedź jest znana".
5. Prompty do LLM **po angielsku** (dzień 2: polskie prompty kosztują więcej tokenów); UI i dokumenty po polsku.

## 1. Kryteria sukcesu (mierzalne)

- [ ] `npx nx run cli:solve --input <dowolny-problem.md>` zwraca pełny reasoning trail JSON bez błędu (próg oceniania D4, część 1); demo na `problems/oil-spills.md`.
- [ ] Silnik jest uniwersalny: żaden prompt, kryterium ani stała nie odwołuje się do konkretnego problemu z siódemki.
- [ ] Trail zawiera 5 kroków: problem, kontradykcja, wszyscy kandydaci (≥3 TRIZ + ≥3 SCAMPER), ewaluacja, wybór — każdy jako osobny artefakt ze schematem zod.
- [ ] Runner evali przechodzi po pliku scenariuszy (7 problemów + ≥3 trudne przypadki) i liczy metryki automatycznie (D4, część 2).
- [ ] Metryki kodowe: parametry kontradykcji ∈ 1–39, każdy kandydat TRIZ traceowalny do zasady z lookupu, wybór == argmax ocen. LLM-judge tylko do jakości pomysłów.
- [ ] `nx graph` pokazuje czyste granice (tagi + depConstraints, bez cykli) — kryterium D3.
- [ ] UI: Angular renderuje trail jako graf w ng-diagram; przechodzi minimalną checklistę a11y z dnia 2 (semantyczny HTML, fokus, skip link, labelki).
- [ ] Deploy w chmurze uruchamialny przez jury (D5).

## 2. Architektura monorepo

```
apps/
  api       — NestJS: moduł pipeline + REST + Swagger
  agent     — agent uruchamiany osobno i wołany przez api
  frontend  — Angular 21.2: pole tekstowe na problem → trail (ng-diagram)
```

Reguły: domeny nie importują się nawzajem; `util` importuje tylko `util`; jeden wrapper HTTP po stronie Angulara; DTO zamiast encji na granicy API; sekrety tylko w `.env`.

## 3. Pipeline (każdy krok = osobny serwis z własnym artefaktem)

| Krok | Logika | Kod czy LLM |
| --- | --- | --- |
| 1. Problem | intake, normalizacja, zapis wejścia | kod |
| 2. Kontradykcja | problem → para parametrów TRIZ (improving/worsening) + uzasadnienie | LLM → walidacja zod + zakres 1–39 kodem |
| 2a. Lookup | para parametrów → zasady wynalazcze z matrycy | **kod** (JSON, unit testy) |
| 3a. Kandydaci TRIZ | ≥3: jeden kandydat na zwróconą zasadę | LLM per zasada; kod pilnuje traceability |
| 3b. Kandydaci SCAMPER | ≥3: iteracja operatorów S-C-A-M-P-E-R | kod iteruje, LLM generuje treść |
| 1a. Kryteria | z treści problemu LLM wyprowadza constrainty (np. dla Problemu 4: „transport as-is") → rubryka = kryteria generyczne (rozwiązuje kontradykcję, wykonalność, koszt/skala) + wyprowadzone constrainty | LLM → walidacja zod; rubryka zapisana w trailu |
| 4. Ewaluacja | ocena 0–100 + uzasadnienie per kandydat×kryterium wg rubryki z kroku 1a | LLM (subiektywne) |
| 5. Wybór | argmax po zagregowanym score | **kod** |
| Wyjście | reasoning trail: JSON ze wszystkimi artefaktami + zużyciem tokenów | kod |

## 4. Evals (plan pod ocenianie D4, 50/50)

- **Inputy (50%):** scenariusze eval w repo — 7 problemów z zadania + trudne przypadki: wejście niejasne, śmieciowe, problem spoza siódemki. Format: pliki tekstowe + manifest JSON.
- **Metryki (50%):** runner (`nx run cli:eval`) odpala pipeline po całej liście i liczy:
  - kodem: kompletność trailu (schema), poprawność lookupu (znane pary → znane zasady), liczność kandydatów, wybór == max, zachowanie na śmieciowym wejściu (czytelny błąd, nie crash);
  - LLM-judge (skala %, nie pass/fail — dzień 4): sensowność kontradykcji, jakość kandydatów.
- Test first: pierwszy eval powstaje razem z krokiem 2, nie na końcu.

## 5. Kamienie milowe

```
M0 Setup           → NX 23 workspace, apps/libs, tagi, lint, .env.example
                     weryfikacja: nx graph czysty, `nx run-many -t lint test` zielone
M1 Silnik          → kroki 1–5 + trail JSON + matryca TRIZ z testami
                     weryfikacja: CLI zwraca pełny trail dla Problemu 4
M2 CLI + API       → `solve`/`eval` w nest-commander, REST + Swagger
                     weryfikacja: 1 komenda = pełna odpowiedź; Swagger `execute` działa
M3 Evals           → scenariusze + runner + metryki kodowe + LLM-judge
                     weryfikacja: raport metryk po pełnej liście inputów
M4 Frontend        → Angular: formularz + trail w ng-diagram (dagre/elkjs layout), a11y-checklista
                     weryfikacja: klawiatura end-to-end, fokus widoczny, skip link
M5 Baza + deploy   → SQLite/Postgres (historia uruchomień), Docker Compose, deploy chmurowy
                     weryfikacja: jury otwiera URL i uruchamia problem
```

Kolejność priorytetów przy braku czasu: **M1 → M2 → M3** (bez działającego silnika i evali nie ma punktów D4), potem M4, na końcu M5-deploy.

## 6. Poza zakresem (świadomie)

- Multi-agent/handoffy, RAG po raportach SDG — dopiero gdy M1–M3 działają (dzień 1: najprostsze skuteczne rozwiązanie; „use of web-search is appreciated" = nice-to-have).
- Rozbudowany design system — minimalne tokeny wg scenariusza A z dnia 2.
- Auth/SSO — brak w wymaganiach.
