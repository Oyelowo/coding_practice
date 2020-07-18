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

to exit e.g redis-cli
`ctr + c`

shell(command processor) allows executing commands
to exit the shell(command processor) itself
`ctr + D` or `exit`

- useful sometimes if you want to poke around. But mostly, you want a primary process running first. With this, you will not be able to run any other process(not sure: 2020-07-18). So, likely, you'd want to start a primary process first, and then
 attach to the container using e.g `docker exec -it a5fee72ead8b sh`:
`docker run -it busybox sh`


-Two containers don't automatically share their file systems, unless specifically specify this. They are two isolated systems
e.g 
run `docker run -it busybox sh` on terminal 1 and terminal2
create a file in the first terminal e.g `touch somefile`
`docker ps` to check that the two containers exist with the `sh` command
goto the second terminal and execute `ls`. You will see that the file created in the first container does not show up in the second

# if using another file name e.g Dockerfile.dev, you need to use the .f flag
`docker build -f Dockerfile.dev .`



# Dockerfile - Making custom image
has some files in it which specifies how our program behaves.

This is then passed to the docker-cli which is then passed to docker-server which builds a usable image that can be used to start a container

## Steps
- Specify base image
- run some commands to install additional programs
- specify a command to run the container


- Tagging the image - building with a name
`docker build -t oyelowo/redis:<version> .`
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

# Mapping ports
Port redirection 8080:8080 = from source e.g in browswer(can be anything) : to port in the container (which is also being used in the web server). This is specified at run time
  `docker run -p 8080:8080 oyelowo/node-app `




# Copying files from local directory to container directory
The ./ is relative to the build context i.e in(`docker build .`)
`COPY <Current working directory> <image directory>`
`COPY ./ ./`


# Specifying a working directory
If you use the copy command above, it copies all files to the home 
directory of the container which is not ideal as their might be conflict with original files in the container. This could also cause this to overwrite exising files e.g
```
➜  node-app-docker git:(master) docker run -it oyelowo/nodeapp sh
/ # ls
Dockerfile         etc                media              package-lock.json  run                tmp
Dockerfile_old     home               mnt                package.json       sbin               usr
bin                index.js           node_modules       proc               srv                var
dev                lib                opt                root               sys
/ # 
```
In the above all the local files were copied to the container root directory.
By using `WORKDIR` command, we can specify and any command that follows will be executed relative to that path in the container.
e.g  and note that /usr/app will be created if it does not exist. We can also use `var` folder i.e /var/app
`WORKDIR /usr/app`

if we run `COPY ./ ./` after the below, the files in our local build context will be copied into the working directory in the container specied above



# Docker Run with PORT mapping for incoming requests
When request is being made to e.g `localhost:8080`, it is not automatically
forwarded to the container. We can use port mapping to forward the incoming request
coming from the browser to the container. E.g below we map 8080 to 8080. The previous could also be something else e.g 8080 to 5000. This is only for incoming requests. Docker can already make requests to the outside world. The port mapping is a runtime constraint. So, it only happens when we run/start a container

`docker run -p <route incoming requests to this port on local host to>:<the port inside the container> <image id>`

`docker run -p 5000:8080 <image id>`
i.e/e.g
`docker run -p <port from browser>:<port of  node app running in container> <image id>`


# Cache busting and rebuilds
When you do `COPY ./ ./` and `RUN npm install` after that, everytime you build the image, `npm install` will be run which will cause reinstallation
of packages even though nothing has changed in the `package.json` file.
e.g

BEFORE
```
FROM node:alpine

WORKDIR /usr

COPY ./ ./

RUN npm install

CMD ["npm", "start"]

```

AFTER
```
FROM node:alpine

WORKDIR /usr/app

# We broke the Copy step into two. We first copy package.json which will make it cache better. If we don't change the file, the cache will remain valid and we dont have to reinstall everything again
COPY ./package.json ./  
RUN npm install

# We then copy other files. Therefore, when we make changes to only our source code and not `package.json`, `npm install` won't be rerun everytime we rebuild the image
COPY ./ ./

CMD ["npm", "start"]
```

