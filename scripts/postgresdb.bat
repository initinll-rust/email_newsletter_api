@echo off

echo "CMD HELP ::"
echo "CMD: start-admin -> starts pgadmin GUI"
echo "CMD: stop-admin -> stops pgadmin GUI"
echo "CMD: start-db -> starts postgres db"
echo "CMD: stop-db -> stops postgres db"
echo "CMD: install-sqlx -> installs sqlx binary for postgres db"

set args=%1

if defined args (
	if %args%==start-admin (
		docker container rm pgadmin4
		docker pull dpage/pgadmin4
		docker run -d --name pgadmin4 -e PGADMIN_DEFAULT_EMAIL=admin@admin.com -e PGADMIN_DEFAULT_PASSWORD=admin -p 5050:80 dpage/pgadmin4
	)

	if %args%==stop-admin (
		docker stop pgadmin4
		docker container rm pgadmin4
	)

	if %args%==start-db (
		docker container rm postgresql
		docker pull postgres
		docker run -d --name postgresql -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=newsletter -p 5432:5432 postgres
		docker inspect --format="{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}" postgresql
		sqlx migrate info --source ../migrations
		sqlx migrate run --source ../migrations
	)

	if %args%==stop-db (
		docker stop postgresql
		docker container rm postgresql
	)

	if %args%==install-sqlx (
		cargo install sqlx-cli --no-default-features --features native-tls,postgres
		set DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter
	)
)