#!/bin/sh

set -o errexit
set -o xtrace

if [ "$GITHUB_EVENT_NAME" = "release" ] ; then
  export PROJECT_ID=dappface-prd-v2
	export APP_ENV=prd
elif [ "$GITHUB_EVENT_NAME" = "push" ] ; then
  if [ $(basename  "$GITHUB_REF") = 'master' ] ; then
    export PROJECT_ID=dappface-stg-v2
		export APP_ENV=stg
  else
    export PROJECT_ID=dappface-dev
		export APP_ENV=dev
  fi
else
	export PROJECT_ID=dappface-dev
	export APP_ENV=dev
fi

APP_NAME=api-rust
IMAGE_SRC_PATH=gcr.io/"$PROJECT_ID"/"$APP_NAME"

gcloud builds submit \
	--project "$PROJECT_ID" \
	--tag "$IMAGE_SRC_PATH" \
	--timeout 1200s

gcloud container images add-tag "$IMAGE_SRC_PATH" \
    "$IMAGE_SRC_PATH":"$GITHUB_SHA"

gcloud beta run deploy "$APP_NAME" \
	--project "$PROJECT_ID" \
	--image "$IMAGE_SRC_PATH":latest \
	--platform managed \
	--allow-unauthenticated \
	--region us-east1 \
	--update-env-vars APP_ENV="$APP_ENV",SLACK_API_TOKEN="$SLACK_API_TOKEN"
