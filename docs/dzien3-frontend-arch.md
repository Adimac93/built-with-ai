# CONTEXT.md — Hackathon Dzień 3 (Scaling Frontend Architecture & AI-Assisted Debugging)

> Ten plik jest źródłem prawdy o architekturze frontendu, monorepo (NX), dostępności/CSS, wizualizacji (ng-diagram) i AI-wspomaganym debugowaniu z dnia 3.
> Agencie: nie wymyślaj narzędzi, nazw ani API od nowa — trzymaj się tego, co poniżej. Jeśli czegoś brakuje, powiedz o tym, nie zgaduj.
> Uzupełnia: CONTEXT dnia 1 (proces produktowy), dnia 2 (UI/UX, dostępność, design system + AI/Figma) i dnia 4 (backend/evals). Ten plik = warstwa skalowania architektury frontendu + debugowanie z AI.

**Kontekst wydarzenia:** Build with AI Wrocław. Dzień 3 (01.07) = „Scaling Frontend Architecture & AI-Assisted Debugging": NX monorepo → nowoczesny Angular (signals) → ng-diagram (diagramy natywnie w Angularze) → debugowanie z AI (Antigravity, Chrome DevTools MCP).

---

## 1. Główna filozofia dnia

1. **Granice modułów chronią przed spaghetti.** W miarę wzrostu monorepo, reguły importów (tagi, `depConstraints`) są ważniejsze niż dyscyplina „na słowo".
2. **Dumb/smart split + sygnały > mega-komponenty.** Rozbijać UI na małe, prezentacyjne komponenty; logikę trzymać w kontenerach/serwisach.
3. **Graf zależności to mapa — także dla AI.** Zarówno człowiek, jak i agent potrzebują grafu/metadanych projektu, żeby rozumieć duże monorepo, zamiast zgadywać strukturę.
4. **Dostępność i CSS to fundament, nie polish.** Landmarki, semantyka i flex/grid dobrze dobrane rozwiązują 80% problemów UX zanim w ogóle pomyślimy o AI.
5. **AI przyspiesza debug, ale zmiany „w przeglądarce" nie istnieją, dopóki nie trafią do repo.** Agent może zreprodukować bug i zaproponować fix — commit i weryfikacja to zawsze człowiek.

---

## 2. NX i modularność monorepo

### Typy projektów i siła tagów (od najsilniejszego)

- **application** — najsilniejszy tag, może importować (prawie) wszystko.
- **feature / app** — może importować prawie wszystko.
- **UI / data-access** — bardziej ograniczone.
- **util** — może być importowany przez innych, ale sam importuje tylko inne utile.

**Zasada:** jeśli moduł `util` (np. obiekt typu `price`) jest importowany z poziomu `feature`, a jednocześnie sam próbuje importować coś z powrotem z feature'a → **cykl zależności („site independency")**. Skutki złych importów: powtarzające się/większe paczki, spowolniona aplikacja, trudne do wykrycia błędy runtime.

### Module boundaries i depConstraints

- Konfiguracja w pliku eslint (depConstraints): definiuje **source tagi** i tablicę **`onlyDependsOn`** — co może importować co.
- Reguła: **jeśli projekt importuje moduł, staje się od niego zależny** — zmiana w module może zaszkodzić konsumentom.
- Walidacja reguł: **`nx-enforce-module-boundaries`**.
- Praktyczne zastosowanie na hackathonie: ocena „czystości architektury" (kryterium jakości).

### Graf zależności

- Uruchomienie grafu NX pokazuje: kierunki importów, potencjalne cykle, projekty dotknięte zmianą, obszary do „czyszczenia".
- **Kluczowe dla pracy z AI:** graf + metadane projektu (project JSON) pozwalają agentowi zrozumieć strukturę monorepo zamiast zgadywać. AI samo z siebie **nie podzieli projektu poprawnie bez tego kontekstu**.

### Komponowanie aplikacji Angular

- Rozbijać UI na **maksymalnie małe komponenty**; unikać „single component" z tysiącami linii.
- **Dumb (presentational)** — działa na inputach/outputach, bez logiki domenowej.
- **Smart (kontener)** — zarządza danymi, wstrzykuje serwisy, orkiestruje UI. (Spójne z zasadą smart/dummy z dnia 4.)
- **Standalone components** + lokalny change detection; inspiracja: RxAngular / Angular Signals.
- **RxJS** bywa przydatny, ale komplikuje debugowanie przy złożonych operatorach (`combineLatest`, `switchMap`, behavioral subjects) — zachować ostrożność.

### Domeny

- Domeny **nie powinny importować się nawzajem** (wyjątki rzadkie).
- **„Shared module"** (starsze podejście) = „worek" bez jasnej odpowiedzialności — unikać. Lepiej rozdzielać odpowiedzialność między domenami; monorepo dobrze wspiera spójność (style, accessibility, brand) globalnie.

### Cache i CI

