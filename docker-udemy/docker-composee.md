# WHY 
Docker compose is meant for configuring and running multiple containers without
doing everything via docker-cli. Makes it easy. Rather than writing everything in script

NOTE: docker-compose commands should be run from the directory where the docker-compose.yml lives.
This is to let it know which services to run
# launch 
`docker-compose up`

# rebuild and launch. 
This is necessary when you make a change to code in one of our images
`docker-compose up --build`

# launch in background
`docker-compose up -d`

# stop all containers
`docker-compose down`

# print status of all containers which should be run from current directory where docker-compose.yml is
`docker-compose ps`

# Restart policies

```
version: '3'
services: 
    redis-server:
        image: 'redis'
    node-app:
        restart: on-failure # RESTART POLICY: when no, it has to be string i.e 'no' because no is interpreted as false in yml file
        build: .
        ports: 
            - "4001:8081"
```
### SIDE NOTE
in `process.exit(0)`,
0 means: everything is okay, we just want to stop the process
1,2,3 or any other number means that something went wrong

'no' - never restart. NB: for 'no' policy, it has to be surrounded by the quote  because no is interpreted as false in yml file
- always: restart always if container stops. Good for web-service that we want available everytime

- on-failure: when container stops with error code. i.e other than 0. Other numbers (1 upwards) means error. Maybe good for worker process. E.g when a container is processing a file, you probably want to only restart if the process fails and not restart even when the
process is done

In this case, if you `process.exit(0)`, the container won't restart because status 0 means everything is okay, we only
exited because we wanted to. but with `process.exit(1)` or any other number than 0, this restart policy
will restart the container involved because status code other than 0 indicates failure(e.g 1,2,3 or 4)

- unless-stopped: always unless I specifically sop it



## WORKING WITH MULTIPLE CONTAINERS
Anytime you're creating client for e.g redis or driver for database, for the host, just use
the name of docker-compose service
e.g redis-server below is what is used in the docker-compose.yml file below
```
const client = redis.createClient({
  host: "redis-server",
  port: 6379 // This is default, not necessary as such
});
```

```
version: '3'
services: 
    redis-server:
        image: 'redis'
    node-app:
        restart: on-failure # when no, it has to be string i.e 'no'
        build: .
        ports: 
            - "4001:8081"
```


## DEALING WITH CONTAINERS THAT CRASH, RESTARTING A CONTAINER