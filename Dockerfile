# Stage 1: Build registration-listener
FROM golang:1.22.4 AS builder-registration-listener
WORKDIR /app
COPY cmd/mqtt-listener .
COPY internal/ internal/
COPY go.mod go.sum ./
RUN go mod download
RUN CGO_ENABLED=0 GOOS=linux go build -o /reg-listener

# Stage 2: Build registration-handler
FROM golang:1.22.4 AS builder-registration-handler
WORKDIR /app
COPY cmd/registration-handler .
COPY internal/ internal/
COPY go.mod go.sum ./
RUN go mod download
RUN CGO_ENABLED=0 GOOS=linux go build -o /reg-handler

# Stage 3: Create runtime image for registration-listener
FROM scratch AS runtime-registration-listener
COPY --from=builder-registration-listener /reg-listener .
COPY --from=builder-registration-listener /app/config.yaml .
# set the default environment variables
ENTRYPOINT ["/reg-listener"]

# # Stage 4: Create runtime image for registration-handler
FROM scratch AS runtime-registration-handler
COPY --from=builder-registration-handler /reg-handler .
ENV AMQP_URI: ${AMQP_URI}
ENV QUEUE_NAME: ${QUEUE_NAME}
ENTRYPOINT ["/reg-handler"]
