# BitBuilder Cloud CLI Deployment Guide

## Introduction

This guide provides comprehensive instructions for deploying applications and infrastructure using BitBuilder Cloud CLI (bbctl). It covers deployment workflows, configuration options, automation techniques, and best practices for various deployment scenarios.

## Deployment Concepts

BitBuilder Cloud CLI is designed around a consistent infrastructure-as-code approach to deployments:

- **Resources**: The building blocks of your infrastructure (instances, volumes, networks)
- **Templates**: Reusable configurations for deployment
- **Environments**: Distinct deployment targets (development, staging, production)
- **Workspaces**: Isolated deployment contexts for multi-tenant usage

### Deployment Workflow

The typical deployment workflow consists of:

1. **Define**: Create deployment configuration and resources
2. **Validate**: Verify configuration and check dependencies
3. **Deploy**: Provision resources and deploy applications
4. **Configure**: Apply post-deployment configuration
5. **Verify**: Confirm successful deployment
6. **Monitor**: Track performance and health

## Deployment Configuration

### Configuration File Format

BitBuilder Cloud CLI uses TOML configuration files for deployments. The main deployment file is typically named `deploy.toml`:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[app]
name = "my-web-app"
version = "1.0.0"
description = "Web application deployment"

[infrastructure]
provider = "vyos-pe1"
region = "nyc"

[instances]
count = 2
size = "standard-2x"
image = "debian-11"

[networks]
name = "web-tier"
cidr = "10.0.1.0/24"
type = "routed"

[volumes]
data = { size = 100, type = "ssd" }

[services]
enable_loadbalancer = true
subdomain = "web-app"
```

### Environment-Specific Configuration

For environment-specific configurations, use separate files or environment sections:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[environments.development]
instances = { count = 1, size = "small" }
enable_metrics = false

[environments.production]
instances = { count = 3, size = "large" }
enable_metrics = true
volumes.data.size = 500
```

## Basic Deployments

### Creating a Deployment

1. Initialize a new project:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
bbctl init --name my-web-app
```

2. Create a `deploy.toml` file in the project directory

3. Deploy the application:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
bbctl deploy
```

### Deployment Options

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
# Deploy with a specific configuration file
bbctl deploy --config custom-deploy.toml

# Deploy to a specific environment
bbctl deploy --env production

# Dry-run to validate without deploying
bbctl deploy --dry-run

# Force deployment (skip confirmation)
bbctl deploy --force
```

## Advanced Deployment Features

### Multi-Stage Deployments

For complex applications with dependencies, use multi-stage deployments:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[stages]
order = ["infrastructure", "database", "application", "monitoring"]

[stages.infrastructure]
resources = ["networks", "security"]

[stages.database]
resources = ["db-instances", "db-volumes"]
depends_on = ["infrastructure"]

[stages.application]
resources = ["app-instances", "lb"]
depends_on = ["database"]

[stages.monitoring]
resources = ["metrics", "alerts"]
depends_on = ["application"]
```

### Rolling Deployments

Minimize downtime using rolling deployments:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[deployment.strategy]
type = "rolling"
batch_size = 1
batch_interval = "30s"
health_check = "/health"
timeout = "5m"
```

### Blue-Green Deployments

Implement blue-green deployment strategy:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[deployment.strategy]
type = "blue-green"
traffic_shift = "instant" # or "gradual"
verification_period = "2m"
rollback_on_failure = true
```

## Infrastructure as Code Integration

### Terraform Integration

For Terraform integration:

1. Install the bbctl Terraform provider:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
terraform init -plugin-dir=~/.terraform.d/plugins
```

2. Create a Terraform configuration using bbctl resources:

<<<<<<< Updated upstream
```hcl
=======
<<<<<<< HEAD
``` hcl
=======
```hcl
>>>>>>> chore/bisect
>>>>>>> Stashed changes
provider "bbctl" {
  config_path = "~/.bbctl/config.toml"
}

resource "bbctl_instance" "web" {
  name     = "web-server"
  provider = "vyos-router"
  region   = "nyc"
  size     = "standard"
  count    = 2
}
```

3. Apply the Terraform configuration:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
terraform apply
```

### Pulumi Integration

For Pulumi integration:

<<<<<<< Updated upstream
```typescript
import * as bbctl from '@pulumi/bbctl';
=======
<<<<<<< HEAD
``` typescript
=======
```typescript
>>>>>>> chore/bisect
import * as bbctl from "@pulumi/bbctl";
>>>>>>> Stashed changes

const network = new bbctl.Network('app-network', {
  cidr: '10.0.0.0/24',
  provider: 'vyos-router',
  region: 'nyc',
});

const instance = new bbctl.Instance('web-server', {
  provider: 'vyos-router',
  region: 'nyc',
  size: 'standard',
  networks: [network.id],
});

export const instanceIp = instance.publicIp;
```

## CI/CD Integration

### GitHub Actions Integration

Example GitHub Actions workflow:

<<<<<<< Updated upstream
```yaml
=======
<<<<<<< HEAD
``` yaml
=======
```yaml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
name: Deploy Application

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install bbctl
        uses: bitbuilder-io/setup-bbctl@v1
        with:
          version: "latest"

      - name: Configure bbctl
        run: |
          mkdir -p ~/.bbctl
          echo "${{ secrets.BBCTL_CONFIG }}" > ~/.bbctl/credentials.toml

      - name: Deploy
        run: bbctl deploy --env production
```

