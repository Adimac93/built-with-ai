#!/usr/bin/env bash
set -euo pipefail

PROJECT_ID="${PROJECT_ID:-built-with-ai-gdg-wroclaw}"
REGION="${REGION:-us-central1}"
AGENT_REGION="${AGENT_REGION:-us-east1}"
API_SERVICE="${API_SERVICE:-api}"
AGENT_SERVICE="${AGENT_SERVICE:-agent}"
IMAGE="gcr.io/${PROJECT_ID}/${API_SERVICE}"

echo "Resolving ${AGENT_SERVICE} Cloud Run URL in ${PROJECT_ID}/${AGENT_REGION}..."
AGENT_URL="$(
  gcloud run services describe "${AGENT_SERVICE}" \
    --platform managed \
    --region "${AGENT_REGION}" \
    --project "${PROJECT_ID}" \
    --format='value(status.url)'
)"

if [[ -z "${AGENT_URL}" ]]; then
  echo "Could not resolve AGENT_URL for Cloud Run service '${AGENT_SERVICE}'." >&2
  exit 1
fi

echo "Deploying ${API_SERVICE} with AGENT_URL=${AGENT_URL}"
docker build --no-cache --platform linux/amd64 -t "${IMAGE}" -f apps/api/Dockerfile .
docker push "${IMAGE}"
gcloud run deploy "${API_SERVICE}" \
  --image "${IMAGE}" \
  --platform managed \
  --region "${REGION}" \
  --allow-unauthenticated \
  --project "${PROJECT_ID}" \
  --update-env-vars "AGENT_URL=${AGENT_URL}"
