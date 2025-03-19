export $(grep -v '^#' ./.env | xargs -d '\n')

dev(){
  cd ./www && pnpm dev
}

build(){
  cd ./www && pnpm build
}

if [ "$1" == "dev" ]; then
  dev
elif [ "$1" == "build" ]; then
  build
else 
  echo "Command not found"
  echo "Available commands: dev, build"
fi
