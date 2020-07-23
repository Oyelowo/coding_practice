# Elastic beanstalk
The web application is created and deployed to this

# IAM
Here, you can provide either programmatic access by e.g SDKs, CDKs etc or by directly login in to aws account with password
Used to manage api keys that is used for outside services. 
To use, 
- you can go to services > IAM 
- On the tabs, click on Users, to generate a new user that can be used for e.g travis ci.

Once, you generate the access key id and secret access key, you can then use this for the service (e.g travis).

But for security reasons, it makes more sense to use environmental variable in your configuration or codebase and then add the access key to the service provider website as environment variables
  

# PORT MAPPING: DOCKER EXPOSE
```
FROM node:alpine as builder

WORKDIR /app

COPY package.json .

RUN npm install

COPY . .

RUN npm run build


FROM nginx
# This does nothing automatically locally. More as a documentation. However, services like elasticbeanstalk 
# uses this EXPORT <port> information to map directly to
# otherwise, things wont work when we deploy our app because 
# we always need to give docker info about port mapping i.e something like
# docker run -p <port from outside>:<port in our container> (e.g docker run -p elasticBS-PORT:80)
# NB: nginx uses port 80 by default for our prod app
EXPOSE 80
COPY --from=builder /app/build /usr/share/nginx/html

```



# AWS Build Still Failing?
If you still see a failed deployment, try the following fixes:

1) Use an Unnamed Builder

Currently, there seems to be a bug with Elasticbeanstalk and the multi-stage builder step is failing. If you pull the AWS logs, you will see:

"docker pull" requires exactly 1 argument

The quick fix will be to use an unnamed builder, rather than a named builder:

By default, the stages are not named, and you refer to them by their integer number, starting with 0 for the first FROM instruction.
https://docs.docker.com/develop/develop-images/multistage-build/#name-your-build-stages

The Dockerfile would now look like this:

  FROM node:alpine
  WORKDIR '/app'
  COPY package*.json ./
  RUN npm install
  COPY . .
  RUN npm run build
 
  FROM nginx
  EXPOSE 80
  COPY --from=0 /app/build /usr/share/nginx/html
2) Upgrade to t2small instance

The npm install command frequently times out on the t2.micro instance that we are using.  An easy fix is to bump up the instance type that Elastic Beanstalk is using to a t2.small.

Note that a t2.small is outside of the free tier, so you will pay a tiny bit of money (likely less than one dollar if you leave it running for a few hours) for this instance.  Don't forget to close it down!  Directions for this are a few videos ahead in the lecture titled 'Environment Cleanup'.

3) Refactor COPY line

Try editing the 'COPY' line of your Dockerfile like so:

COPY package*.json ./

Sometimes AWS has a tough time with the '.' folder designation and prefers the long form ./