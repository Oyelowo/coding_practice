`docker create hello-world`

`docker start -a long-text-create-from above`

`docker run -it hello-world`

`docker ps`

`docker ps --all`

## delete all eexisting created containers
docker system prune

`docker logs <container-id>`


SIGTERM - terminate signals. This gives some additional time to shut down. If container doesnt shutdown in 10 seconds, it falls back to docker kill

SIGKILL - kill signal. Shut down right now without doing any additional work


`docker stop`

`docker kill`


`docker exec -it <container-id> <command>`

sh is name of a program being run in the container. It is a shell that allows us run a program in the container. There are also
bash,powershell, zsh and sh. Most containers have sh already included and bash sometimes also.
`docker exec -it a5fee72ead8b sh`