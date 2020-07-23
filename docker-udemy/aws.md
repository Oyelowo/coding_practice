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