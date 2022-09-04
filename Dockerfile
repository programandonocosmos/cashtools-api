FROM ubuntu

RUN apt-get update && apt-get install -y libpq-dev

COPY ./target/release/my-money ./my-money

ENV ROCKET_SECRET_KEY=12345678901234567890123456789012345678901234

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8080

CMD ["./my-money"]