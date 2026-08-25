# scripts

Python skripte koje preuzimaju i učitavaju demo podatke. Podaci o igrama se dobijaju iz
[RAWG](https://rawg.io/) baze. Njihova API dokumentacija: https://api.rawg.io/docs/

> Ako Vam ne trebaju demo podaci, zanemarite ovaj direktorijum.

## Postavljanje

Projekat koristi [`uv`](https://docs.astral.sh/uv/) za upravljanje zavisnostima i pokretanje fajlova.

Minimalna verzija _Python_-a: `3.13`.

Instaliranje zavisnosti, kreiranje i aktivacija virtuelnog okruženja (`.venv`) se obavlja komandom:

```bash
uv sync
```

Konfiguracija se čita iz `.env` (pogledati `.env.example`). Obavezni parametri uključuju
`RAWG_API_KEY` i kredencijale/URI-je baza koje koriste pojedinačne skripte.

Neophodno je instalirati [ffmpeg](https://ffmpeg.org/download.html) da bi skripta `seed_media.py` mogla da se pokrene.

> Važna napomena: portovi baza moraju da budu vidljivi na host mašini.
>
> Za dodavanje demo podataka u produkcionom režimu, iskoristiti `make start-exposed` komandu u korenskom `Makefile`-u.

## Skripte

| Skripta             | Izvor podataka       | Ciljno odredište                   | Namena                                                                                                                           |
|---------------------|----------------------|------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
| `get_games.py`      | RAWG API             | `sql_seed/`, `media/`              | Preuzima 200 najpopularnijih igara (metapodaci + omoti/_screenshot_-ovi/_trailer_-i) u lokalne fajlove koje koriste seed skripte |
| `seed_catalogue.py` | `sql_seed/*.sql`     | catalogue-service PostgreSQL       | Učitava platforme, žanrove, programere, izdavače, tagove, igre i njihove relacije                                                |
| `seed_users.py`     | generisano           | user-service PostgreSQL            | Kreira demo korisnike (zajednička šifra: `password123`)                                                                          |
| `seed_library.py`   | users + catalogue DB | library-service PostgreSQL         | Dodeljuje igre korisničkim bibliotekama po kategorijama (owned, playing, wishlist, completed, dropped)                           |
| `seed_media.py`     | `media/`             | MinIO + multimedia-service MongoDB | Upload-uje omote/_screenshot_-ove/_trailer_-e i registruje njihove metapodatke                                                   |
| `seed_reviews.py`   | users + catalogue DB | review-service MongoDB             | Generiše recenzije i ocene                                                                                                       |
| `seed_comments.py`  | users + reviews      | review-service MongoDB             | Generiše komentare na igre i recenzije                                                                                           |
| `seed_reports.py`   | users + reviews      | review-service MongoDB             | Generiše prijave neprimerenog sadržaja                                                                                           |

## Redosled izvršavanja

Neke skripte zavise od podataka koje generišu prethodne. Preporučeni redosled izvršavanja:

```bash
uv run get_games.py      # jednom i uvek pre svih, da popuni sql_seed/ i media/
uv run seed_catalogue.py
uv run seed_users.py
uv run seed_library.py
uv run seed_media.py
uv run seed_reviews.py
uv run seed_comments.py
uv run seed_reports.py
```

## Struktura generisanih podataka

- `sql_seed/` - numerisani SQL fajlovi (`01_seed_platforms.sql` … `11_seed_game_tags.sql`) koje učitava
  `seed_catalogue.py`
- `media/` - po jedan direktorijum po ID-ju igre, sa `cover.jpg`, `screenshot_*.jpg` i `trailer.mp4`
