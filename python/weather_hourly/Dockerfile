# Stage 1 build
FROM ghcr.io/astral-sh/uv:python3.14-trixie-slim AS builder
LABEL authors="jabbott-iii"

WORKDIR /app

# Enable bytecode compilation
ENV UV_COMPILE_BYTECODE=1
# Copy from the cache instead of linking since it's a mounted volume
ENV UV_LINK_MODE=copy
# Omit development dependencies
ENV UV_NO_DEV=1
# Ensure installed tools can be executed out of the box
ENV UV_TOOL_BIN_DIR=/usr/local/bin

# Install the project's dependencies using the lockfile and settings
RUN --mount=type=cache,target=/root/.cache/uv \
    --mount=type=bind,source=uv.lock,target=uv.lock \
    --mount=type=bind,source=pyproject.toml,target=pyproject.toml \
    uv sync --locked --no-install-project

# Then, add the rest of the project source code and install it
# Installing separately from its dependencies allows optimal layer caching
COPY . .
RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --locked


# Stage 2 production
FROM ghcr.io/astral-sh/uv:python3.14-trixie-slim

WORKDIR /app

COPY --from=builder /app/.venv .
COPY --from=builder /app/app .
COPY --from=builder /app/main.py .

ENV PATH="/opt/.venv/bin:${PATH}"
# Webserver port
ENV PORT=8000
EXPOSE 8000
# Erase default entry point set by docker automatically
ENTRYPOINT []

CMD ["uv", "run", "main.py", "--host", "0.0.0.0"]
