FROM node:20-alpine AS build

ENV NODE_OPTIONS="--max-old-space-size=4096"

WORKDIR /app

RUN apk add --no-cache \
    python3 \
    make g++  \
    gcc \
    libc-dev \
    linux-headers \
    libusb-dev \
    eudev-dev

COPY ./client/package*.json ./

RUN npm install

COPY ./client .

RUN npm run build

FROM nginx:1.27.1-alpine

COPY --from=build /app/dist /usr/share/nginx/html
COPY --from=build /app/env.sh /docker-entrypoint.d/env.sh
RUN chmod +x /docker-entrypoint.d/env.sh
CMD ["nginx", "-g", "daemon off;"]