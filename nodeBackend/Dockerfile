# Build stage
FROM node:18 AS build

WORKDIR /usr/src/app

COPY package*.json ./
RUN npm install

COPY ../static .
COPY . .

FROM node:18-alpine

WORKDIR /usr/src/app

COPY --from=build /usr/src/app .

EXPOSE 3000

CMD ["node", "index.js"]

