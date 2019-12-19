FROM node AS build

WORKDIR /src
COPY package.json yarn.lock ./
RUN yarn install
COPY . .
RUN yarn run generate --fail-on-error

FROM nginx
COPY docker/entrypoint /usr/bin/entrypoint
COPY --from=build /src/dist /usr/share/nginx/html

EXPOSE 80

ENTRYPOINT ["/usr/bin/entrypoint"]
