#!/bin/bash

export $(grep -v '^#' ./.env.local | xargs -d '\n')

setup_diesel_database(){

  cd ./echo_db/ &&  diesel setup
}

migrate_up(){
  cd ./echo_db/ && diesel migration run
}

migrate_dw(){
  cd ./echo_db/ && diesel migration revert
}

test_help() {
  ./target/debug/echo_cli -h
}

#### FIND A PLAYLIST
test_help_find_playlist(){
  ./target/debug/echo_cli find -h
}
find_playlist(){
  ./target/debug/echo_cli find "Sa"
}

#### LOAD A PLAYLIST
test_help_load_playlist(){
  ./target/debug/echo_cli load -h
}
load_playlist(){
    local playlist_id="$2"
    if [ -z playlist_id ]; then
    ./target/debug/echo_cli load -t
    else
    echo "here $playlist_id"
    ./target/debug/echo_cli load -p playlist_id -t
    fi
}

#### LOAD A PLAYLIST
loaded_playlists(){
  ./target/debug/echo_cli loaded
}



if [ "$1" == "setup" ]; then
  setup_diesel_database
elif [ "$1" == "migrate_up" ]; then
  migrate_up
elif [ "$1" == "migrate_dw" ]; then
  migrate_dw
elif [ "$1" == "test_help" ]; then
  test_help
elif [ "$1" == "test_help_find_playlist" ]; then
  test_help_find_playlist
elif [ "$1" == "find_playlist" ]; then
  find_playlist
elif [ "$1" == "test_help_load_playlist" ]; then
  test_help_load_playlist
elif [ "$1" == "load_playlist" ]; then
  load_playlist
elif [ "$1" == "loaded_playlists" ]; then
  loaded_playlists
else
  echo "Invalid Command"
  echo "Usage: ./run.sh setup|migrate_up|migrate_dw|test_help|test_help_find_playlist|find_playlist|test_help_load_playlist|load_playlist|loaded_playlists"
fi
