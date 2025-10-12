FROM node:20-alpine

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy source code
COPY . .

# Build TypeScript
RUN npm run build

# Create non-root user
RUN addgroup -g 1001 -S nodejs
RUN adduser -S discord-bot -u 1001

# Change ownership of the app directory
RUN chown -R discord-bot:nodejs /app
USER discord-bot

# Expose port (for health checks if needed)
EXPOSE 3003

# Start the bot
CMD ["npm", "start"]
