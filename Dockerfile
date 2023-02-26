FROM rust:1.67 AS build
COPY . .
RUN cargo build --release
WORKDIR /dist
RUN mkdir lib \
    && mv $(ldd /target/release/ephemeris | grep libgcc_s.so.1 | awk '{print $3}') lib/


FROM gcr.io/distroless/base-debian11:latest
COPY --from=build /target/release/ephemeris /usr/local/bin/
COPY --from=build /dist/lib/* /usr/lib/
ENTRYPOINT ["ephemeris"]
