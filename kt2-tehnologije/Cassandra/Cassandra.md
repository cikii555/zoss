# Cassandra – distribuirana NoSQL baza podataka

Cassandra je distribuirana NoSQL baza podataka. Po dizajnu, NoSQL baze podataka open-source, nerealacionog tipa i uglavnom distribuirane. Među njihovim prednostima su horizontalna skalabilnost, distribuirane arhitekture i fleksibilan pristup definisanju šeme.

## Arhitektura Cassandre

![Arhitektura Cassandre](cassandra_arhitektura.jpg)

Cassandra se sastoji iz čvorova. Jedan čvor predstavlja jednu instancu Cassandre. Cassandra podržava horizontalnu skalabilnost, koja se postiže dodavanjem više čvorova u okviru Cassandra klastera. Ovi čvorovi međusobno komuniciraju putem protokola nazvanog gossip, što je proces peer-to-peer komunikacije između računara.

Cassandra ima arhitekturu bez glavnog čvora (masterless), što znači da bilo koji čvor u bazi može pružiti potpuno istu funkcionalnost kao i bilo koji drugi čvor.

## Klaster, rack-ovi i data centri

Klaster je podeljen na rack-ove i data centre. U Cassandri se čvorovi mogu grupisati u rack-ove i data centre pomoću konfiguracije snitch.

## Organizacija podataka

Cassandra čuva podatke u tabelama, gde je svaka tabela organizovana u redove i kolone. Tabele su grupisane u keyspace-ove (ključ-prostore). Keyspace se koristi za grupisanje tabela koje služe sličnoj svrsi sa poslovne perspektive.

Replikacija podataka se konfiguriše po keyspace-u u smislu faktora replikacije po data centru i strategije replikacije.

## Primarni ključ

Svaka tabela ima definisan primary key (primarni ključ). Primarni ključ je podeljen na partition key (ključ particije) i clustering columns (kolone za klasterovanje), koje su opcione. Ne postoji ograničenje jedinstvenosti za bilo koji od ključeva.

### Partition key

![Arhitektura jezika](partition_key.jpg)

Partition key se koristi u Cassandri za indeksiranje podataka. Svi redovi koji dele isti partition key čine jednu data particiju, koja predstavlja osnovnu jedinicu particionisanja podataka, skladištenja i preuzimanja u Cassandri.

###Partitioning

U Cassandri se partition key ne koristi direktno za pronalaženje podataka. Umesto toga, on se prvo prosleđuje funkciji koja se zove partitioner, čiji je zadatak da od vrednosti partition key-a izračuna token.
 On primenjuje hash funkciju nad partition key-em i kao rezultat daje jedan celobrojni token. Taj token je broj iz velikog opsega vrednosti i nazivase token range.
Kada aplikacija pošalje upit sa određenim partition key-em, Cassandra:

izračuna token na osnovu partition key-a

proveri kojem čvoru pripada taj token

prosledi upit baš tom čvoru (i njegovim replikama)

Ako klaster ima samo jedan čvor, taj čvor poseduje ceo token range, pa samim tim i sve podatke. Kako se dodaju novi čvorovi, token range se ravnomerno deli između njih. Svaki novi čvor preuzima deo opsega, a podaci se automatski preraspodeljuju.

## Replikacija podataka

Jedan podatak može biti repliciran na više čvorova (replika), čime se obezbeđuje pouzdanost i otpornost na greške. Cassandra podržava pojam faktora replikacije (Replication Factor – RF), koji opisuje koliko kopija podataka treba da postoji u bazi.

Ako je RF = 1, podaci se nalaze samo na jednom čvoru. Kod RF = 2, podaci se čuvaju na dva čvora, pri čemu svaki čvor postaje odgovoran i za sekundarni opseg tokena pored svog primarnog opsega. Faktor replikacije od tri obezbeđuje da postoje tri čvora (replike) koja pokrivaju određeni opseg tokena.


## Strategije replikacije
U Cassandri se podaci u okviru jednog keyspace-a čuvaju u više kopija, što se definiše faktorom replikacije, koji je najčešće 3. Jedna kopija podataka se nalazi na čvoru koji poseduje odgovarajući token, dok se ostale kopije raspoređuju na druge čvorove kako bi se obezbedila dostupnost i otpornost na greške. Na to gde će replike biti smeštene najviše utiču snitch, koji Cassandri govori u kom data centru i rack-u se nalazi svaki čvor, i strategija replikacije, koja određuje pravila raspodele podataka. U praksi se gotovo uvek koristi NetworkTopologyStrategy, jer je svesna data centara i rack-ova i omogućava pouzdan rad klastera, dok se SimpleStrategy koristi samo za male ili testne klastere.


## Cassandra Query Language (CQL)

Cassandra Query Language (CQL) je interfejs za upitivanje Cassandre preko binarnog protokola. Ranije verzije Cassandre su podržavale Thrift, koji je danas u potpunosti zamenjen CQL-om.

CQL je dizajniran da bude sličan SQL-u, kako bi se omogućila brža kriva učenja i poznata sintaksa.

### Operacije

DDL (Data Definition Language) operacije omogućavaju kreiranje keyspace-ova i tabela. CRUD operacije uključuju SELECT, INSERT, UPDATE i DELETE, pri čemu je SELECT operacija čitanja u Cassandri, dok su sve ostale operacije pisanja.

### CAP teorema
Svaki distribuirani sistem funkcioniše po principu CAP teoreme.
CAP teorema kaže da bilo koji distribuirani sistem može snažno garantovati najviše dve od sledeće tri osobine:

Konzistentnost (Consistency)

Dostupnost (Availability)

Otpornost na particionisanje (Partition tolerance)

Cassandra pruža fleksibilnost u izboru između konzistentnosti i dostupnosti prilikom upita nad podacima. Drugim rečima, podaci mogu biti visoko dostupni uz niži stepen konzistentnosti, ili visoko konzistentni uz manju dostupnost.

# Reference
- https://cassandra.apache.org/_/cassandra-basics.html 
-https://www.instaclustr.com/blog/cassandra-architecture/ 