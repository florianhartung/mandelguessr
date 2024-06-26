# MandelGuessr
Ein Browserspiel ähnlich zu GeoGuessr, bei welchem zufällige Orte aus Google Maps gefunden werden müssen, allerdings im Mandelbrot-Set.

# Anleitung zum ausführen
Die Fullstack-Anwendung kann mit Docker ausgeführt werden.
Zusätzlich muss nur eine PostgreSQL Datenbank bereitgestellt werden. Diese wird dann beim Ausführen des Docker-Images über die Umgebungsvariable `DATABASE_URL` in das Programm hineingegeben.

1. Bauen des Docker-Images: `docker build -t .`
2. Herausfinden der PostgreSQL-Datenbank URL. Diese folgt dem Schema: `postgres://<username>:<password>@<IP>/mandelguessr`. Falls Docker verwendet wird und die Datenbank auf dem Host-Computer läuft, bietet sich als IP `host.docker.internal` an.
3. Starten des Docker-Images: `docker run -e DATABASE_URL="postgres://<username>:<password>@<IP>/mandelguessr" -p 80:80 mandelguessr`


In case the server just crashes randomly consider trying this fix: https://github.com/diesel-rs/diesel/discussions/2947#discussioncomment-2025857