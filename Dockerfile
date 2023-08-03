FROM rust:1.70

WORKDIR /usr/src/receipts
COPY . .

RUN cd presentation && cargo install --path .

CMD ["presentation"]