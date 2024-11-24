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
  ./target/debug/echo -h
}

#### FIND A PLAYLIST
test_help_find_playlist(){
  ./target/debug/echo find-playlist -h
}
find_playlist(){
  ./target/debug/echo find-playlist "Sa"
}

#### LOAD A PLAYLIST
test_help_load_playlist(){
  ./target/debug/echo load-playlist -h
}
load_playlist(){
# 1z0FnFhsSKP0rqNvNqc30c All The Small Things Playlist id
  ./target/debug/echo load-playlist -p "spotify:playlist:0pDotXuIfBJpmZfUQ9zFfx" -t
}

#### COMPARE TWO PLAYLISTS
test_help_compare_playlist(){
  ./target/debug/echo compare-playlist -h
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
elif [ "$1" == "test_help_compare_playlist" ]; then
  test_help_compare_playlist
else
  echo "Invalid Command"
fi

