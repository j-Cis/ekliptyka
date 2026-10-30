# info

## 1. Wygenerowanie lokalnego certyfikatu (Self-Signed)

Otwórz terminal (najlepiej Git Bash) w głównym folderze projektu (tam gdzie Trunk.toml) i wpisz dokładnie to polecenie (podmień IP na swoje obecne, jeśli się zmieniło):

```powershell
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 365 -nodes -subj "/CN=192.168.1.6"
```

Wygeneruje to w Twoim folderze dwa pliki: key.pem (klucz prywatny) oraz cert.pem (certyfikat).

## Dodanie certyfikatów do konfiguracji Trunka

Otwórz plik Trunk.toml i dodaj ścieżki do wygenerowanych plików w sekcji [serve]. Cała sekcja powinna wyglądać tak:

```toml
[serve]
addresses = ["127.0.0.1", "0.0.0.0"]
tls_key_path = "key.pem"
tls_cert_path = "cert.pem"
```

## 3. Uruchomienie i test na telefonie

1. Zrestartuj serwer: trunk serve

2. W przeglądarce na telefonie musisz teraz koniecznie wpisać adres z prefiksem HTTPS:
[https://192.168.1.6:8080/](https://192.168.1.6:8080/) (zwróć uwagę na https).

3. Przeglądarka wyświetli duży, czerwony komunikat "Połączenie nie jest prywatne" (to normalne, bo certyfikat wystawiłeś sam sobie, a nie przez globalny urząd).

4. Kliknij Zaawansowane (Advanced), a następnie Przejdź do witryny 192.168.1.6 (niebezpieczne) / Proceed to....

Zignorowanie tego ostrzeżenia ostatecznie "otwiera" bezpieczny kontekst w przeglądarce. Dla silnika Chromium warunek HTTPS zostaje spełniony, aplikacja w Ruście dostaje pełen dostęp do WebGPU i kanwa powinna wyrenderować się bez błędu.

---

Rozwiązanie dla całej sieci LAN, a nie tylko test na chwilę, musimy oszukać system telefonu tak, aby w 100% ufał Twojemu serwerowi.

Przeglądarka na telefonie blokuje dostęp do układu graficznego (WebGPU/WebGL), ponieważ wyświetla czerwoną kłódkę (błąd `ERR_CERT_AUTHORITY_INVALID`). Aby uzyskać zieloną kłódkę (pełny Secure Context) w prywatnej sieci LAN bez wykupywania domen internetowych, musisz dodać swój certyfikat do **zaufanych certyfikatów systemowych Androida**.

Oto jak to rozwiązać na stałe, by każda aplikacja uruchomiona pod tym IP działała od ręki:

### Metoda 1: Instalacja obecnego certyfikatu na telefonie

Spróbujmy użyć pliku `cert.pem`, który wygenerowałeś wcześniej przez OpenSSL:

1. Prześlij plik **`cert.pem`** z komputera na swój telefon (np. przez Dysk Google, kabel USB lub e-mail).
2. Na telefonie wejdź w **Ustawienia**.
3. Wyszukaj w ustawieniach opcję **Certyfikaty**, **Zainstaluj certyfikat** lub wejdź w *Zabezpieczenia i prywatność -> Więcej ustawień zabezpieczeń -> Szyfrowanie i dane uwierzytelniające -> Zainstaluj certyfikat*.
4. Wybierz **Certyfikat urzędu certyfikacji (CA)** (jeśli pojawi się ostrzeżenie, kliknij "Zainstaluj mimo to").
5. Wskaż pobrany plik `cert.pem`. System poprosi o podanie pinu/wzoru i zapyta o nazwę (wpisz np. "MojLAN").
6. Całkowicie zamknij (ubij) przeglądarkę Kiwi i wejdź ponownie na `[https://192.168.1.6:8080](https://192.168.1.6:8080)`.

Jeśli Android przyjmie certyfikat, zamiast błędu zobaczysz zamkniętą kłódkę, a przeglądarka odblokuje sprzętowe renderowanie dla Twojej aplikacji.

### Metoda 2: Użycie narzędzia `mkcert` (Zalecany standard dla LAN)

Nowsze wersje Androida bywają wybredne i mogą nie chcieć zainstalować surowego certyfikatu z OpenSSL (wymagają specjalnych flag CA). Standardem branżowym do stawiania zaufanego HTTPS w LAN jest mikronarzędzie `mkcert`.

1. Na komputerze zainstaluj `mkcert` (jeśli masz menedżer pakietów `scoop`, wpisz: `scoop install mkcert`, albo pobierz plik .exe z ich GitHuba).

2. W terminalu wpisz polecenie, które wygeneruje Twój prywatny "Urząd Certyfikacji" (CA):

```bash
mkcert -install

```

3. Wygeneruj idealny certyfikat dla swojego IP (w folderze projektu):

```bash
mkcert 192.168.1.6

```

*Powstaną dwa pliki: `192.168.1.6.pem` (certyfikat) oraz `192.168.1.6-key.pem` (klucz).*

4. Podmień te pliki w `Trunk.toml`:

```toml
tls_key_path = "192.168.1.6-key.pem"
tls_cert_path = "192.168.1.6.pem"

```


5. Teraz najważniejsze: wejdź w lokalizację swojego "Urzędu CA" na komputerze (sprawdzisz ją wpisując w terminalu `mkcert -CAROOT`). Będzie tam plik **`rootCA.pem`**.
6. **To ten plik (`rootCA.pem`) prześlij na telefon i zainstaluj w Ustawieniach Androida jako Certyfikat CA (kroki z Metody 1).**

Po instalacji `rootCA.pem` na telefonie, Twoje urządzenie mobilne będzie bezgranicznie ufać każdemu certyfikatowi wystawionemu w Twojej sieci przez Twój komputer. Odpalasz serwer, wchodzisz z telefonu na HTTPS i masz w pełni działające środowisko WebGPU bez żadnych ostrzeżeń o niebezpieczeństwie.

---


Certyfikaty są gotowe w folderze projektu. Teraz zepnijmy to z serwerem i zainstalujmy certyfikat główny na telefonie:

### 1. Zaktualizuj `Trunk.toml`

Otwórz plik **`Trunk.toml`** i podmień sekcję `[serve]` na poniższą:

```toml
[serve]
addresses = ["127.0.0.1", "0.0.0.0"]
tls_key_path = "192.168.1.6+2-key.pem"
tls_cert_path = "192.168.1.6+2.pem"

```

---

### 2. Wyciągnij certyfikat główny (`rootCA.pem`) dla telefonu

Wpisz w PowerShellu:

```powershell
mkcert -CAROOT

```

Komenda wyświetli ścieżkę do folderu (np. `C:\Users\GEPIDEN\AppData\Local\mkcert`). Przejdź tam w Eksploratorze Windows – znajduje się tam plik **`rootCA.pem`**.

---

### 3. Instalacja `rootCA.pem` na telefonie z Androidem

1. Prześlij plik **`rootCA.pem`** na telefon (np. przez USB, Bluetooth lub wrzucając na Dysk Google).
2. Na telefonie otwórz **Ustawienia**.
3. Wpisz w lupce wyszukiwania ustawień: **Certyfikat** lub **Zainstaluj certyfikat** (ścieżka zależy od nakładki: *Hasła i bezpieczeństwo / Prywatność -> Certyfikaty i dane logowania -> Zainstaluj certyfikat*).
4. Wybierz opcję **Certyfikat urzędu certyfikacji (CA)**.
5. Zaakceptuj ewentualne ostrzeżenie systemowe ("Zainstaluj mimo to") i wybierz przesłany plik `rootCA.pem`.
6. Potwierdź instalację kodem PIN lub odciskiem palca.

---

### 4. Uruchomienie serwera

W PowerShellu w folderze projektu uruchom:

```powershell
trunk serve

```

Po uruchomieniu wejdź na telefonie w przeglądarce pod adres:
**`[https://192.168.1.6:8080/](https://192.168.1.6:8080/)`**

Przeglądarka wyświetli zieloną/zamkniętą kłódkę bez ostrzeżeń, system rozpozna pełny Secure Context, a WebGPU/WebGL uruchomi aplikację bez czarnego ekranu.

---
