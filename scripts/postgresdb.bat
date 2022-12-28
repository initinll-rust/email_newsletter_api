@echo off

echo "CMD HELP ::"
echo "CMD: startdb -> starts postgres db"
echo "CMD: stopdb -> stops postgres db"
echo "CMD: isqlx -> installs sqlx binary for postgres db"


if %1==startdb (
	docker container rm postgresql
	docker pull postgres
	docker run -d --name postgresql -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=newsletter -p 5432:5432 postgres
)

if %1==stopdb (
	docker stop postgresql
	docker container rm postgresql
)

if %1==isqlx (
	cargo install sqlx-cli --no-default-features --features native-tls,postgres
	set DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter
)
