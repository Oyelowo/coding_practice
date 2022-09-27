# Installation

See [AWS-CLI-with-SeaweedFS](https://github.com/seaweedfs/seaweedfs/wiki/AWS-CLI-with-SeaweedFS#installation)

# Execute commands

## Pre-requisite

* Remove `-s3.config` params
* Create an admin through `weed shell`, and use these credentials to access IAM

e.g.:
```
s3.configure -apply -user admin -access_key some_access_key1 -secret_key some_secret_key1 -actions Admin
```

## Create S3 credentials 

Create user and access key
```
aws --endpoint http://127.0.0.1:8111 iam create-access-key --user-name Bob
{
    "AccessKey": {
        "UserName": "Bob",
        "AccessKeyId": "X8R439UM7OSQJX28I9QTP",
        "Status": "Active",
        "SecretAccessKey": "FLh9yeeYhzA7qsiyLIXsvuhv4g2cSgoUJJe/EqZw1z"
    }
}
```

Create read only access to the bucket
```
echo '
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "s3:Get*",
        "s3:List*"
      ],
      "Resource": [
        "arn:aws:s3:::EXAMPLE-BUCKET",
        "arn:aws:s3:::EXAMPLE-BUCKET/*"
      ]
    }
  ]
}
' > S3-read-only-example-bucket.policy
aws --endpoint http://127.0.0.1:8111 iam put-user-policy --user-name Bob --policy-name ExamplePolicy --policy-document file://S3-read-only-example-bucket.policy
```

Checking
```
echo 's3.configure' | weed shell
{
  "identities": [
    {
      "name": "Bob",
      "credentials": [
        {
          "accessKey": "X8R439UM7OSQJX28I9QTP",
          "secretKey": "FLh9yeeYhzA7qsiyLIXsvuhv4g2cSgoUJJe/EqZw1z"
        }
      ],
      "actions": [
        "Read:EXAMPLE-BUCKET",
        "List:EXAMPLE-BUCKET"
      ]
    }
  ]
}
```

## Show S3 credentials

List access keys
```
aws --endpoint http://127.0.0.1:8111 iam list-access-keys
{
    "AccessKeyMetadata": [
        {
            "UserName": "iam",
            "AccessKeyId": "B04R0WM64L0DAJ0N9LFZ",
            "Status": "Active"
        },
        {
            "UserName": "Bob",
            "AccessKeyId": "X8R439UM7OSQJX28I9QTP",
            "Status": "Active"
        }
    ]
}
```
