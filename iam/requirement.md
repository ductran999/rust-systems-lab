# Authenticate Mechanism

## Requirement
- Should be stateless -> ensure all requests are treated the same so we can scale easier.
- Should be centralize -> Easier for manage 

## Solution

OAuth:
- `/authorize:` path client provide credential to get code
- `/token`: patch exchange code to token pairs (access token and refresh token)
- `/refresh`: refresh token
- `/userinfo`: get user profile from access token
- `/revoke`: revoke token
- `/introspect`: verify token