- NX liczy **hashe inputów/outputów** — jeśli inputy się nie zmieniły, task jest pomijany (cache).
- **Remote cache (NX Cloud)** oszczędza czas przy wielu deweloperach; artefakty CI (np. GitLab) też mogą być wykorzystane.
- Źle zdefiniowane inputy/outputy → nadmierne przebudowy i błędy w pipeline (przykładowy efekt poprawy: **1.5–2h → 15 min**).
- Unikać ręcznego podbijania wersji w monorepo — korzystać z narzędzi NX (`nx release`) do migracji, wersjonowania i changeloga.
- Konfiguracje bazowe: `nx.json`, `tsconfig.base.json`, aliasy ścieżek.

---

## 3. Dostępność (accessibility) — praktyczne podstawy dnia 3

_(Uzupełnia sekcję dostępności z dnia 2 — tu nacisk na landmarki i nawigację techniczną.)_

- **Landmarki** — semantyczne regiony strony: `header`, `main`, `footer`, `nav`, `aside`. Czytnik ekranu widzi je jako grupy, użytkownik może między nimi „skakać".
- **Nagłówki jako checkpointy** — poprawna hierarchia H1–H6 pozwala czytnikom ekranu szybko przeskakiwać między sekcjami (klawisz **H**).
- **Skip links** — pomijają powtarzalne elementy, powinny być dopięte do landmarków (patrz też dzień 2).
- **`<article>`** — self-contained content, dobre dla komponentów zawierających samodzielną treść.
- Czytniki ekranu: **NVDA** (darmowy), **JAWS** (komercyjny), **VoiceOver** (macOS).
- Podejście pragmatyczne: zacząć od semantycznego HTML-a i prostych nagłówków, zamiast wrzucać same `div`/`span`. AI/LLM tylko jako wsparcie — trzeba wiedzieć co i jak poprawić ręcznie.

---

## 4. CSS: Flexbox vs Grid

- **Flexbox = 1-wymiarowy** (poziom albo pion). Użycie: wyrównania w linii, skalowanie elementów w rodzicu. Kluczowe właściwości: `align-items`, `gap`, `stretch`.
- **Grid = 2-wymiarowy** (wiersze + kolumny). Użycie: galerie, powtarzalne siatki (np. layout 3×3 jak Instagram). `repeat()`/`auto-fill` do prostych galerii.
- **Zasada:** używać `gap` zamiast liczenia marginesów na pierwszym/ostatnim elemencie.
- **Unikać** starych technik: `float`, `position: absolute/left/right`, ręczne marginesy — historyczne źródło problemów.
- **Responsywność:** projektować fluidowo, ustawiać **min/target/max** (np. `clamp()`) dla elementów (przyciski, kafelki).
- **Accessibility mobile:** strefy klikalne przycisków ≈ 44×44 px jako zalecenie.
- **Ostrzeżenie o AI:** AI potrafi pomóc w stylowaniu, ale generowane layouty mogą się „rozjechać" (brak precyzji responsywnej); AI miesza flex/grid i generuje nieoptymalne media queries — **nie ufać bez sprawdzenia**.

---

## 5. ng-diagram — natywne diagramy w Angularze

Biblioteka open-source do budowania interaktywnych diagramów (node/edge) natywnie w Angularze, wykorzystywana m.in. do organizacyjnych diagramów (orgchart), wizualizacji sieci, edytorów obwodów itp. Będzie używana na hackathonie.

### Model danych

- **Model** = kolekcja **nodów** i **edge'ów** (to dwie rzeczy, które się wyświetlają).
- **Node** ma unikalne **ID (string)**, pozycję, oraz pole **`data`**, które może zawierać dowolne dane odpowiadające domenie biznesowej (np. imię, rola, zdjęcie osoby).
- Domyślny szablon noda wyświetla tylko etykietę (label) — żeby pokazać własne dane, trzeba zdefiniować **custom node template**.

### Custom node template

- Trzeba stworzyć zwykły komponent Angular, który **implementuje interfejs node template'a**.
- Rejestruje się mapowanie (słownik) **typ node'a → komponent Angular** — działa podobnie jak selektory w samym Angularze (dopasowanie i wstawienie odpowiedniej klasy).
- Trzeba dopiąć ten szablon do modelu, inaczej ng-diagram nie wie, jak wyrenderować nieznany typ danych.

### Porty i połączenia (edges)

- **Porty** to punkty (kółka) na nodzie, z których wychodzą połączenia — lepsze UX niż traktowanie całego node'a jako portu (łatwiej „złapać").
- Port ma **`type`**: `source`, `target` albo `both` — determinuje kierunek połączenia.
- Domyślne funkcje noda: resize, obrót, łączenie edge'ami.

### Grupowanie i layout

