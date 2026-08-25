# playlog-frontend

_Angular_ 22 aplikacija, stilizovana pomoću [Angular Material](https://material.angular.dev/) i sa
[Bootstrap](https://ng-bootstrap.github.io/)-om (samo CSS klase za raspored na stranici i veličinu elemenata).

_Package manager_: [bun](https://bun.sh/).

Korištena verzija _Node.js_: `24.16.0`

## Struktura projekta

```
src/
└── app/
    ├── core/               # AuthInterceptor (bearer token + refresh), AuthGuard, SessionService (signali), ApiError, PagedResponse
    ├── features/           # Stranice i komponente grupisane po funkcionalnostima, DTO i servisi
    │   ├── auth/
    │   ├── comments/
    │   ├── game-entities/
    │   ├── games/
    │   ├── home/
    │   ├── library/
    │   ├── navbar/
    │   ├── reports/
    │   ├── reviews/
    │   └── users/
    └── shared/             # Deljene komponente i korisni servisi
        ├── components/
        └── services/
```

Zaštićene rute koriste funkcionalne _guard_-ove (`CanActivateFn`) koji čitaju uloge iz `route.data['roles']`. Stanje
korisnika/tokena se nalazi u `SessionService` kao _Angular_ signali.

## Konfiguracija

Videti `.env.example` i `.env.production.example` za potrebne promenljive.

Pokrenuti komandu:

```bash
ng generate environments
```

Pogledati fajlove `environment.development.example.ts` i `environment.example.ts` za izgled `environment` promenljive.

## _Development server_

```bash
bun run start
```

Server je podrazumevano pokrenut na `http://localhost:4200/`. Aplikacija se automatski ponovo učitava kada god se
bilo koji od fajlova modifikuje.

## _Production build_

```bash
bun run build
```

Ovo kompajlira projekat i čuva _build_ artefakte u `dist/` direktorijumu. Podrazumevano, produkcioni _build_
optimizuje aplikaciju za performanse i brzinu.

Za _build_ u _Docker_-u, najlakše je pokrenuti komandu `make build-frontend` iz korena repozitorijuma.

Sa komandom `make start` pokreće se nginx server, koji servira aplikaciju na adresi `http://localhost:8080/`.

Za detalje pogledati nginx konfiguraciju u `nginx.conf`.

## _Lint_

```bash
bun run lint
```

Pokreće ESLint nad TypeScript i HTML izvornim fajlovima.