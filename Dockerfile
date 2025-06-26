FROM rust:latest 

RUN apt-get update && apt-get install -y libdbus-1-dev pkg-config && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/networkmanager-rs
COPY . .

CMD ["cargo", "build", "--release"]
