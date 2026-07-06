#!/usr/bin/env bash
set -euo pipefail

PROJECT_ID="${PROJECT_ID:-built-with-ai-gdg-wroclaw}"
REGION="${REGION:-us-central1}"
FRONTEND_SERVICE="${FRONTEND_SERVICE:-frontend}"
IMAGE="gcr.io/${PROJECT_ID}/${FRONTEND_SERVICE}"

# Expects apps/frontend/dist/web to be staged (tools/build-frontend.sh).
echo "Deploying ${FRONTEND_SERVICE} to Cloud Run ${PROJECT_ID}/${REGION}"
docker build --platform linux/amd64 -t "${IMAGE}" -f apps/frontend/Dockerfile .
docker push "${IMAGE}"
gcloud run deploy "${FRONTEND_SERVICE}" \
  --image "${IMAGE}" \
  --platform managed \
  --region "${REGION}" \
  --allow-unauthenticated \
  --project "${PROJECT_ID}"
