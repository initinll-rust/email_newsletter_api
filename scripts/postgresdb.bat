@echo off

echo use startdb to start postgres db and stopdb to stop postgres db

if %1==startdb (
	docker container rm postgresql
	docker pull postgres
	docker run -d --name postgresql -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=newsletter -p 5432:5432 postgres postgres-N 1000
)

if %1==stopdb (
	docker stop postgresql
	docker container rm postgresql
)