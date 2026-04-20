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
COPY public ./public

RUN bun run build

# =========================================
# Stage 2: Nginx to Serve Static Files
# =========================================

FROM nginxinc/nginx-unprivileged:1.29-alpine AS runner

# Set the filter to only substitute BACKEND_URL, preserving Nginx variables like $host
ENV NGINX_ENVSUBST_FILTER=BACKEND_URL

# Copy custom Nginx config
COPY --chown=nginx:nginx nginx.conf /etc/nginx/templates/nginx.conf.template

# Set the output directory for envsubst to the main nginx config location
ENV NGINX_ENVSUBST_OUTPUT_DIR=/etc/nginx

# Use root to prepare the config file permissions
USER root
RUN rm -f /etc/nginx/conf.d/*.conf && \
    touch /etc/nginx/nginx.conf && \
    chown nginx:nginx /etc/nginx/nginx.conf

# Copy the static build output from the build stage to Nginx's default HTML serving directory
COPY --chown=nginx:nginx --from=builder /app/dist/*/browser /usr/share/nginx/html

# Use a built-in non-root user for security best practices
USER nginx

# Expose port 8080 to allow HTTP traffic
# Note: The default Nginx container now listens on port 8080 instead of 80
EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]
