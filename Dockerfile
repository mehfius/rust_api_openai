FROM docker.io/mehfius/rust-ngrok
WORKDIR /app

COPY target/release/$API_NAME ./$API_NAME

COPY entrypoint.sh ./entrypoint.sh

RUN chmod +x ./$API_NAME
RUN chmod +x ./entrypoint.sh

EXPOSE $PORT

ENTRYPOINT ["./entrypoint.sh"]
CMD []
