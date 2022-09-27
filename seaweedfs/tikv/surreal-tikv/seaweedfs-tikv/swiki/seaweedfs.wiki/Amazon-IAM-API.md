To be compatible with Amazon IAM API, a separate "weed iam" command is provided.

# How it works?
`weed iam` will start a stateless gateway server to bridge the Amazon IAM API to SeaweedFS Filer. 

# Supported APIs only POST actions

```
* CreateAccessKey
* ListAccessKeys
* DeleteAccessKey
* CreateUser
* ListUsers
* GetUser
* UpdateUser
* DeleteUser
* CreatePolicy
* PutUserPolicy
* GetUserPolicy
* DeleteUserPolicy
```

# Authentication

By default, the access key and secret key to access weed iam is not authenticated. To enable credential based access, create an admin credentials to the example below
```
echo 's3.configure -access_key some_access_key1 -secret_key some_secret_key1 -user iam -actions Admin -apply' | weed shell
```