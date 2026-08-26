# Playlog - platforma za katalog i razmenu utisaka o video igrama

## Opis problema

Moderne platforme za distribuciju video igara, kao što su _Steam_, _Epic Games_ ili GOG, su primarno fokusirane na
vlasništvo i prodaju igara, dok je društveni aspekt igranja ne toliko širok i usko je vezan za konkretnu platformu.

U takvim sistemima, korisnički sadržaj (recenzije, ocene, biblioteke) je vezan za igre koje su kupljene ili aktivirane
na toj platformi, što otežava razmenu utisaka između korisnika različitih platformi: pre svega između PC i konzola (
_Xbox_, _PlayStation_, _Nintendo_).

Iz tehnološkog ugla, takve platforme su izgrađene od _large-scale_ sistema koji upravlja korisničkim nalozima, katalogom
igara, bibliotekom preuzetih igara, multimedijalnim sadržajem, korisnički-kreiranim sadržajem i mnogim drugim.
Moraju da podrže velik broj korisnika i entiteta a pritom da ostanu modularne, skalabilne i da budu lako održive.

Postoji potreba za rešenjem koje, uzimajući u obzir tehnološke aspekte, omogućava korisnicima da, **nezavisno od
platforme na kojoj igraju**:

- vode evidenciju o igrama koje su igrali ili žele da igraju, bez prethodne kupovine,
- dele mišljenja i ocene sa širim krugom korisnika i
- otkrivaju nove igre putem ocena i recenzija drugih korisnika

