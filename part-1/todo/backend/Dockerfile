# use the official Bun image
# see all versions at https://hub.docker.com/r/oven/bun/tags
FROM oven/bun:1 as base
WORKDIR /usr/src/app

# install dependencies into temp directory
# this will cache them and speed up future builds
FROM base AS install
RUN mkdir -p /temp/dev
COPY package.json bun.lockb /temp/dev/
RUN cd /temp/dev && bun install 

# install with --production (exclude devDependencies)
RUN mkdir -p /temp/prod
COPY package.json bun.lockb /temp/prod/
RUN cd /temp/prod && bun install --production

# copy node_modules from temp directory
# then copy all (non-ignored) project files into the image
FROM install AS prerelease
RUN mkdir -p /usr/src/app/public
RUN mkdir -p /tmp/kube
COPY --from=install /temp/dev/node_modules node_modules
COPY . .


# copy production dependencies and source code into final image
FROM base AS release
COPY --from=install /temp/prod/node_modules node_modules
COPY --from=prerelease /usr/src/app/src/index.ts .
COPY --from=prerelease /usr/src/app/package.json .
COPY ./public /usr/src/app/public
RUN mkdir -p /tmp/kube

# Set environment variables
ENV NODE_ENV=production

# run the app
EXPOSE 3000/tcp
ENTRYPOINT [ "bun", "run", "index.ts" ]
