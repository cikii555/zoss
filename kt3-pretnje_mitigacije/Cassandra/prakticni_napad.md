# TOMBSTONE EXPLOSION

Tombstone explosion je problem u bazi Apache Cassandra koji nastaje kada se nagomila veliki broj tzv. tombstone zapisa (oznaka za obrisane podatke). To može ozbiljno usporiti čitanje podataka i čak izazvati pad performansi sistema.Ovo je poznata tehnika za Cassandra DoS - Denial of Service.

## NAPAD

Simulacija tombstone explosion napad nad bazom Apache Cassandra tako što se  ubacuje veliki broj redova u tabelu transactions sa veoma kratkim TTL-om (5 sekundi). Kada TTL-Time to Live istekne, Cassandra ne briše redove odmah, već ih označava kao tombstone zapise. Ako ih se nakupi mnogo u istoj particiji (npr. za isti account_id), svako kasnije čitanje mora da obradi ogromnu količinu tih tombstone oznaka, što značajno usporava upite i dovodi do Denial of Service.

## MITIGACIJE

servis ograničava upit tako da čita samo novije transakcije (poslednja 24h) i dodatno koristi LIMIT 100, čime sprečava skeniranje ogromnog broja starih redova i tombstone zapisa. Time se značajno smanjuje broj particija i SSTable fajlova koje Cassandra mora da pregleda, pa čitanje ostaje brzo čak i ako napadač prethodno napravi mnogo tombstone-ova.