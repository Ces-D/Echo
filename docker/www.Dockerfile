FROM node:23-alpine AS builder
WORKDIR /app

# Install pnpm globally
RUN npm install -g pnpm

# Copy package manifest and lockfile
COPY package.json pnpm-lock.yaml ./

# Install all dependencies (excluding devDependencies)
RUN pnpm install --frozen-lockfile

# Set environment variables
ARG NEXT_PUBLIC_API_URL
ENV NEXT_PUBLIC_API_URL=${NEXT_PUBLIC_API_URL}
ENV NODE_ENV=production

# Copy all source files
COPY . .

# Build the Next.js application (this step also runs Tailwind CSS processing)
RUN pnpm build


FROM builder AS runner

# Copy the built application and public assets from the builder stage
COPY --from=builder /app/.next ./.next
COPY --from=builder /app/public ./public

# Expose the port Next.js listens on
EXPOSE 3000

# Start the application
CMD ["pnpm", "start"]

