FROM bun:alpine
WORKDIR /app
COPY package*.json ./
RUN bun install
COPY . .


CMD ["bun", "run", "dev"]
EXPOSE 7004