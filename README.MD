# Data Node

Data Node is a robust data storage solution designed to support full-text search capabilities. It provides an API for efficient data retrieval and management, tailored for developers requiring scalable and searchable storage options.

## Features

- Full-text search support for complex queries
- API interface for seamless integration with other services
- Containerized environment for easy deployment and scalability

## Getting Started

These instructions will get your copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

Before you begin, ensure you have the following installed:

- Docker and Docker Compose
- Rust and Cargo (latest stable version)

### Local Development Build

To build the project locally with Docker, use the following command:

```bash
docker-compose up -d --build
```

This command builds the Docker images and starts the containers in detached mode.

### Running Locally

To run the Data Node project on your local machine, follow these steps:

1. Update dependencies:

```bash
cargo upgrade
```

2. Run the project:

```bash
cargo run
```

Alternatively, to run the project with debug logging enabled:

```bash
RUST_LOG=debug cargo run
```

### API Documentation

For detailed information on the API endpoints and their usage, refer to the `docs/api.md` file.

## Contributing

We welcome contributions to the Data Node project! Please read `CONTRIBUTING.md` for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/yourproject/datnode/tags).

## Acknowledgements

This project utilizes Redis Stack, developed by Redis Labs and its open-source community.

This project is intended for development and testing purposes and should be used responsibly.
