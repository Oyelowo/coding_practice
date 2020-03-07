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
