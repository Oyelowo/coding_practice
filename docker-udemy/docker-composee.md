# launch 
`docker-compose up`

# rebuild and launch 
`docker-compose up --build`

# launch in background
`docker-compose up -d`

# stop all containers
`docker-compose down`

# print status of all containers which should be run from current directory where docker-compose.yml is
`docker-compose ps`

# Restart policies
'no' - never restart
always: restart always if container stops. Good for web-service that we want available everytime
on-failure: when container stops with error code. i.e 0. Other numbers 1 upwards, isnt error. Maybe good for worker process
unless-stopped: always unless I specifically sop it



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