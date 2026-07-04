# CONTEXT.md — Hackathon Dzień 4 (kontekst techniczny dla agenta Claude Code)

> Ten plik jest źródłem prawdy o stacku technicznym, architekturze i podejściu do AI/evals z warsztatów dnia 4.
> Agencie: nie wymyślaj bibliotek, wzorców ani nazw od nowa — trzymaj się tego, co poniżej. Jeśli czegoś brakuje, powiedz o tym, nie zgaduj.
> Uzupełnia CONTEXT dnia 1 (proces produktowy: discovery → persona → MVP → backlog). Ten plik = warstwa implementacyjna.

---

## 1. Stack techniczny (NumpStack)

**NX + Angular + NestJS**, TypeScript end-to-end, monorepo.

- **NestJS** — backendowy framework TS, wzorowany na Angularze (moduły, kontrolery, providery), z **dependency injection**.
- **Angular** — frontend. Uwaga środowiskowa: **NX 23 wspiera Angular 21.2**; nie da się „wcisnąć" Angulara 20.
- **NX** — zarządzanie monorepo: generatory, tagi, skrypty, granice modułów (`NX_MODULE_BOUNDARIES`). Planować zależności front/back od startu, by uniknąć spaghetti.
- **Baza:** Postgres (produkcyjnie) / **SQLite** (lokalnie, szkoleniowo) / opcjonalnie JSON-server jako prosty backend testowy.
- **Docker / Docker Compose** — reprodukowalne środowisko (szczególnie lab bazodanowy).
- **OpenAPI / Swagger** (w materiałach „sloger"/„Cogen") — dokumentacja i interaktywne testowanie endpointów bez Postmana.

### Organizacja monorepo

- Appy frontendowe + backendowe + biblioteka współdzielona. Angular zależny od biblioteki; API niekoniecznie.
- Repo labowe: ~9 branchy; kluczowe branche do pracy + skrypty startowe (np. `npm run start:state` uruchamia Angular + backend).
- Pierwsze zadania: pobrać repo → `npm install` → uruchomić i sprawdzić, że działa. Wczesna nauka przez kopiowanie kodu (zrozumienie struktury), potem implementacja logiki.
- Troubleshoot `npm install`: usunąć zmodyfikowane node_modules / złe ścieżki do JSON-a i zainstalować ponownie; pilnować wersji (patch/minor) w package.json.

---

## 2. Frontend — Angular (wzorce)

### Struktura komponentów

- **Page components** — powiązane z routingiem, grupują smart komponenty.
- **Smart components** — pobierają dane, wstrzykują serwisy, wykonują requesty (REST/GraphQL). Delegują logikę do serwisów (fasady); **nie** trzymać logiki domenowej w kontenerach.
- **Dummy components** — prezentacja, komunikacja przez **@Input / @Output** (sygnały/observables).

### Template i binding

- `templateUrl` (plik HTML) **albo** inline template — tylko jedno.
- Trzy rodzaje bindingów: **property, two-way, event**.
- Change detection: **OnPush** dostępny domyślnie (kwestia wydajności/renderowania — temat rozbudowany, m.in. nowe sygnałowe API/formularze).

### DI i providery

- Rejestracja providera w module/komponencie → singleton dla danego zakresu.
- Provider w komponencie żyje i ginie z instancją komponentu (np. przy `@if`/`@switch`).
- Nie wszystko musi być w routingu — stan można trzymać w serwisach.

### HTTP client — jedno centralne miejsce (single source of truth)

- **`provideHttpClient`** globalnie z interceptorami.
- **Interceptor** = ostatni middleware przed wysłaniem requestu; globalnie modyfikuje requesty: dodaje token, zmienia URL, obsługa **refresh tokenów**, SSO. Pozwala dynamicznie przełączać providery danych (lokalny page API vs remote API).
- **App-initializer** — ładuje konfigurację **w runtime** (np. JSON w `public/`): URL-e API, SSO, domyślny język. Można podmienić po starcie (nie kompilowane jak environment).
- HTTP client zwraca **Observable (stream)** → pamiętać o subskrypcjach i zarządzaniu żywotnością.
- **Jeden centralny wrapper HTTP** opakowujący metody, data objects i mapowanie. Nie rozpraszać klientów HTTP po modułach domenowych.

---

## 3. Backend — NestJS (wzorce)

### Bloki podstawowe

- `main.ts` (bootstrap) → app module → moduły domenowe (user/order).
- **Moduł** grupuje: **controller** + **service (provider/injectable)**.
- **Controller** — dekoratory mapujące URL i metody HTTP (GET/POST/PUT/PATCH/DELETE).
- **Provider/Service** — logika biznesowa (analogia do serwisów Angulara).
- **Middleware** — logowanie, autoryzacja (przed/po requeście).
- **Guardy** — weryfikacja dostępu przed zwróceniem danych.

