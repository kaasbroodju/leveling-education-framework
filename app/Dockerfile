# Install dependencies only when needed
FROM node:22-alpine AS deps
# Check https://github.com/nodejs/docker-node/tree/b4117f9333da4138b03a546ec926ef50a31506c3#nodealpine to understand why libc6-compat might be needed.
RUN apk add --no-cache libc6-compat
WORKDIR /app


# Install dependencies based on the preferred package manager
COPY package.json yarn.lock* package-lock.json* pnpm-lock.yaml* ./
RUN \
  if [ -f yarn.lock ]; then yarn --frozen-lockfile; \
  elif [ -f package-lock.json ]; then npm ci; \
  elif [ -f pnpm-lock.yaml ]; then yarn global add pnpm && pnpm i --frozen-lockfile; \
  else echo "Lockfile not found." && exit 1; \
  fi

# Rebuild the source code only when needed
FROM node:22-alpine AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .

RUN npx prisma generate
# Next.js collects completely anonymous telemetry data about general usage.
# Learn more here: https://nextjs.org/telemetry
# Uncomment the following line in case you want to disable telemetry during the build.
ENV NEXT_TELEMETRY_DISABLED 1
ENV USE_MOCK_DATA=true
# RUN yarn build

# If using npm comment out above and use below instead
RUN npm run build


# Production image, copy all the files and run next
FROM node:22-alpine AS runner
WORKDIR /app
ENV USE_MOCK_DATA=false
ENV NODE_ENV production
# Uncomment the following line in case you want to disable telemetry during runtime.
ENV NEXT_TELEMETRY_DISABLED 1

RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

COPY --from=builder /app/public ./public

# Automatically leverage output traces to reduce image size
# https://nextjs.org/docs/advanced-features/output-file-tracing
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

# Copy package.json (needed for scripts)
COPY --from=builder /app/package.json ./package.json

# Copy the entire node_modules to ensure Prisma and all dependencies are available
COPY --from=builder --chown=nextjs:nodejs /app/node_modules ./node_modules

# Copy Prisma schema & migrations
COPY --from=builder /app/prisma ./prisma

# Install runtime dependencies for Prisma
RUN apk add --no-cache libc6-compat openssl

# Set proper permissions on the app directory
RUN chown -R nextjs:nodejs /app

USER nextjs

EXPOSE 3000

ENV PORT 3000

# Run Prisma migrations before starting the app
CMD ["sh", "-c", "node -e \"const { execSync } = require('child_process'); try { execSync('npx prisma migrate deploy', {stdio: 'inherit'}); } catch (error) { console.error('Migration failed but continuing startup'); }\" && node server.js"]