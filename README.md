# 📁 portfolio-code

Dette prosjektet består av en frontend bygget med **React + Tailwind CSS**, og en backend skrevet i **Rust**. For å kjøre prosjektet trenger du både **Node.js** og **Rust** installert.

---

## 📆 Krav

* [Installér Rust](https://www.rust-lang.org/tools/install)
* [Installér Node.js](https://nodejs.org/en/download)
* En MariaDB-instans (lokalt eller ekstern)

---

## 🚀 Kom i gang

### 🔹 Frontend (React + Tailwind)

1. Åpne terminalen og naviger til `client`-mappen:

   ```bash
   cd client
   ```

2. Installer avhengigheter:

   ```bash
   npm install
   ```

3. Start utviklingsserveren:

   ```bash
   npm run build
   ```

Frontend vil da kjøre på [http://localhost:5173](http://localhost:5173) (som er Vite sin standardport).

---

### 🔹 Backend (Rust)

1. Naviger til `backend`-mappen:

   ```bash
   cd backend
   ```

2. Kjør serveren:

   ```bash
   cargo run --bin main
   ```

Backend vil da kjøre på [http://localhost:8080](http://localhost:8080).

---

## 🔐 Opprett bruker i MariaDB

1. Logg inn i MariaDB som root:

   ```bash
   sudo mysql -u root -p
   ```

2. Kjr følgende kommandoer for å opprette en ny bruker og gi rettigheter:

   ```sql
   CREATE USER 'brukernavn'@'localhost' IDENTIFIED BY 'passord';
   GRANT ALL PRIVILEGES ON *.* TO 'brukernavn'@'localhost' WITH GRANT OPTION;
   FLUSH PRIVILEGES;
   ```

   Husk å erstatte `brukernavn` og `passord` med det du faktisk vil bruke.

---

## 💡 Hvorfor disse teknologiene?

* Jeg valgte **React** med **Node/Vite** fordi jeg ønsket å lære nye og moderne metoder for å bygge frontend-applikasjoner.
* Jeg bruker **Tailwind CSS** fordi jeg liker hvordan det forenkler og effektiviserer CSS-styling.
* Jeg valgte **Rust** fordi det er et raskt, moderne og trygt språk som gir meg mye å lære – spesielt innen web og systemprogrammering.

---

## 📚 Oppsummering

Dette prosjektet er et læringsprosjekt hvor jeg kombinerer frontend og backend-teknologier jeg ønsket å utforske. Målet er å bygge noe funksjonelt samtidig som jeg lærer mer om moderne webutvikling.
