
When we are running our react app locally, it uses the dev server
which serves static files(html, css, js) to the browser during 
development. However, the dev server is not available in production environment.

NGINX helps in this case, i.e helps to serve static content for production. It is used as below

## MULTIPLE STEPS DOCKER BUILD
```
# Block 1
# image alias
FROM node:alpine as builder
WORKDIR /app
COPY ./package.json .
RUN npm install
COPY . .
RUN npm run build
#/app/build will now have all the static files


# Block 2
# This terminates the previous block
FROM nginx

#copy the static files from the previous block(builder)'s directory - # `/app/build` to nginx' image directory - `/usr/share/nginx/html` (NB: this directory is mentioned/provided in the documentation in dockerhub). It is from there that nginx servers the static contents

COPY --from=builder /app/build /usr/share/nginx/html

# Default command of nginx will start the container for us
# we don't have to specify the command ourselves
```




## ROUTING TO CLIENT SERVER.
Check nginx.png for more info.

Nginx helps to direct app from browser to the appropriate server.
i.e react Server or Express server.

Static files (e.g /index.html, /main.js) are routed to React Server

while

Calls to server(e.g /api/values/all, /app/values/current) in our react codebase/app are routed to the express server.

Within Nginx we can just check what the request start with.
E.g
If it starts with `/`, route to react server.
If it starts with `/api`, route to the express server