- **Nody można grupować** — grupa zachowuje się jak „pudełko"; przesunięcie grupy przesuwa wszystkie dzieci.
- **ng-diagram nie oferuje domyślnego automatycznego layoutu** (auto-pozycjonowania) — trzeba użyć zewnętrznych pakietów npm z algorytmami grafowymi lub policzyć samemu.
- Przykład algorytmu: **force-directed** (nody odpychają się jak elektrony; połączenia działają jak sprężyny wg prawa Hooke'a; pozycje wynikają z zbalansowania energii układu) — używany np. w grafie zależności NX.

### AI i MCP dla ng-diagram

- Istnieje **serwer MCP dla ng-diagram** (paczka npm) — po prostu dodać konfigurację MCP JSON w swoim IDE/agencie (np. VS Code + Claude).
- MCP pozwala agentowi AI **łączyć się z dokumentacją ng-diagram** (dużo przykładów i opisów funkcjonalności) — warto z tego korzystać zamiast pisać wszystko ręcznie od zera.
- Rekomendacja prowadzącego: poznać podstawy manualnie (skąd się co bierze), a potem swobodnie korzystać z AI/MCP do przyspieszenia pracy — „żeby AI nas nie połamało".

### Setup projektu (skrót)

1. `npm install` biblioteki ng-diagram (zwykła paczka npm).
2. Zaimportować style CSS ng-diagram.
3. Wygenerować komponent Angular CLI (`ng generate component`) i wstawić w nim tag `<ng-diagram>` z przekazanym modelem (nody + edges) jako input.
4. Dla własnych typów danych — dodać custom node template + zarejestrować mapowanie typ→komponent.

---

## 6. AI-assisted Web Debugging

### Filozofia

- Większość profesjonalnej pracy to **utrzymanie i debugowanie istniejącego kodu**, nie pisanie greenfield frameworków od zera.
- Nowa kluczowa umiejętność: **rozumieć i debugować kod wygenerowany przez AI/agenta**, nie tylko pisać własny.
- AI przyspiesza produkcję kodu, ale **człowiek odpowiada za poprawność i stabilność**.

### Chrome DevTools — funkcje AI (trzeba włączyć w ustawieniach)

- **Console Insights** — wyjaśnia komunikaty w konsoli i możliwe przyczyny błędu.
- Wsparcie AI dla stylowania, network requests, performance, sugestii plików.
- **Auto-annotations** — automatyczne nazwy elementów performance.
- Sugestie kodu w edytorze/terminalu.
- Uwaga: dostępność funkcji może zależeć od regionu/języka konta.

### Platformy agentowe: Antigravity + MCP

- **Antigravity** — platforma agentowa (Command Center, IDE-like widok, CLI, SDK) — rekomendowana przez prowadzącą.
- **Plan mode** — agent iteracyjnie planuje, zadaje pytania doprecyzowujące, poprawia się w pętli.
- **MCP (Model Context Protocol) / Chrome DevTools MCP** — daje agentowi „oczy, uszy i ręce": nawigacja, emulacja urządzeń, klikanie, wpisywanie tekstu, upload plików, zrzuty ekranu, uruchamianie Lighthouse i performance trace'ów.
- Agent może uruchomić **świeżą, czystą instancję przeglądarki** (bez rozszerzeń, bez cache) — dla bezpieczeństwa i powtarzalności reprodukcji buga.

### Zasady bezpieczeństwa

- Zawsze pracować pod **kontrolą źródła (source control)** — historia i wersjonowanie to podstawa bezpiecznej pracy z agentem.
- **Nigdy nie eksponować sekretów / zmiennych środowiskowych agentowi.** Agent może przez autocomplete „podpowiedzieć" wyciek danych uwierzytelniających — chronić tokeny/klucze.
- Do pracy produkcyjnej: wymagać **dobrych testów**, czytelnych wymagań w prostym języku (plain-English) i guardaili bezpieczeństwa, zanim agent dostanie prawo modyfikować kod produkcyjny.

### Ograniczenia pokazane w demo

- Zmiany agenta wykonane bezpośrednio w przeglądarce **nie są trwałe** — znikają po odświeżeniu. **Człowiek musi scommitować fix do repozytorium.**
- Wyniki bywają **niedeterministyczne** — czasem trzeba powtórzyć próbę lub iterować w plan mode.

### Praktyczny workflow reprodukcji buga (demo)

1. Agent otwiera świeżą instancję przeglądarki, nawiguje na stronę/aplikację.
2. Reprodukuje zgłoszony błąd (np. przycisk nic nie robi po kliknięciu).
3. Robi zrzuty ekranu, tworzy raport reprodukcji.
4. Proponuje fix (kod/CSS).
5. Człowiek weryfikuje i commituje poprawkę do repo.

### Lab „Bug Museum"

- Aplikacja demo z ~15 zadaniami/bugami w kategoriach: **Accessibility, Performance, CSS, JavaScript, Network** i inne.
- Setup: sklonować repo (Discord/GitHub) → `npm install` → `npm run dev` → otworzyć localhost.
- Zadanie: wziąć przydzielony bug, zreprodukować go z Chrome DevTools/agentem (Antigravity lub DevTools MCP), wdrożyć **trwały** fix w repozytorium.
- Rekomendowane narzędzia: Antigravity, Chrome DevTools (z włączonymi funkcjami AI), Modern Web Guidance, wtyczka Chrome DevTools MCP.