### DTO i serializacja (ważne)

- **Nie serializować encji DB bezpośrednio** — encja ma pola implementacyjne/proxy ORM → problemy przy serializacji i wyciek danych.
- **DTO (Data Transfer Object)** definiuje co udostępniamy klientowi i co walidujemy.
- Przy patch/update: **opcjonalne właściwości (Partial)**, kontrolować które pola są modyfikowalne (np. pomijać `ID`).
- Wbudowany serializer JSON w Nest eliminuje część problemów serializacji.

### Walidacja, CORS, dokumentacja

- **Global ValidationPipe** — spójne typowanie i uproszczone błędy.
- **CORS** — whitelistować host frontendu (np. `localhost:4200`), bo front i back działają lokalnie na różnych portach.
- **Swagger/OpenAPI** — interaktywna dokumentacja, `execute` bez Postmana.

### REST / HTTP

- REST = bezstanowy dostęp do zasobów pod URL-ami. Metody: GET (pobieranie), POST (tworzenie), PUT (zamiana całości), PATCH (częściowa), DELETE.
- Kody: 4xx (błędy klienta: 403, 405…), 5xx (błędy serwera).

---

## 4. Baza danych i ORM

### Docker Compose

- Baza uruchamiana **jedną komendą**. `init` folder wykonuje skrypty SQL po kolei — **prefiksy w nazwach plików** wymuszają kolejność (brak prefiksów → konflikty przy tabelach/constraintach).
- W compose: user, hasło, nazwa bazy, port (Postgres). **Wolumeny** przechowują dane i pliki init.
- Konfiguracje (porty, user, password, nazwa bazy) trzymać w `.env` (dobra praktyka).
- Race condition przy starcie lokalnym → checker konfiguracji bazy pomaga wykryć problem wcześnie. Poprawny komunikat backendu przy starcie = komunikacja z bazą działa.

### Schemat relacyjny (przykład Users → Orders)

- **1:N** — jeden user, wiele zamówień; zamówienie przypisane do jednego usera.
- Constraint w `orders` z kluczem obcym; **usunięcie usera kasuje jego zamówienia (cascade)**.
- Typy: int, varchar, primary key.

### ORM (wspomniane: TypeORM, Prisma, MikroORM, itd.)

- **`synchronize = false`** — preferowane; ręczna kontrola migracji/mapowania modeli.
- Modele przez dekoratory (`@Table` itd.), jawne nazwy tabel odpowiadające Postgresowi.
- **Rejestrować model w dwóch miejscach**: w module domenowym i w module głównym (dostępność globalna).
- Timestampy: ustawić tak, by ORM nie dodawał automatycznie kolumn (zapobiega błędom serializacji).
- Relacje: trzeba **jawnie `include`** powiązane tabele (np. user z orders).
- Operatory zapytań: `like` z wariantami case-insensitive; metody find/create/update/delete przez ORM.
- **Async wszędzie**: serwisy i kontrolery obsługują asynchroniczność (`async`/`await`).
- Mapowanie DB → warstwa aplikacji jawne; kontrolować które pola usera się udostępnia (serializacja/bezpieczeństwo).

---

## 5. AI: prompt engineering i kontekst

### Kontekst: statyczny vs dynamiczny

- **Statyczny** — stałe system-instructions, ogólne zasady; zawsze w promptach. Trzymać główne zasady w **jednym pliku** (np. Agent.md), symlink/platform dla różnych narzędzi.
- **Dynamiczny** — narzędzia, skille, wyniki retrievera, pamięć sesyjna; dodawane ad-hoc wg tasku.
- Cel podziału: oszczędność tokenów + lepsza kontrola zachowań.

### Prompting

- **Zaczynać od prostej, generycznej instrukcji**, potem rozbudowywać (refactor).
- **Zero-shot** (bez przykładu) / **one-shot** (1 przykład) / **few-shot** (pozytywny + negatywny przykład).
- Reguła: instrukcja + przykłady (pozytywne **i** negatywne) — żeby model się uczył, nie kopiował.
- **Limit ~120 instrukcji** w prompcie to orientacyjna granica; zbyt długi prompt przeładowuje model i zżera tokeny.

### Pamięć projektowa (second brain)

- Budować pamięć: **proceduralną, semantyczną, tymczasową**. Skille i prompty w rosnącej bazie wiedzy.

---

## 6. AI: agenty i architektura

