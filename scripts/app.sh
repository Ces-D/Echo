export $(grep -v '^#' ./.env | xargs -d '\n')

test(){
  cd ./server/ && cargo test
}

build(){
  cd ./server/ && cargo build
}

if [ "$1" = "test" ]; then
  test
elif [ "$1" = "build" ]; then
  build
else 
  echo "Invalid command"
  echo "Usage: ./scripts/app.sh [test|build]"
  echo "This script is intended to be run directly from the root directory of the project."
fi
