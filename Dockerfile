FROM rust:slim

WORKDiR /app

COPY . .

ENV DISCORD_TOKEN="DISCORD TOKEN"

CMD ["cargo", "run", "--release"]
