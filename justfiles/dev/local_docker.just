local_docker:
  #!/usr/bin/env bash
  docker stop $(docker ps -aq)
  sudo chown -R 472:472 ./docker/grafana_data
  sudo chown -R 472:472 ./docker/provisioning
  docker compose --env-file .env -f docker/docker-compose.yml up -d
