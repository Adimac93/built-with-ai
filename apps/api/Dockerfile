FROM docker.io/node:20-alpine

WORKDIR /app

COPY dist/apps/api/package.json ./

RUN npm install --omit=dev --ignore-scripts

COPY dist/apps/api .

ENV PORT=3000
EXPOSE 3000

CMD ["node", "main.js"]
