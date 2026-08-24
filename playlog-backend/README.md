# playlog-backend

_Rust workspace_ sa šest mikroservisa koje pokreće radni okvir _axum_.

## Struktura projekta

```
playlog-backend/
├── docker/                 # Docker infrastruktura - compose fajlovi (Postgres, MongoDB, MinIO, pgAdmin, Mongo Express)
├── keys/                   # RSA par ključeva za JWT
├── services/
│   ├── api-gateway/        # Port 3000 - ulazna tačka, JWT validacija, rutiranje
│   ├── catalogue-service/  # Port 3001 - katalog video igara
│   ├── library-service/    # Port 3002 - korisničke biblioteke igara
│   ├── multimedia-service/ # Port 3003 - slike i video sadržaj
│   ├── review-service/     # Port 3004 - recenzije, ocene, komentari i prijava neprimerenog sadržaja
│   └── user-service/       # Port 3005 - korisnički nalozi, autentifikacija, uloge
└── shared/
    ├── jwt-common/         # JWT enkodiranje/dekodiranje/validacija
    └── service-common/     # Deljeni kod za servise (ApiError, PagedResponse, boilerplate za inicijalizaciju itd.)
```

## Lokalni razvoj

Svaki servis (osim `api-gateway`) u svom direktorijumu ima sopstveni `Makefile` koji podiže samo njegove zavisnosti
(bazu + alate):

```bash
cd services/catalogue-service
make start-dev   # pokreće zavisnosti i čeka da budu 'healthy'
make start-db    # pokreće samo bazu
make logs-dev    # prati logove (opciono: make logs-dev <service>)
make stop-dev    # zaustavlja dev okruženje
make rebuild-dev # ponovo gradi Docker slike bez keširanja
```

Bazama podataka i alatima je moguće pristupiti na sledećim portovima:

| Servis             | Baza                           | Alati              |
|--------------------|--------------------------------|--------------------|
| user-service       | PostgreSQL 5433                | pgAdmin 5050       |
| catalogue-service  | PostgreSQL 5434                | pgAdmin 5050       |
| library-service    | PostgreSQL 5435                | pgAdmin 5050       |
| review-service     | MongoDB 27018                  | Mongo Express 8085 |
| multimedia-service | MongoDB 27019, MinIO 9000/9001 | Mongo Express 8085 |

> Napomena: portove za baze podataka je moguće otvoriti i u produkcijskom okruženju
> (pogledati glavni Makefile repozitorijuma).

Kada je _Docker_ infrastruktura podignuta, izabrani servis može da se pokrene iz korena _workspace_-a sa komandom:

```bash
cargo run -p catalogue-service
```

Konfiguracija servisa se čita iz `.env` fajla u direktorijumu svakog servisa (dev) ili `.env.production`.
Videti `.env.example` i `.env.production.example` za potrebne promenljive.

## Rad sa sqlx

Za pokretanje sqlx migracija, prvo je potrebno instalirati `sqlx-cli` sanduk: `cargo install sqlx-cli`.

Komanda za kreiranje baze (pokrenuti samo jednom): `sqlx database create`

Komanda za kreiranje migracije: `sqlx migrate add -r <name>`

Komanda za pokretanje migracija: `sqlx migrate run`

Kada se završe migracije, neophodno je pokrenuti komandu `cargo sqlx prepare` da bi se kompajlirali sačuvani sqlx makro
upiti. Pokrenuti je pri svakoj izmeni ili brisanju postojećih upita i pri dodavanju novih.

## JWT ključevi

Generisati RSA par ključeva pomoću:

```bash
openssl genpkey -algorithm RSA -out private.pem -pkeyopt rsa_keygen_bits:2048
```

```bash
openssl rsa -pubout -in private.pem -out public.pem
```

Ključeve sačuvati u `keys` direktorijumu ovog projekta.