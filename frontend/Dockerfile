FROM node:18-slim AS builder

WORKDIR /app
COPY package.json /app/
COPY package-lock.json /app/

RUN npm ci && npm cache clean --force

ADD . /app

RUN npm run build

FROM node:18-slim

WORKDIR /app
COPY --from=builder /app /app

ENV HOST 0.0.0.0
EXPOSE 3000

ENTRYPOINT ["node", ".output/server/index.mjs"]
