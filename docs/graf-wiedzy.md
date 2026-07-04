# Graf wiedzy — Build with AI Wrocław (hackathon)

> Synteza plików z `docs/` w formie grafów (Mermaid). Źródła prawdy: `task.md`, `dzien1-produkt.md`, `dzien2-ux-ui.md`, `dzien3-frontend-arch.md`, `dzien4-backend-llm.md`, `Wskazowki do oceniania.md`, `harmonogramOceniania.md`.

---

## 1. Mapa dokumentów — co z czego wynika

```mermaid
flowchart TD
    TASK["task.md<br/>Problem 4: Oil Spills (SDG 14)<br/>TRIZ + druga metoda, ≥3+≥3 kandydatów,<br/>reasoning trail: Problem→Kontradykcja→Kandydaci→Ewaluacja→Wybór"]

    D1["dzien1-produkt.md<br/>Discovery → Persona → MVP → Backlog<br/>problem przed technologią, KPI, prostota"]
    D2["dzien2-ux-ui.md<br/>Dostępność (WCAG 2.1 AA), tokeny,<br/>design system jako source of truth,<br/>bezpieczeństwo kluczy"]
    D3["dzien3-frontend-arch.md<br/>NX monorepo, module boundaries,<br/>smart/dumb, ng-diagram, AI debugging"]
    D4["dzien4-backend-llm.md<br/>NestJS, DTO, ORM, prompting,<br/>multi-agent, RAG, EVALS"]
    OC["Wskazowki do oceniania.md<br/>Nx + clean code + warstwy (D3),<br/>ewaluacja 50/50 (D4),<br/>full stack + deploy (D5), pitch"]
    HO["harmonogramOceniania.md<br/>pitch + artefakty, 'every step must run<br/>as real, inspectable logic'"]

    D1 -->|jak zdefiniować problem i MVP| TASK
    D2 -->|jak zbudować dostępny UI| TASK
    D3 -->|jak zorganizować repo i wizualizować trail| TASK
    D4 -->|jak zbudować silnik LLM + evals| TASK
    TASK --> OC
    OC --> HO
```

---

## 2. Graf pipeline'u systemu (to, co budujemy)

Każdy węzeł = osobny, inspektowalny krok logiki (wymóg z `task.md` i `harmonogramOceniania.md`). Zasada z oceniania Dnia 4: **kodem to, co policzalne; LLM tylko do subiektywnego.**

```mermaid
flowchart TD
    IN["1. PROBLEM<br/>intake + normalizacja wejścia<br/>(DOWOLNY problem wynalazczy:<br/>pole tekstowe w UI lub plik w CLI)"]
    CON["2. KONTRADYKCJA<br/>LLM: problem → para parametrów TRIZ<br/>(improving / worsening, 1–39)<br/>walidacja kodem: zakres + schema"]
    MTX["Lookup w matrycy kontradykcji<br/>DETERMINISTYCZNY KOD<br/>matryca 39×39 jako JSON w repo<br/>→ zasady wynalazcze (1–40)"]
    TRZ["3a. KANDYDACI TRIZ (≥3)<br/>LLM: 1 kandydat na zasadę<br/>kod: traceability principle_id → wynik lookupu"]
    SC["3b. KANDYDACI metodą 2 — SCAMPER (≥3)<br/>deterministyczna iteracja 7 operatorów<br/>LLM: 1 kandydat na operator"]
    EV["4. EWALUACJA<br/>rubryka: kryteria generyczne<br/>+ constrainty wyprowadzone z treści problemu<br/>LLM: ocena + uzasadnienie per kryterium"]
    CH["5. WYBÓR<br/>DETERMINISTYCZNY argmax po score<br/>(kod, nie LLM)"]
    OUT["REASONING TRAIL<br/>ustrukturyzowany JSON<br/>z artefaktem każdego kroku"]

    IN --> CON --> MTX --> TRZ --> EV
    IN --> SC --> EV
    EV --> CH --> OUT
```

---

## 3. Graf stacku technicznego