### GitLab CI Integration

Example GitLab CI pipeline:

<<<<<<< Updated upstream
```yaml
=======
<<<<<<< HEAD
``` yaml
=======
```yaml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
stages:
  - test
  - build
  - deploy

deploy:
  stage: deploy
  image: bitbuilder/bbctl:latest
  script:
    - mkdir -p ~/.bbctl
    - echo "$BBCTL_CONFIG" > ~/.bbctl/credentials.toml
    - bbctl deploy --env production
  only:
    - main
```

## Application Configuration Management

### Environment Variables

Inject environment variables into your instances:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[instances.web.env]
DATABASE_URL = "postgres://user:pass@db.internal:5432/mydb"
REDIS_HOST = "redis.internal"
LOG_LEVEL = "info"
```

### Configuration Files

Deploy configuration files to instances:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[instances.web.files]
"/etc/nginx/nginx.conf" = { source = "./configs/nginx.conf" }
"/etc/app/config.json" = { content = '{"debug": false, "port": 3000}' }
```

### Secrets Management

Secure handling of sensitive information:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[secrets]
provider = "vault"
path = "secret/my-app"

[instances.web.secrets]
API_KEY = "vault:secret/my-app#api_key"
DB_PASSWORD = "vault:secret/my-app#db_password"
```

## Multi-Region Deployments

Deploy across multiple regions:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[regions]
enabled = ["nyc", "sfo", "fra"]
strategy = "all" # or "weighted"

[regions.nyc]
weight = 60
instances = { count = 3 }

[regions.sfo]
weight = 30
instances = { count = 2 }

[regions.fra]
weight = 10
instances = { count = 1 }
```

## High Availability Deployments

Configure highly available deployments:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[availability]
zones = ["a", "b", "c"]
distribution = "spread"

[instances]
count = 6 # 2 instances per zone

[database]
replicas = 3
failover = "automatic"
```

## Monitoring and Logging

Configure monitoring for deployments:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[monitoring]
enable = true
provider = "prometheus"
endpoints = ["/metrics"]
scrape_interval = "15s"

[logging]
driver = "fluentd"
options = { tag = "app-logs" }
```

## Deployment Testing

### Pre-deployment Testing

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[testing.pre_deployment]
enabled = true
command = "./scripts/pre-deploy-test.sh"
timeout = "2m"
fail_on_error = true
```

### Smoke Testing

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[testing.smoke]
enabled = true
endpoints = [
  { url = "/health", expect_status = 200 },
  { url = "/api/status", expect_contains = "running" },
]
timeout = "30s"
retries = 3
```

### Load Testing

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[testing.load]
enabled = true
tool = "k6"
script = "./tests/load-test.js"
vus = 50
duration = "1m"
threshold = "p95(http_req_duration) < 200"
```

## Security and Compliance

### Security Configurations

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[security]
ssl_enabled = true
certificate = "acme"
domains = ["app.example.com"]
waf_enabled = true
headers = {
  "Strict-Transport-Security" = "max-age=31536000; includeSubDomains",
  "Content-Security-Policy" = "default-src 'self'"
}
```

### Compliance Checks

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[compliance]
enabled = true
standards = ["pci-dss", "gdpr"]
automatic_remediation = true
scans = ["vulnerability", "configuration"]
```

## Deployment Rollbacks

To roll back to a previous deployment:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
# List deployments
bbctl deployments list

# Roll back to a specific deployment
bbctl deployments rollback d-01234567

# Roll back to the previous deployment
bbctl deployments rollback --previous
```

## Troubleshooting Deployments

### Common Issues and Solutions

1. **Resource Provisioning Failures**

- Check provider connectivity
- Verify resource limits and quotas
- Review error logs with `bbctl logs get d-01234567`

2. **Network Configuration Issues**

- Verify CIDR blocks don't overlap
- Ensure security groups allow required traffic
- Check DNS resolution with `bbctl network test-dns net-01234567`

3. **Application Deployment Failures**

- Validate application configuration
- Check for dependency issues
- Examine application logs with `bbctl instances logs i-01234567`

### Deployment Logs

Access deployment logs:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
# Get summary of deployment logs
bbctl deployments logs d-01234567

# Get detailed logs with timestamps
bbctl deployments logs d-01234567 --detailed --timestamps

# Stream logs in real-time
bbctl deployments logs d-01234567 --follow
```

## Conclusion

BitBuilder Cloud CLI provides a powerful platform for deploying and managing infrastructure and applications across multiple providers. By following this deployment guide, you can create efficient, repeatable, and scalable deployment workflows that support your application needs.

## Additional Resources

- [User Guide] - Comprehensive usage instructions
- [Command Reference] - Detailed command documentation
- [Configuration Guide] - Configuration file reference
- [Architecture Design] - System architecture details

[User Guide]: user-guide.md
[Command Reference]: command-reference.md
[Configuration Guide]: configuration-guide.md
[Architecture Design]: ARCHITECTURE_DESIGN.md
