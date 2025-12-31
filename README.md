## 1. Domen problema

Softver se nalazi u domenu **digitalnog bankarstva (FinTech)**.  
Cilj sistema je da omogući **potpuno digitalno pružanje bankarskih usluga**, bez potrebe za fizičkim poslovnicama, uključujući upravljanje računima, obradu finansijskih transakcija, kreditne usluge, verifikaciju identiteta korisnika i detekciju prevara.

Sistem je projektovan za rad u realnom vremenu, visok stepen bezbednosti i usklađenost sa finansijskim i regulatornim zahtevima.

---

### 1.1. Učesnici

- **Klijenti banke**  
  Otvaraju i koriste račune, vrše transakcije i transfere novca. Podnose zahteve za kredite.

- **Bankarski službenici**  
  Otvaraju i zatvaraju račune klijentima, obrađuju zahteve za transakcije i kredite, bave se ažuriranjem podataka o klijentima, kao i verifikacijom njihovog identiteta.

- **Administrator sistema**  
  Upravlja korisničkim nalozima i pravima pristupa, bavi se konfiguracijom sistema i servisa.

- **Eksterni platni sistemi**  
  Visa, MasterCard koji omogućavaju autorizaciju plaćanja putem kartice na prodajnim mestima, platni sistemi SEPA, SWIFT.

- **Regulatorna tela**  
  Nadgledaju rad banke i brinu se da je sve u skladu sa zakonskim regulativama i standardima.

---

### 1.2. Poslovni procesi

- **Digitalna registracija i verifikacija korisnika (KYC)**  
  Identifikacija korisnika, validacija dokumenata i provera identiteta.

- **Upravljanje bankarskim računima**  
  Otvaranje računa, pregled stanja, upravljanje limitima i karticama.

- **Obrada finansijskih transakcija**  
  Izvršavanje domaćih i međunarodnih plaćanja u realnom vremenu.

- **Proces kreditiranja**  
  Podnošenje zahteva, procena kreditnog rizika i praćenje otplate.

- **Anti-fraud i upravljanje rizicima**  
  Detekcija sumnjivih obrazaca i blokada potencijalno nelegitimnih aktivnosti.

- **Izveštavanje i regulatorna usklađenost**  
  Generisanje audit logova i regulatornih izveštaja.

---

## 2. Arhitektura sistema

Sistem je projektovan kao mikroservisna, event-driven arhitektura, prilagođena visokoj dostupnosti, skalabilnosti i bezbednosti koja je neophodna u bankarskom domenu.

Sastoji se iz sledećih celina:

- Mikroservisi – svaki implementira odgovarajući specifični domen.
- Klijentske aplikacije – mobilne i web aplikacije koje koriste krajnji korisnici (klijenti banke, bankarski službenici).
- Skladištenje podataka – Digitalna banka koristi više specijalizovanih skladišta podataka, prilagođenih različitim tipovima informacija i zahtevima za performansama, dostupnošću i skalabilnošću. Ovaj višeslojni pristup omogućava optimizovanu obradu podataka, visoku pouzdanost i sigurnost.
- API Gateway – rutiranje zahteva ka odgovarajućim mikroservisima.
- Event-driven komponente – omogućena asinhrona razmena događaja.
- Integracioni adapteri – povezivanje sa eksternim sistema (Visa, MasterCard).
- **Autentifikacija i autorizacija – OAuth 2.0 / OpenID Connect**

Centralizovan Identity servis  

Višefaktorska autentifikacija (MFA), Token-based pristup servisima

---

### 2.1. Korištene tehnologije

#### Web App – Frontend
- **React.js** (Web) – moderni UI framework

#### Mobile App
- **Flutter** (Dart) – cross-platform za brže razvijanje funkcionalnosti na oba OS-a

#### Backend
- Mikroservisi – Golang / Rust  
- **API Gateway** – Golang

#### Event streaming
- **Apache Kafka** – event streaming između mikroservisa, bolja otpornost na greške, događaji ostaju u kanalu i mogu se naknadno obraditi

#### Baze podataka
- **MongoDB** – skladištenje podataka o korisnicima i njihovim nalozima, pogodna za brze promene strukture podataka bez rigidne šeme
- **Cassandra** – skladištenje podataka o transakcijama, dobra horizontalna skalabilnost
- **Redis** – cache, keširanje sesija, privremeno skladištenje tokena i podrška za rate-limiting, brza obrada autentifikacije, smanjenje opterećenja primarnih baza i poboljšanje performansi sistema u realnom vremenu

#### Eksterni servisi
- **Visa**, **MasterCard** – eksterni provajderi finansijskih servisa koji omogućavaju autorizaciju i obradu plaćanja preko kartice. Digitalna banka se povezuje sa njima putem standardizovanih API-ja, što omogućava:
  - autorizaciju transakcija
  - prenos sredstava sa korisnika na trgovce
  - praćenje statusa plaćanja u realnom vremenu
  - primenu sigurnosnih mehanizama (tokenizacija, 3D Secure, anti-fraud)

