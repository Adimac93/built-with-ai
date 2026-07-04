FROM docker.io/nginx:alpine

# Set default port if not provided by Cloud Run
ENV PORT=8080

COPY apps/frontend/nginx.conf /etc/nginx/conf.d/default.conf.template
COPY dist/apps/frontend/browser /usr/share/nginx/html

# Substitute $PORT in the nginx config template and start nginx
CMD ["/bin/sh", "-c", "envsubst '${PORT}' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'"]
