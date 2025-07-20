# W/ept

Built on  [W]ordPress and L[ept]os (originally), now giving Dioxus a go.
<br>

## Features

This list is a work in progress. Features are implemented based on what I feel like doing at the time.  

- [x] Setup GraphQL schema extracted from WordPress
- [x] Setup WordPress API access
- [x] Setup WooCommerce and GraphQL plugins
- [ ] Front-end:
    - [x] Create Home page routing and components
    - [x] Create Error page routing and components
    - [x] Create Post page routing and components
    - [x] Create Product page routing and components
    - [x] Create Category page routing and components
    - [x] Create the Categories page routing and components
    - [x] Create the Posts page routing and components
    - [x] Create the Page page routing and components
    - [x] Create the Pages page routing and components
    - [x] Create the Search page routing and components
    - [x] Create the Cart page routing and components
    - [ ] Create components:
        - [x] Footer
        - [x] Menu
        - [x] Mini Cart + state management
        - [ ] Get Quote form/component
        - [ ] Layered Navigation (subcategory navigation at least)
        - [ ] Pagination
    - [ ] Create the Contact page routing and components
    - [ ] Create the Gallery page routing and components
- [ ] Integrations:
    - [ ] SCSS
    - [ ] Contact Form 7 via REST API, or Ninja Forms via GraphQL
    - [ ] Google Analytics 4
    - [ ] Schema.org
    - [ ] reCAPTCHA Enterprise
- [ ] Update WordPress settings on install:
    - [ ] GraphQL > General Settings > Enable GraphQL Debug Mode
    - [ ] GraphQL > General Settings > Enable Public Introspection
    - [ ] GraphQL > CORS Settings > Add Site Address to "Access-Control-Allow-Origin" header
- [ ] Improved and consolidated error handling (WIP)
- [ ] Review critical SEO features
- [ ] Secure SSR
- [ ] CICD pipeline
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

### Test WordPress API

```shell
http POST http://localhost:8080/graphql \
    Content-Type:application/json \
    <<< '{
        "query": "{ generalSettings { url } }"
    }'
```

### Extract GraphQL Schema

Use a tool like [get-graphql-schema](https://github.com/prisma-labs/get-graphql-schema) to extract the schema from a WordPress site, after enabling WPGraphQL Public Introspection.  /

```shell
get-graphql-schema http://localhost:8080/graphql > schema.graphql
```
<br>
