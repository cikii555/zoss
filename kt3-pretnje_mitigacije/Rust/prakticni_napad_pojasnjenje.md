# TOCTOU (Time-of-Check to Time-of-Use) trka uslova

TOCTOU trka uslova nastaje kada ponašanje programa zavisi od stanja resursa u dva odvojena trenutka:  
vreme provere (Time of Check - TOC) i vreme korišćenja (Time of Use - TOU).  

U Rust-u, TOCTOU ranjivosti mogu dovesti do sigurnosnih eksploatacija, posebno kada se iskoriste za manipulaciju stanja resursa između provere i korišćenja, što može zaobići sigurnosne provere ili narušiti kontrolu pristupa.

## Napad

Funkcija `insecure_file_access` prvo proverava da li fajl postoji pomoću `fs::metadata`, a zatim pokušava da pročita fajl koristeći `fs::read_to_string`.  

Međutim, postoji vremenski prozor između provere i korišćenja u kojem se stanje fajla može promeniti, što dovodi do TOCTOU ranjivosti. Napadač bi potencijalno mogao da manipuliše stanjem fajla između provere i korišćenja kako bi zaobišao kontrole pristupa ili pročitao osetljive podatke.

## Mitigacija

Funkcija `secure_file_access` direktno pokušava da pročita fajl koristeći `fs::read_to_string` bez prethodne provere.  

Izbegavanjem odvojenih koraka provere i korišćenja, kod eliminiše vremenski prozor za TOCTOU ranjivosti. Umesto toga, svi eventualni problemi prilikom pristupa fajlu se direktno obrađuju, što obezbeđuje integritet i sigurnost operacije.
