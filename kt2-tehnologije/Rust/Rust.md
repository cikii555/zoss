# Uvod

Rust je relativno “mlad” programski jezik koji je stekao veliku popularnost zahvaljujući tome što stavlja akcenat na bezbednost, performanse kao i produktivnost. Nastao je kao sigurnija alternativa za C/C++ zbog problema koji ovi jezici mogu imati sa upravljanjem memorijom.`Ownership` je jedan od najbitnijih koncepata koje uvodi Rust. On omogućava Rustu da obezbedi sigurnost memorije bez potrebe za garbage collector-om.

Upravljanje memorijom zasniva se na sistemu vlasništva (`Ownership`), pri čemu kompajler striktno proverava poštovanje tih pravila. Ukoliko se bilo koje pravilo prekrši, program se neće uspešno kompajlirati.

# Arhitektura jezika
![Arhitektura jezika](673dd84b7825bdfb8796ef7a_65254585c1fe3a8290e5fe9b_rust-applications-2023)

## Rustup

Rustup je zvanični i preporučeni alat za komandnu liniju za instalaciju, upravljanje i ažuriranje celog Rust programskog toolchain-a. Toolchain zapravo predstavlja kompletnu instalaciju Rust kompajlera kao i drugih alata poput cargo, clippy itd. Rustup funkcioniše kao menadžer verzija, omogućavajući da efikasan rad sa različitim verzijama i komponentama Rust-a.

## Rustc

Rustc je ugrađeni kompajler Rust programskog jezika. On je odgovoran za pretvaranje izvornog koda u izvršne datoteke i biblioteke. Rustc se oslanja na LLVM za backend optimizaciju i generisanje mašinskog jezika što dovodi do poboljšanja performansi. U većini slučajeva rustc se ne poziva direktno nego kroz Cargo alat o kome će u nastavku biti reči.

## LLVM

LLVM je kompajlerska infrastruktura koja se koristi za pretvaranje izvornog koda u efikasan mašinski kod. Njegova glavna uloga je da preuzme međukorak u kompajliranju i uradi sledeće:

- Prima međukod (IR – Intermediate Representation) iz kompajlera  
- Analizira i optimizuje kod – uklanja nepotrebne operacije, poboljšava performanse i smanjuje potrošnju memorije  
- Generiše mašinski kod prilagođen konkretnoj arhitekturi procesora (x86, ARM, RISC-V…)

## Cargo

Cargo je Rustov sistem za izgradnju projekata i menadžer paketa. Većina Rust programera koristi ovaj alat za upravljanje svojim Rust projektima, jer Cargo obavlja mnogo zadataka umesto vas, kao što su izgradnja koda, preuzimanje biblioteka od kojih kod zavisi. Pored toga, Cargo omogućava pripremu paketa za distribuciju i njegovo postavljanje na crates.io, zvanični registar paketa Rust zajednice. Prilikom kreiranja projekta koristeći Cargo kreira se i fajl Cargo.toml u kome se navode sve biblioteke – crates koje se koriste u projektu, kao i opis konfiguracije projekta. Tom prilikom takođe kreira se i Cargo.lock koji sadrži precizne informacije o tačnim verzijama svih zavisnosti, automatski ga održava Cargo i ne treba ga ručno menjati.

## Crate

Crate je najmanja jedinica koda koju Rust kompajler obrađuje u jednom trenutku. Čak i kada pokrenete rustc umesto Cargo-a i prosledite samo jedan izvorni fajl, kompajler taj fajl smatra crate-om. Crate-ovi mogu sadržati module, a ti moduli mogu biti definisani u drugim fajlovima koji se kompajliraju zajedno sa crate-om. Crate može biti u jednom od dva oblika: binary crate ili library crate.

- **Binary crates** su programi koje se mogu kompajlirati u izvršni fajl i pokrenuti, poput programa iz komandne linije ili servera. Svaki binary crate mora imati funkciju `main` koja definiše šta se dešava kada se izvršni fajl pokrene.  
- **Library crates** nemaju funkciju `main` i ne kompajliraju se u izvršni fajl. Umesto toga, oni definišu funkcionalnost koja je namenjena da se deli između više projekata.


# Standardna biblioteka
![Standardna biblioteka](Rust_standard_libraries.svg)

Rust standardna biblioteka predstavlja temelj prenosivog Rust softvera: skup minimalnih i provernih apstrakcija koje dijeli širi Rust ekosistem. Ona pruža osnovne tipove, poput Vec<T> i Option<T>, bibliotečke operacije nad jezičkim primitivima, standardne makroe, ulaz/izlaz (I/O) i multithreading, kao i mnoge druge funkcije.

Standardna Rust biblioteka se sastoji od dosta modula, kao i primitivnih tipova koje je moguće koristiti. Interno, standardna biblioteka je podeljena na tri dela: **core**, **alloc** i **std**.

- **core** je osnovna Rust biblioteka koja predstavlja “vezu” između jezika i njegovih biblioteka i koja definiše osnovne i primitivne delove celog Rust koda.  
- **alloc** je izgrađen na core i pruža pametne pokazivače i kolekcije za upravljanje vrednostima smeštenim na heap-u.  
- **std** zavisi od alloc i core i pruža kompletnu standardnu biblioteku sa I/O, fajlovima, nitima i drugim funkcionalnostima. std je dostupna svim Rust paketima (crate-ovima) podrazumevano. Zbog toga se standardna biblioteka može koristiti u use izrazima putem putanje std, na primer: `use std::env`.

# Makroi

Makroi omogućavaju generisanje i transformaciju Rust koda kako bi se smanjila ponavljanja. Makroi postoje u dva oblika:

- **Deklarativni makro (macro by example)** je makro koji se definiše pomoću ključne reči `macro_rules!` i koristi pattern matching da odredi kako će se proširiti.  
- **Proceduralni makroi** omogućavaju izvršavanje koda tokom kompajliranja koji obrađuje Rust sintaksu, pri čemu mogu i da čitaju i da generišu Rust kod, to jest oni modifikuju tok tokena koji kompajler prima kao ulaz. Definišu se u posebnim crate-ovima tipa proc-macro-pomoćnu biblioteku koja pomaže pri definisanju novih makroa i koriste se tako što se importuju u drugim crate-ovima.

# Reference

- https://rust-lang.github.io/rustup/concepts/index.html  
- https://doc.rust-lang.org/beta/rustc/what-is-rustc.html  
- https://doc.rust-lang.org/std/  
- https://www.freecodecamp.org/news/procedural-macros-in-rust/  
- https://doc.rust-lang.org/book/ch01-03-hello-cargo.html  
- https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html  
- https://doc.rust-lang.org/std/index.html  
- https://doc.rust-lang.org/reference/macros.html