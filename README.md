# IDS721 Spring 2023 Project 3 - Rust Serverless with AWS

Serverless S3 Backup using Lambda x Rust  


## What I Learnt

* Deploying AWS serverless using Lambda x S3 trigger


## Developer Setup

**Prequisites**
1. S3 Buckets x 2 for input and output. Input bucket has access point.

**Configure AWS IAM Role**

1. Create an AWS IAM User policy with `AWSLambda_FullAccess`, `AmazonS3FullAccess` permissions and added custom permission config
```
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "PermissionsToDeploy",
            "Effect": "Allow",
            "Action": [
                "iam:CreateRole",
                "iam:CreatePolicy",
                "iam:PutRolePolicy",
                "iam:AttachRolePolicy",
                "iam:UpdateAssumeRolePolicy"
            ],
            "Resource": "*"
        }
    ]
}
```

2. Create your ~/.aws/credentials file with environment variables: 
* aws_access_key_id
* aws_secret_access_key
* aws_role_arn
* region 

*Note: `aws_role_arn` is copied from the IAM user summary and is formatted as arn:aws:iam::<aws_acct>:user/<iam_user>*

**Install Cargo Lambda into Virtual Env**

```
$ python3 -m venv ~/.venv
$ source ~/.venv/bin/activate
$ make install
```

**Build Binary**

```
$ make release
```

**Deploy Lambda Fxn to AWS**
```
$ make deploy
```

## ToDos

- [ ] Deploy base serverless lambda function + S3 trigger for image resizing
- [ ] Additional augmentations: greyscale, rotations
- [ ] Unit testing (a la AWS Serverless Rust)
- [ ] CI/CD with spinup/teardown (a la Datadog)


## References

* [Cargo Lambda for Rust](https://www.cargo-lambda.info/)
* [AWS Lambda Events Docs for Rust](https://docs.rs/aws_lambda_events/latest/aws_lambda_events/index.html)
* [AWS Serverless Rust Demo](https://github.com/aws-samples/serverless-rust-demo/)
* [AWS Tutorial: S3 Object Lambda](https://aws.amazon.com/getting-started/hands-on/amazon-s3-object-lambda-to-dynamically-watermark-images/)