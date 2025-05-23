#### BUILDER IMAGE  ####
FROM docker.io/node:18 as builder
LABEL maintainer="Xerberus Engineering Team <engineering@xerberus.io>"
LABEL description="Xerberus Telemetry Frontend builder image"

WORKDIR /opt/builder

COPY . .

RUN npm install && npm run build

#### OUTPUT IMAGE ####
FROM docker.io/nginx:stable-alpine
LABEL maintainer="Xerberus Engineering Team <engineering@xerberus.io>"
LABEL description="Xerberus Telemetry Frontend"

# Each time this container is ran, the value that's provided for this env var
# determines where the frontend will try to request feed information from:
ENV SUBSTRATE_TELEMETRY_URL="wss://feed.telemetry.polkadot.io/feed"

WORKDIR /app

COPY --from=builder /opt/builder/scripts/*.sh /usr/local/bin/
COPY --from=builder /opt/builder/build /app
COPY --from=builder /opt/builder/nginx/nginx.conf /etc/nginx/nginx.conf

RUN apk add --no-cache bash && \
    chown -R nginx:nginx /app && \
    chown -R nginx:nginx /var/cache/nginx && \
    chown -R nginx:nginx /var/log/nginx && \
    chown -R nginx:nginx /etc/nginx/conf.d && \
    touch /var/run/nginx.pid && \
    chown -R nginx:nginx /var/run/nginx.pid && \
    rm /etc/nginx/conf.d/default.conf

# UID= 101
USER nginx
EXPOSE 8000

CMD ["nginx", "-g", "daemon off;"]
