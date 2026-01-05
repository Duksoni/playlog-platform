# Playlog - platforma za katalog i razmenu utisaka o video igrama


## Opis problema

Moderne platforme za distribuciju video igara, kao što su Steam, Epic Games ili GOG, su primarno fokusirane na vlasništvo i prodaju igara, dok je društveni aspekt igranja ne toliko širok i usko je vezan za konkretnu platformu.

U takvim sistemima, korisnički sadržaj (recenzije, ocene, biblioteke) je vezan za igre koje su kupljene ili aktivirane na toj platformi, što otežava razmenu utisaka između korisnika različitih platformi: pre svega između PC i konzola (Xbox, PlayStation, Nintendo).

Iz tehnološkog ugla, takve platforme su izgrađene od large-scale sistema koji upravlja korisničkim nalozima, katalogom igara, bibliotekom preuzetih igara, multimedijalnim sadržajem, korisnički-kreiranim sadržajem, analitikom korišćenja i mnogim drugim. Moraju da podrže velik broj korisnika i entiteta a pritom da ostanu modularne, skalabilne i da budu lako održive.

Postoji potreba za rešenjem koje, uzimajući u obzir tehnološke aspekte, omogućava korisnicima da, **nezavisno od platforme na kojoj igraju**:
- vode evidenciju o igrama koje su igrali ili žele da igraju, bez prethodne kupovine
- dele mišljenja i ocene sa širim krugom korisnika
- otkrivaju nove igre putem ocena i recenzija drugih korisnika

Jedno od postojećih rešenja je [Backloggd](https://backloggd.com/). Međutim, fokus ovog projekta nije da parira postojećim platformama, što je objašnjeno u nastavku.

## Opis projekta

Cilj projekta je dizajn i implementacija platforme za katalog i razmenu utisaka o video igrama, sa fokusom na principe mikroservisne arhitekture. Ideja je da se ovaj kompleksan domen razloži na nezavisne servise, gde je svaki zadužen za specifičan deo sistema i koji komuniciraju preko dobro definisanog API-a.

S tim na umu, Playlog je platforma zamišljena da omogući korisnicima da:
- koriste jedan nalog za upravljanje ličnom kolekcijom igara, nezavisno od platforme
- prelistavaju centralizovan katalog video igara
- vide detaljan prikaz neke igre sa svim relevantnim informacijama
- dodaju igre u svoju biblioteku, u neku od potkategorija (navedene u sekciji)
- ostavljaju komentare, pišu recenzije i ocenjuju igre
- pregledaju biblioteke i aktivnosti drugih korisnika

> Projekat planiram da radim za **maksimalnih 80 bodova**.


## Tehnologije

### Backend:

- **Programski jezik:** `Rust`
- **Web framework:** `axum`
- **Baze podataka:**
  - *Relaciona:* `PostgreSQL`
  - *Nerelaciona:* `MongoDB`
- **Skladište multimedijalnog sadržaja:** `MinIO`
- **Kontejnerizacija:** `Docker`

### Frontend:

- **Web framework:** `Angular`
- **UI biblioteka:** `Angular Material`
- *Opcioni deployment pomoću `Docker`-a i `nginx`-a*


## Uloge u sistemu

*Napomena: svaka naredna uloga ima sve privilegije prethodne*

- Neautentifikovani korisnici - mogu samo da gledaju javno dostupni sadržaj platforme, bez mogućnosti interakcije
- Registrovani (obični) korisnici - mogu da upravljaju sopstvenim profilom i bibliotekom igara, ostavljaju komentare, ocene i recenzije; imaju mogućnost prijave neprimerenog sadržaja i profila drugih korisnika
- Moderatori - registrovani korisnici sa dodatnim ovlašćenjima za pregled i uklanjanje prijavljenog sadržaja
- Administratori - zaduženi za održavanje sistema, kataloga igara, unapređivanje običnih korisnika u moderatore; imaju uvid u prijavljene korisnike i mogu da im blokiraju nalog


## Arhitektura sistema

Backend sistema je organizovan kao skup nezavisnih mikroservisa, od kojih svaki ima jasno definisane odgovornosti i sopstveni model podataka.

### 1. Servis za upravljanje korisnicima

**Odgovornosti:**

- Registracija i prijava korisnika
- Izdavanje i upravljanje JWT
- Upravljanje korisničkim nalozima
- Upravljanje statusom korisničkih naloga (aktivan, blokiran, obrisan/deaktiviran)
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

- Upravljanje slikama (naslovne slike igara, screenshotovi)
- Upravljanje video sadržajem (traileri igara, gameplay snimci)
- Skladištenje multimedijalnog sadržaja
- Čuvanje i obrada metapodataka o multimedijalnom sadržaju

**Skladište:** `MinIO` object storage

**Baza podataka:** `MongoDB` (metapodaci o multimedijalnom sadržaju, lokacije fajlova)


### 5. Servis za recenzije, ocene i komentare

**Odgovornosti:**

- Upravljanje korisničkim recenzijama i ocenama igara
- Upravljanje komentarima nad igrama i recenzijama
- Evidentiranje prijava neprimerenog sadržaja
- Omogućavanje pregleda i uklanjanja prijavljenog sadržaja (za moderatore i admine)

**Baza podataka:** `MongoDB`


### 6. API Gateway servis 

Centralna ulazna tačka u sistem

**Odgovornosti:**

- Rutiranje zahteva ka odgovarajućim servisima
- Validacija JWT i prosleđivanje identiteta korisnika servisima
- Ograničavanje pristupa funkcionalnostima na osnovu korisničkih uloga
- Primena `API composition` šablona, ako bude potrebe za tim
