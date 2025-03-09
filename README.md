# W/ept

Built on  [W]ordPress and L[ept]os (originally), now giving Dioxus a go.
<br>

## Todo
- [ ] Create schema.graphql using the wordpress example
- [ ] Test WordPress API access
- [ ] Add woocommerce and graphql plugin
- [ ] Add products schema
- [ ] Update WP settings on install:
    - [ ] GraphQL > General Settings > Enable GraphQL Debug Mode
    - [ ] GraphQL > General Settings > Enable Public Introspection
    - [ ] GraphQL > CORS Settings > Add Site Address to "Access-Control-Allow-Origin" header

```shell
http POST http://localhost:8080/graphql \
    Content-Type:application/json \
    <<< '{
        "query": "{ generalSettings { url } }"
    }'
```
<br>

## Usage

### Run

To serve the application, run the following command.  

```shell
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
dx serve --port 3000 --addr 0.0.0.0
```

### Build

To build the application, run the following command.  

```shell
dx build
```

To build for release, run the following command.  

```shell
dx build --release
```
<br>

## GraphQL

### Extract GraphQL Schema

Use a tool like [get-graphql-schema](https://github.com/prisma-labs/get-graphql-schema) to extract the schema from a WordPress site, after enabling WPGraphQL Public Introspection.  /

```shell
get-graphql-schema http://localhost:8080/graphql > schema.graphql
```
<br>
