# FROM node:16.13.1 AS client
# WORKDIR /app
# COPY ./client .
# RUN npm i
# RUN npm run build


# FROM rust:1.60 AS server
# WORKDIR /app
# COPY . .
# COPY --from=client /app/dist ./client/dist
# RUN cargo build --release
# CMD ["./target/release/project"]

FROM rust:1.66.1 as builder
RUN USER=root

# Create folder, copy files and build
RUN mkdir rust_api_project
WORKDIR /rust_api_project
ADD . ./
RUN cargo clean && cargo build --release

FROM ubuntu:20.04
WORKDIR /app

# Install MySQL client for Diesel
RUN apt update
# install certificates for https (firebase)
RUN apt install ca-certificates -y 
RUN apt install libmysqlclient-dev -y
RUN apt install libmariadbd-dev -y


# Copy the compiled binaries into the new layer.
COPY --from=builder /rust_api_project/target/release/rust_api_project .
COPY --from=builder /rust_api_project/Rocket.toml .

EXPOSE 8000
# Run the binary.
CMD ["./rust_api_project"] 
# ENTRYPOINT ["tail", "-f", "/dev/null"]

