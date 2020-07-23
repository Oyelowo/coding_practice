FROM node:alpine as builder
WORKDIR /app
COPY ./package.json .
RUN npm install
COPY . .
RUN npm run build

#/app/build will now have all the static files

# This terminates the previous block
FROM nginx
#copy the static files from the previous block
COPY --from=builder /app/build /usr/share/nginx/html

# Default command of nginx will start the container for us
# we dont have to specify the command ourselves