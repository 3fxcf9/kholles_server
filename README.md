# Kholles Server

A web application for viewing structured math proofs, created for the MPSI1 class at Fermat High School in Toulouse.

The goal of this project is to display proofs organized by weeks, using data from a [separate repository](https://github.com/3fxcf9/kholles_content). A GitHub webhook is configured to trigger data refreshes whenever new content is pushed to the repository.

## Installation

1. Clone this repository to your local environment.

2. Clone the compatible data repository into the `FS_LISTEN_PATH` directory (set to `content` by default):

   ```bash
   git clone https://github.com/3fxcf9/kholles_content content
   ```

   This repository will serve as the data source for the website.

3. Set up a GitHub webhook to trigger on push events. Use the server's address with the `/events/push` path to trigger an update whenever new content is pushed to the data repository.

4. Build and start the docker container with the following commands:

```bash
echo "GITHUB_WEBHOOK_SECRET=******" > .env         # Set your GitHub webhook secret for validation

DOCKER_BUILDKIT=1 docker build -t kholles_server . # Build the Docker image

docker compose up -d                               # Start the Docker container
```

> [!IMPORTANT]
> Ensure Docker Buildx is installed and enabled on your system before building.

## Configuration

- The server's listen address, port, and the `FS_LISTEN_PATH` (directory path for the data repository) are all configurable in the `docker-compose.yml` file.
- Ensure that the `.env` file contains the correct GitHub webhook secret to properly validate incoming webhook requests.
