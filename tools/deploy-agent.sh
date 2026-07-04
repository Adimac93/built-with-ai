#!/usr/bin/env bash
set -euo pipefail

PROJECT_ID="${PROJECT_ID:-built-with-ai-gdg-wroclaw}"
REGION="${AGENT_REGION:-us-east1}"
AGENT_SERVICE="${AGENT_SERVICE:-agent}"
IMAGE="gcr.io/${PROJECT_ID}/${AGENT_SERVICE}"

echo "Deploying ${AGENT_SERVICE} to Cloud Run ${PROJECT_ID}/${REGION}"
docker build --platform linux/amd64 -t "${IMAGE}" -f apps/agent/Dockerfile .
docker push "${IMAGE}"
gcloud run deploy "${AGENT_SERVICE}" \
  --image "${IMAGE}" \
  --platform managed \
  --region "${REGION}" \
  --allow-unauthenticated \
  --project "${PROJECT_ID}" \
  --update-env-vars "GOOGLE_CLOUD_PROJECT=${PROJECT_ID},GOOGLE_CLOUD_LOCATION=global"
