# specify a baseimage
FROM node:alpine

# specify working directory
WORKDIR /usr/app

# install some dependencies. package.json is first copied to prevent
# npm install from running again even if we dont have any new dependency but change one of our files
# prevent the invalidation of package.json when not necessary
COPY ./package.json ./
RUN npm install

COPY ./ ./

# Default command
CMD [ "npm", "start" ]