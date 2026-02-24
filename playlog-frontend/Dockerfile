# syntax=docker/dockerfile:1

# =========================================
# Stage 1: Build the Angular Application with Bun
# =========================================

FROM oven/bun:1-alpine AS builder

WORKDIR /app

# dependency layer
COPY package.json bun.lock ./
RUN bun install --frozen-lockfile

# config layer
COPY angular.json tsconfig*.json ./

# source layer
COPY src ./src

RUN bun run build

# =========================================
# Stage 2: Nginx to Serve Static Files
# =========================================

FROM nginx:1.29-alpine AS runner

# Remove default config
RUN rm /etc/nginx/conf.d/default.conf

# Copy your custom nginx config
COPY nginx.conf /etc/nginx/nginx.conf

# Copy Angular build output
COPY --from=builder /app/dist/*/browser /usr/share/nginx/html

# Security: run as non-root
USER nginx

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]