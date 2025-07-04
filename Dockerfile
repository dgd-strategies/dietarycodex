# Use Python 3.12 slim base image
FROM python:3.12-slim

# Set workdir
WORKDIR /app

# Install required system packages (minimal footprint)
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl build-essential && \
    rm -rf /var/lib/apt/lists/*

# Install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Optionally add pre-commit config
COPY .pre-commit-config.yaml .

# Copy project directories
COPY compute/ compute/
COPY tests/ tests/
# Copy the static HTML entry point and assets
COPY index.html index.html
COPY assets/ assets/
COPY data/template.csv data/template.csv
COPY sw.js sw.js
COPY docs/ docs/
COPY setup.sh setup.sh

# Disable Python bytecode, enable logs
ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

# Expose port for API + frontend
EXPOSE 8000

# Run setup script on container start
CMD ["bash", "./setup.sh"]
