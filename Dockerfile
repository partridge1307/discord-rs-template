FROM rust:slim

WORKDiR /app

COPY . .

ENV DISCORD_TOKEN="MTIxNDQ4NTAxOTk0MzI0MzgwNg.GZrdDk.81nH9k10fcqJx_jV8Nc92lqjjgb5stCXcbwNBI"

CMD ["cargo", "run", "--release"]
