FROM rust:latest AS build
RUN cargo install wasm-pack
RUN apt update && apt install -y npm
RUN npm install --global rollup
WORKDIR /build/
COPY ./ ./
RUN wasm-pack build --release --target web
RUN rollup ./main.js --format iife --file ./pkg/bundle.js

FROM nginx:latest
COPY --from=build /build/pkg /usr/share/nginx/html/pkg
COPY --from=build /build/static /usr/share/nginx/html/static
COPY --from=build /build/index.html /usr/share/nginx/html/index.html
COPY ./nginx/prod.conf /etc/nginx/nginx.conf
COPY ./nginx/mime.types /etc/nginx/mime.types