---

## 3. Slučajevi korišćenja

### Upravljanje korisničkim računom
- Omogućava korisnicima da kreiraju, ažuriraju i prate svoje račune u banci.
- Funkcionalnosti uključuju: pregled stanja računa, upravljanje ličnim podacima, postavljanje limita i blokada, praćenje istorije aktivnosti.

### Transakcije i plaćanja
- Omogućava iniciranje i praćenje plaćanja, kako domaćih tako i međunarodnih.
- Funkcionalnosti uključuju: SEPA plaćanja, internu razmenu između računa, evidenciju statusa transakcija i generisanje potvrda.

### Usluge koje podržavaju kartice
- Podržava sve operacije vezane za debitne i kreditne kartice.
- Funkcionalnosti uključuju: autorizaciju plaćanja, pregled stanja kartice, aktivaciju/deaktivaciju kartice, online kupovine i integraciju sa Visa/Mastercard mrežama.

### Verifikacija i KYC (Know Your Customer)
- Omogućava verifikaciju identiteta korisnika u skladu sa regulatornim zahtevima.
- Funkcionalnosti uključuju: prikupljanje i verifikaciju dokumenata, integraciju sa eksternim KYC provajderima, praćenje statusa verifikacije i ažuriranje korisničkog profila.

### Anti-fraud i sigurnosne provere
- Detekcija i prevencija sumnjivih aktivnosti u realnom vremenu.
- Funkcionalnosti uključuju: monitoring transakcija, identifikacija neobičnih obrazaca, blokada transakcija ili iniciranje dodatne provere, praćenje sigurnosnih događaja i audit logova.

### Notifikacije i obaveštenja
- Omogućava korisnicima praćenje aktivnosti i stanja računa putem različitih kanala.
- Funkcionalnosti uključuju: push notifikacije, SMS ili email obaveštenja o izvršenim transakcijama, statusu kartica i bezbednosnim upozorenjima.

### Administracija i izveštavanje
- Podržava administratore banke i interne procese nadzora i izveštavanja.
- Funkcionalnosti uključuju: generisanje finansijskih izveštaja, praćenje sistemskih događaja, audit logova, statistiku transakcija i korisničkih aktivnosti.

---

## 4. Osetljivi resursi

### Korisnički podaci i KYC (Know Your Customer)
**Opis resursa:**  
Lični podaci korisnika kao što su ime, prezime, JMBG, datum rođenja, adresa, kredencijali.

**Bezbednosni ciljevi:**  
Poverljivost, integritet, auditabilnost.

**Regulativa:**  
GDPR (za zemlje EU) i zakoni o zaštiti podataka koji postoje u Srbiji.

---

### Finansijske transakcije
**Opis resursa:**  
Podaci o svim novčanim tokovima: iznosi, valute, računi pošiljaoca i primaoca, vreme kad je transakcija obavljena, status transakcije.

**Bezbednosni ciljevi:**  
Integritet, poverljivost, dostupnost.

---

### Podaci sa kartice i autorizacije
**Opis resursa:**  
Brojevi kartica, CVV, tokeni, autorizacije plaćanja.

**Bezbednosni ciljevi:**  
Poverljivost, integritet, dostupnost.

**Regulativa:**  
PCI DSS – Payment Card Industry Data Security Standard.

---

### Audit logovi i bezbedonosni događaji
**Opis resursa:**  
Prijave korisnika, izmene podataka, transakcije, blokade računa, administrativne akcije.

**Bezbednosni ciljevi:**  
Integritet, dostupnost.

---

### Sigurnosni tokeni i sesije
**Opis resursa:**  
Privremeni tokeni (JWT, OAuth tokeni), sesije korisnika i servisni tokeni za komunikaciju između mikroservisa.

**Bezbednosni ciljevi:**  
Poverljivost, integritet, ograničeno trajanje.

---

### API ključevi i kredencijali za eksterne sisteme
**Opis resursa:**  
Ključevi i sertifikati za pristup Visa i MasterCard servisima.

**Bezbednosni ciljevi:**  
Poverljivost, kontrola pristupa.

---

### Sistem za notifikacije
**Opis resursa:**  
Sistem za notifikacije je odgovoran za slanje obaveštenja korisnicima o bitnim događajima u digitalnoj banci, kao što su izvršene transakcije, pokušaji prijave, promene bezbednosnih podešavanja, blokade kartica i status KYC verifikacije. Notifikacije se isporučuju putem različitih kanala, uključujući push notifikacije, email i SMS poruke.

**Bezbednosni ciljevi:**  
Poverljivost, integritet, dostupnost.
