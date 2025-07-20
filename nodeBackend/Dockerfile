# Build stage
FROM node:18 AS build

WORKDIR /usr/src/app

COPY package*.json ./
RUN npm install

COPY . .

# If you have a build step (e.g., npm run build), do it here:
# RUN npm run build

# Production stage
FROM node:18-alpine

WORKDIR /usr/src/app

# Copy only the necessary files from build stage
COPY --from=build /usr/src/app .

EXPOSE 3000

CMD ["node", "index.js"]

