# syntax=docker/dockerfile:1

FROM node:20-alpine AS deps
WORKDIR /app
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

FROM node:20-alpine AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY frontend .
# Build SSR server (output: server) using Node adapter config
RUN npm run build

FROM node:20-alpine AS runtime
WORKDIR /app
ENV NODE_ENV=production
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/package.json ./package.json
RUN npm pkg set scripts.start="node dist/server/entry.mjs" && npm pkg delete dev && npm pkg delete build || true
EXPOSE 4321
CMD ["node", "dist/server/entry.mjs"]