- **Agent = orchestrator + dostęp do narzędzi** (retriever jako tool).
- **Multi-agent** — rozbić duże zadanie na role: **planer → implementer → verifier**. Każdy agent ma własny kontekst i model → mniej tokenów, lepsza kontrola.
- **Handoffy** — automatyczne przekazywanie specyfikacji między krokami tworzy flow (wymaga observability przekazywania kontekstu).
- **Pętla:** agent wykonuje → verifier testuje (evals, black-box) → przy błędach agent ponownie pobiera kontekst/dokumenty i poprawia.
- **Ryzyko: eksplozja tokenów**, gdy agent wielokrotnie pyta retrievera. Śledzić trace i optymalizować wywołania (metryki użycia tokenów). Wcześniejsze podejścia zużywały dziesiątki tysięcy tokenów — optymalizacja krytyczna.
- Cel docelowy: **autonomia end-to-end** (task solver bez ręcznej interwencji), stabilny i powtarzalny workflow.
- Skuteczność zależy **bardziej od kontekstu i harnessu niż od samego modelu** — dobry prompt/struktura potrafi podbić słabszy model.

### Narzędzia wspomniane (inspiracje, nie wymysły)

QMD (Spotify) do lokalnej wiki/chunkingu; LangChain Deep Agent jako wzorce agentów i pętli; DeepEval / Harbor / Benchmark Runner do evals i harnessów.

---

## 7. AI: Retrieval / RAG

- **RAG** = przed generacją doklejać wybrane, relewantne dokumenty.
- Bazy wektorowe + embeddingi → wybór najbardziej relewantnych ~1–10% dokumentów.
- **QMD / lokalna wiki** — chunking tematyczny, automatyczny RAG przez SQLite; proste „ragless" wdrożenie.
- **Precision / Recall / F1:**
  - Precision = ile ze zwróconego jest faktycznie relewantne.
  - Recall = ile z tego, co powinno być zwrócone, zostało zwrócone.
  - F1 = harmoniczna precision i recall.
- **Trade-off zależny od domeny**: restrykcyjny próg → precision rośnie (nawet 100%), recall/F1 spada. Domena medyczna → priorytet precision.

---

## 8. AI: Evals (Evals-Driven Development)

- **Eval to nie binarny pass/fail — to skala procentowa 0–100%** określająca zbliżenie do „złotej prawdy". Evals = „unit testy LLM", ale procentowe → dają pętlę poprawy kontekstu/promptu.
- Proste zero-jedynkowe oceny nie wystarczają → **skala (np. 100% / 75% / 0)** i metryki agregujące.
- **Test first:** używać evals od początku. Start od prostego testu → dopracować kontekst → dodać trudniejsze evals → zapętlić.
- **Mutation testing (analogia):** mutować wejścia i sprawdzać, czy system wychwytuje błędy — miara jakości samych evali.
- **Verifier / pętle weryfikujące** — deterministyczne sprawdzenia rezultatów; wykrywanie overfittingu.
- AI może pisać własne evalsy, ale **zalecane ręczne sprawdzanie** (halucynacje).
- **Benchmarking modeli:** zbudować stabilny zestaw ewaluacyjny → automatycznie oceniać nowe modele → decydować o migracji do tańszych/nowszych.
- Konfiguracja runnerów: flagi typu skip-screenshot / skip-lighthouse → minimalizować dodatkowe wywołania i koszt tokenów. Uwaga na **niedeterminizm** modeli (wyniki się wahają).

---

## 9. Ocena jakości kodu (whitebox → blackbox)

- **Brak jednej „złotej prawdy"** — kod da się zapisać milionami sposobów. Wybór **metryki determinuje optymalizację**; metryki służą porównaniu i priorytetyzacji.
- **Kolejność: najpierw whitebox, potem blackbox** — tak proces najlepiej się skaluje.
  - **Whitebox:** statyczna analiza, lintery, architektura, konwencje, testowalność, security, unit testy.
  - **Blackbox:** testy przez publiczne interfejsy (endpoints), performance, load — zachowanie systemu.
- Metryki/cechy: complexity, kohezja, coupling, liczba linii/funkcji/parametrów. Mniej parametrów + proste (monadyczne) funkcje bywa lepsze. Zbiór ~300 metryk → łatwiej orzec „kod jest dobry".
- **Startować od gotowych reguł** (np. linter zespołu Angulara) zamiast wymyślać nowe; przypisywać **high impact** kluczowym regułom, by ukierunkować ocenę.
- **DI: konstruktor vs `inject()`** — modele LLM widziały więcej przykładów z konstruktorem niż z nowym syntaksem; mogą nie znać najnowszego API.
- **„Ufaj, ale sprawdzaj"** — LLM nie zweryfikuje sam wszystkich reguł; bazuje na wcześniejszych przykładach/wagach. Nie mieszać frameworków (żadnego Reacta w Angularze).
