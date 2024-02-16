FROM docker.io/debian:12.5

RUN mkdir /data /data/true /data/false /data/maybe
WORKDIR /data
COPY ./render_output/*true*.mp4 true
COPY ./render_output/*false*.mp4 false
COPY ./render_output/*maybe*.mp4 maybe
COPY adachi-bot /data/adachi-bot

ENV ADACHI_CONFIG_PATH="/data/config.toml"

ENTRYPOINT ["/data/adachi-bot"]
