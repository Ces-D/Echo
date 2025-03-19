# echo

## END GOAL:

Allow users to create playlists and music queues based on their loaded playlists and specified preferences.

## Ideation:

Allow users to prompt the end goal for how they would like to interact with their songs

Example -

> "I am about to workout. Create a queue for running. Make it high tempo and energizing"
> "I am studying. Create a playlist. I want to be stimulated but not distracted"
> "Update my afro beats playlist with my latest likes"
> "Create a playlist using music from my likes that fits the theme of techno energy"
> "Organize my likes based on genre"

How -

1. User enters a prompt providing guidelines for what they would like achieved
1. Interpret the prompt into instructions to complete the task
1. Using the instructions make requests to spotify

## Directories

- _scripts_: Contains some useful scripts for local development. The scripts all inject environment variables from a `Echo/.env` file prior to running the command
- _docker_: Contains the `Dockerfile`'s and docker-compose.yaml for the project
- _server_: Code for the `actix_web` server
- _www_: Code the for the `nextjs` web app

# TODO:

- [ ] debug server docker file and issue related to docker compose 
You replicate by running docker compose. some type of diesel and postgres adapter issue
- [ ] Make sure that we can get the user data
