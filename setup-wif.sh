#!/usr/bin/env bash
set -e

PROJECT_ID="built-with-ai-gdg-wroclaw"
REPO="Adimac93/built-with-ai"
SA_NAME="github-actions-sa"
POOL_NAME="github-actions-pool"
PROVIDER_NAME="github-provider"

echo "1. Enabling IAM Credentials API..."
gcloud services enable iamcredentials.googleapis.com --project $PROJECT_ID

echo "2. Creating Service Account..."
# Ignore error if it already exists
gcloud iam service-accounts create $SA_NAME \
  --display-name="GitHub Actions Service Account" \
  --project $PROJECT_ID || true

SA_EMAIL="${SA_NAME}@${PROJECT_ID}.iam.gserviceaccount.com"

echo "3. Granting roles to Service Account..."
gcloud projects add-iam-policy-binding $PROJECT_ID \
  --member="serviceAccount:${SA_EMAIL}" \
  --role="roles/run.admin" --condition=None >/dev/null

gcloud projects add-iam-policy-binding $PROJECT_ID \
  --member="serviceAccount:${SA_EMAIL}" \
  --role="roles/iam.serviceAccountUser" --condition=None >/dev/null

gcloud projects add-iam-policy-binding $PROJECT_ID \
  --member="serviceAccount:${SA_EMAIL}" \
  --role="roles/artifactregistry.writer" --condition=None >/dev/null

gcloud projects add-iam-policy-binding $PROJECT_ID \
  --member="serviceAccount:${SA_EMAIL}" \
  --role="roles/storage.admin" --condition=None >/dev/null

echo "4. Creating Workload Identity Pool..."
gcloud iam workload-identity-pools create $POOL_NAME \
  --project=$PROJECT_ID \
  --location="global" \
  --display-name="GitHub Actions Pool" || true

echo "5. Creating Workload Identity Provider..."
gcloud iam workload-identity-pools providers create-oidc $PROVIDER_NAME \
  --project=$PROJECT_ID \
  --location="global" \
  --workload-identity-pool=$POOL_NAME \
  --display-name="GitHub Actions Provider" \
  --attribute-mapping="google.subject=assertion.sub,attribute.actor=assertion.actor,attribute.repository=assertion.repository" \
  --attribute-condition="assertion.repository == '${REPO}'" \
  --issuer-uri="https://token.actions.githubusercontent.com" || true

echo "6. Binding Service Account to GitHub Repo..."
PROJECT_NUMBER=$(gcloud projects describe $PROJECT_ID --format='value(projectNumber)')

gcloud iam service-accounts add-iam-policy-binding $SA_EMAIL \
  --project=$PROJECT_ID \
  --role="roles/iam.workloadIdentityUser" \
  --member="principalSet://iam.googleapis.com/projects/${PROJECT_NUMBER}/locations/global/workloadIdentityPools/${POOL_NAME}/attribute.repository/${REPO}" >/dev/null

echo "=========================================="
echo "Workload Identity Provider Name:"
echo "projects/${PROJECT_NUMBER}/locations/global/workloadIdentityPools/${POOL_NAME}/providers/${PROVIDER_NAME}"
echo "=========================================="