Jedno od postojećih rešenja je [Backloggd](https://backloggd.com/). Međutim, fokus ovog projekta nije da parira
postojećim platformama, što je objašnjeno u nastavku.

## Opis projekta

Cilj projekta je dizajn i implementacija platforme za katalog i razmenu utisaka o video igrama, sa fokusom na principe
mikroservisne arhitekture. Ideja je da se ovaj kompleksan domen razloži na nezavisne servise, gde je svaki zadužen za
specifičan deo sistema.

S tim na umu, Playlog je platforma zamišljena da omogući korisnicima da:

- koriste jedan nalog za upravljanje ličnom kolekcijom igara, nezavisno od platforme
- prelistavaju centralizovan katalog video igara
- vide detaljan prikaz neke igre sa svim relevantnim informacijama
- dodaju igre u svoju biblioteku, u neku od potkategorija (navedene u sekciji o arhitekturi)
- ostavljaju komentare, pišu recenzije i ocenjuju igre
- pregledaju biblioteke i aktivnosti drugih korisnika

## Tehnologije

### Backend:

- **Programski jezik:** `Rust`
- **Web framework:** `axum`
- **Baze podataka:**
    - _Relaciona_: `PostgreSQL`
    - _Nerelaciona_: `MongoDB`
- **Skladište multimedijalnog sadržaja:** `MinIO`
- **Kontejnerizacija:** `Docker`

### Frontend:

- **Web framework:** `Angular`
- **UI biblioteka:** `Angular Material`
- **Deployment:** `nginx` + `Docker`

## Uloge u sistemu

_Napomena: svaka naredna uloga ima sve privilegije prethodne_

- Gosti - mogu samo da gledaju javno dostupni sadržaj platforme, bez mogućnosti interakcije
- Registrovani (obični) korisnici - mogu da upravljaju sopstvenim profilom i bibliotekom igara, ostavljaju komentare,
  ocene i recenzije; imaju mogućnost prijave neprimerenog sadržaja
- Moderatori - registrovani korisnici sa dodatnim ovlašćenjima za pregled i uklanjanje prijavljenog sadržaja
- Administratori - zaduženi za održavanje sistema, kataloga igara, unapređivanje običnih korisnika u moderatore, kao i
  moderatora u administratore; imaju uvid u sve korisnike i mogu da im blokiraju nalog

## Arhitektura sistema

_Backend_ sistema je organizovan kao skup nezavisnih mikroservisa, od kojih svaki ima jasno definisane odgovornosti i
sopstveni model podataka.

Detaljniji opisi organizacije projekta mogu se pročitati u:

- [`playlog-backend/README.md`](playlog-backend/README.md),
- [`playlog-frontend/README.md`](playlog-frontend/README.md) i
- [`scripts/README.md`](scripts/README.md).

### 1. Servis za upravljanje korisnicima

**Odgovornosti:**

- Registracija i prijava korisnika
- Izdavanje i upravljanje JWT
- Upravljanje korisničkim nalozima
- Upravljanje statusom korisničkih naloga (aktivan, blokiran, deaktiviran)
- Upravljanje korisničkim ulogama
- Uvid u naloge drugih korisnika

**Baza podataka:** `PostgreSQL`

### 2. Servis za katalog video igara

**Odgovornosti:**

- Upravljanje centralizovanim katalogom video igara
- Obrada i čuvanje osnovnih podataka o igrama (naziv, opis, žanrovi, datum izlaska...)
- Pretraga i pregled kataloga igara

**Baza podataka:** `PostgreSQL`

### 3. Servis za korisničku biblioteku

**Odgovornosti:**

- Evidentiranje veza između korisnika i igara
- Pregled i organizacija igara u korisničkoj biblioteci u neku od kategorija:
    - u vlasništvu (podrazumevano)
    - trenutno se igra
    - lista želja (igre koje treba da izađu i koje planira da kupi)
    - pređena (podložna ocenjivanju)
    - odustao od igranja (podložna ocenjivanju)
- Omogućavanje pregleda biblioteka drugih korisnika

**Baza podataka:** `PostgreSQL`

### 4. Servis za multimediju

**Odgovornosti:**

- Upravljanje slikama (naslovne slike igara, _screenshot_-ovi)
- Upravljanje video sadržajem (trejleri igara)
- Skladištenje multimedijalnog sadržaja
- Čuvanje i obrada metapodataka o multimedijalnom sadržaju

**Skladište:** `MinIO` object storage

**Baza podataka:** `MongoDB` (metapodaci o multimedijalnom sadržaju, lokacije fajlova)

### 5. Servis za recenzije, ocene i komentare

**Odgovornosti:**

- Upravljanje korisničkim recenzijama i ocenama igara
- Upravljanje komentarima nad igrama i recenzijama
- Evidentiranje prijava neprimerenog sadržaja
- Omogućavanje pregleda i uklanjanja prijavljenog sadržaja (za moderatore i administratore)

**Baza podataka:** `MongoDB`

### 6. API Gateway servis

Centralna ulazna tačka u sistem.

**Odgovornosti:**

- Rutiranje zahteva ka odgovarajućim servisima
- Validacija JWT da bi se sprečio nevažeći zahtev pre nego što prosledi servisu
  - _Napomena: JWT se validira i u servisima, kako bi endpoint-ovi ostali zaštićeni u slučaju da Api Gateway otkaže_
- Ograničavanje pristupa funkcionalnostima na osnovu korisničkih uloga

## Pokretanje aplikacije

Aplikaciju je najlakše pokrenuti preko komandi iz `Makefile`-a u korenu projekta. Aplikacija je rađena na
`Linux Mint 22.3` operativnom sistemu.

### Preduslovi

- (obavezno) [Rust](https://rust-lang.org/tools/install/) (minimum 1.95.0)
- (obavezno) [Node.js](https://nodejs.org/en/download) (minimum 24.16.0),
- (obavezno) [Docker](https://docs.docker.com/get-docker/) i _Docker Compose_,
- (obavezno) RSA ključevi za JWT u `playlog-backend/keys/` - pogledati
  [`playlog-backend/README.md`](playlog-backend/README.md)
- (obavezno) Popunjeni .env fajlovi kod svakog servisa u `playlog-backend/` i u `playlog-frontend/` (videti primere u
  `.env.example` i `.env.production.example` fajlovima),
- (obavezno) Popunjeni `environment.ts` i `environment.development.ts` fajlovi u `playlog-frontend/src/environments/` -
  pogledati [`playlog-frontend/README.md`](playlog-frontend/README.md),
- (preporučeno) [GNU Make](https://www.gnu.org/software/make/) - podrška za pokretanje komandi iz _Makefile_-ova
  (najverovatnije je već instaliran, na popularnim _Linux_ distribucijama),
- (opciono) [uv](https://docs.astral.sh/uv/) za ubacivanje demo podataka - pogledati
  [`scripts/README.md`](scripts/README.md)

### Osnovne komande

Komande za ceo _compose stack_ (produkcioni režim):

```bash
make start         # aplikacija postaje dostupna na http://localhost:8080
make start-exposed
make stop
make build
make rebuild
make logs
```

Pregled dostupnih komandi:

```bash
make help
```
