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

- useful sometimes if you want to poke around. But mostly, you want a primary process running first
`docker run -it busybox sh`



-Two containers dont automatically share their file systems, unless specifically specify this. They are two isolated systems


# Dockerfile - Making custom image
has some files in it which specifies how our program behaves.

This is then passed to the docker-cli which is then passed to docker-server which builds a usable image that can be used to start a container

## Steps
- Specify base image
- run some commands to install additional programs
- specify a command to run the container


- Tagging the image - building with a name
`docker build -t oyelowo/redis:latest .`

oyelowo - dockerhub name of repository
redis - project
latest - version tag


## Manually creating image from a container -(not a good practise, mostly)
e.g download and run apline image interactively
`docker run -it alpine sh`

- run the commands you want as you would declare in a docker file e.g
  `apk add --update redis`

-open another terminal and check running container
`docker ps`

-copy the container from the earlier run image and commit it
`docker commit -c 'CMD ["redis-server"]' cb9914566f0a`

- once it is committed, a long sha string is generated e.g sha256:86ac1d2cb455580f34f9d7c001de9b0b8e888e836b8d6035a261da797929cfbf
- This can then be run e.g 
- Note: a part of the string can be run. Docker would run it anyways if it is unique.
  `docker run sha256:86ac1d2cb455580f34f9d7c001de9b0b8e888e836b8d6035a261da797929cfbf`


  alpine- an image that is as small as possible, with not much programs already installed. so stripped down version of an image