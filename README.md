# identity-service

#### What I want to build
An effcient identity provider service, that should be suitable to run in a cloud environment

#### Why 
I want to try to use rust for something web-related, because I think it would be fun. I also want to be better at understanding some concepts in rust that I am confident I will have to work with during this project.s

#### Data
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

Milestones: 
- [x] HTTP connection
- [ ] Connect to Mongo database with identity collection Standard Endpoints
* post identity-pool
* get /identity-pool/{pool-token}
- [ ] Expand Endpoints implement identities
* post /identity
* get /identity
* put /identity
* delete /identity
- [ ] Expand Endpoints implement authorization
* post identity-pool/roles
* put identity-pool/roles
* get identity-pool/roles
- [ ] Implement JWT authentication
- [ ] Docker webserver
- [ ] Docker MongoDB (strategy for testing too)
- [ ] docker-compose
- [ ] Integration tests
- [ ] CircleCI pipeline that runs tests