```mermaid
flowchart TD
    subgraph MONO["NX 23 monorepo (TypeScript end-to-end)"]
        subgraph APPS["apps/"]
            WEB["web — Angular 21.2<br/>smart/dumb, signals, OnPush<br/>WCAG 2.1 AA, skip link"]
            API["api — NestJS<br/>moduły, DTO, ValidationPipe,<br/>Swagger/OpenAPI, CORS"]
            CLI["cli — nest-commander<br/>'odpalam 1 komendą' (kryterium D4)"]
        end
        subgraph API_MOD["apps/api/src/ (moduły NestJS)"]
            DOM["domain — typy, DTO,<br/>schematy zod reasoning trailu"]
            TRIZ["triz — matryca JSON<br/>+ deterministyczny lookup"]
            MET["methods — SCAMPER"]
            LLM["llm — cienki wrapper providera"]
            EVL["evals — scenariusze + metryki + runner"]
        end
    end
    DB[("SQLite lokalnie /<br/>Postgres produkcyjnie<br/>Docker Compose")]
    GEM["LLM API<br/>Gemini (@google/genai)<br/>ew. Claude (@anthropic-ai/sdk)"]
    NGD["ng-diagram<br/>wizualizacja trailu jako graf<br/>+ zewn. auto-layout (dagre/elkjs)"]

    WEB --> DOM
    WEB --> NGD
    API --> DOM
    API --> TRIZ
    API --> MET
    API --> LLM
    CLI --> API
    EVL --> API
    LLM --> GEM
    API --> DB
```

---

## 4. Technologie: wymagane vs proponowane

### Wynikające z materiałów (nie zmieniamy)

| Technologia | Skąd | Rola |
| --- | --- | --- |
| NX 23 | dzień 3/4 (kryterium oceny D3) | monorepo, tagi, module boundaries |
| Angular 21.2 | dzień 4 (NX 23 wymusza wersję) | frontend |
| NestJS | dzień 4 | backend/API, silnik pipeline'u |
| TypeScript | dzień 4 | end-to-end |
| ng-diagram | dzień 3 („będzie używana na hackathonie") | wizualizacja reasoning trailu (node/edge) |
| Postgres / SQLite | dzień 4 | historia uruchomień (prod / lokalnie) |
| Docker Compose | dzień 4 | reprodukowalna baza |
| Swagger / OpenAPI | dzień 4 | dokumentacja i testowanie API |
| TypeORM lub Prisma | dzień 4 (wspomniane) | ORM, `synchronize=false` |

### Proponowane dodatkowo (do akceptacji)

| Biblioteka | Po co | Uzasadnienie |
| --- | --- | --- |
| `zod` | walidacja outputów LLM na granicy każdego kroku | „every step must run as real, inspectable logic" — schema per krok, kod łapie halucynacje |
| `@google/genai` (Gemini) | klient LLM | event GDG/Google, darmowe tokeny; wrapper providera pozwala podmienić na Claude |
| `nest-commander` | CLI w tym samym kodzie co API | kryterium D4: „odpalam 1 komendą"; zero duplikacji silnika |
| `@dagrejs/dagre` lub `elkjs` | auto-layout grafu | dzień 3 wprost: ng-diagram **nie ma** auto-layoutu — potrzebna zewnętrzna paczka |
| `@nestjs/config` + `.env` | konfiguracja i sekrety | dzień 2/3: klucze API nigdy w repo ani w prompcie agenta |
| Jest/Vitest (default NX) | testy jednostkowe lookupu i selektora | deterministyczne kroki muszą mieć unit testy (whitebox przed blackbox, dzień 4) |

---

## 5. Graf kryteriów oceny → elementy systemu

```mermaid
flowchart LR
    K3["D3: Nx + clean code<br/>+ architektura warstwowa"] --> A["struktura apps/libs,<br/>tagi, depConstraints,<br/>smart/dumb, DTO"]
    K4["D4: Ewaluacja 20 pkt<br/>50% inputy / 50% metryki"] --> B["CLI + plik scenariuszy<br/>(7 problemów + edge case'y)<br/>+ runner metryk"]
    K5["D5: implementacja full stack<br/>+ deploy do chmury"] --> C["Angular + NestJS + DB,<br/>deploy testowalny przez jury"]
    PITCH["Pitch: pokaz działania,<br/>uzasadnienie decyzji"] --> D["demo trailu na żywo<br/>(CLI lub UI z ng-diagram),<br/>nie slajdy"]
```
