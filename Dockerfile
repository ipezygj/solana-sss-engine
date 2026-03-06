# Technical implementation for Solana Stablecoin Standard V2.1.
FROM python:3.10-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y git curl && rm -rf /var/lib/apt/lists/*

# Copy SDK and requirements
COPY sdk/ /app/sdk/
COPY tests/ /app/tests/

# In a real environment, we would install solana-py etc.
RUN pip install --no-cache-dir unittest-xml-reporting

CMD ["python", "tests/test_compliance.py"]
