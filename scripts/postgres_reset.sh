## https://stackoverflow.com/questions/38249434/docker-postgres-failed-to-bind-tcp-0-0-0-05432-address-already-in-use

## Docker-postgres-failed-to-bind-tcp-0-0-0-05432-address-already-in-use

## This will stop the port and allow Docker to once again bind to it
sudo ss -lptn 'sport = :5432'
sudo service postgresql stop
