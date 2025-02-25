# Use the official Bun image based on Alpine Linux
FROM oven/bun:alpine

# Set the working directory
WORKDIR /app

# Install Python, build tools, and other dependencies
RUN apk add --no-cache python3 py3-pip make g++

# Copy package.json and package-lock.json (if using npm)
COPY package*.json ./

# Install dependencies using Bun
RUN bun install

# Copy the rest of the application code
COPY . .

# Expose the port your app runs on
EXPOSE 7004

# Command to run the application
CMD ["bun", "run", "dev"]