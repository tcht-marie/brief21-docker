FROM ubuntu
WORKDIR /app
COPY ./target/release/shop_bin ./shop_bin
CMD [ "./shop_bin" ]
