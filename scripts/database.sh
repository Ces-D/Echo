export $(grep -v '^#' ./.env | xargs -d '\n')

setup(){
  cd ./server/ && diesel setup
}

## Ex. bash scripts/database.sh setup
migrate_up(){
  cd ./server/ && diesel migration run
}

## Ex. bash scripts/database.sh migrate_dw
migrate_down(){
  cd ./server/ && diesel migration revert
}

## Ex. bash scripts/database.sh new_migration create_users
new_migration(){
  echo "Migration Name: $1"
  cd ./server/ && diesel migration generate "$1" 
}

if [ "$1" == "setup" ]; then
  setup
elif [ "$1" == "migrate_up" ]; then
  migrate_up
elif [ "$1" == "migrate_dw" ]; then
  migrate_down
elif [ "$1" == "new_migration" ]; then
  new_migration $2
else
  echo "Invalid Command"
  echo "Usage: ./scripts/database.sh [setup|migrate_up|migrate_dw|new_migration]"
  echo "This script is intended to be run directly from the root directory of the project."
  echo "It is assumed that you have postgres and diesel-cli installed locally on your machine"
fi
