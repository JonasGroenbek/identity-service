# identity-service

What I want to build:
An effcient identity provider service, that should be suitable to run in a cloud environment

Why: 
I want to learn rust

Desired data structure 
```ts
enum Permissions {
    CreateIdentity,
    DeleteIdentity,
    ...
}

identity_pool {
    _id: Object.ID
    token: string,
    pool: Array<{
       _id: Object.ID
       email: string
       password: string
       role: string 
    }>
    roles: Array<{
        role: string,
        permissions: Array<Permissions>
    }>
}
```

Todo: 
- [ ] HTTP connection
- [ ] Connect to Mongo database with identity collection
Standard Endpoints
- post identity-pool
* get /identity-pool/{pool-token}
- [ ] Expand Endpoints implement identities
- post identity
* get /identity
- [ ] Expand Endpoints implement authorization
- post identity-pool/roles
- put identity-pool/roles
- get identity-pool/roles
- [ ] Docker + docker-compose
- [ ] Integration tests
- [ ] CircleCI pipeline that runs tests