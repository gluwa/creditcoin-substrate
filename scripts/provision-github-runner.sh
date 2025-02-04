#!/bin/bash

mkdir actions-runner
pushd actions-runner || exit 1

curl -L https://github.com/actions/runner/releases/download/v2.317.0/actions-runner-linux-x64-2.317.0.tar.gz > runner.tar.gz

tar xzf ./runner.tar.gz
sudo ./bin/installdependencies.sh
sudo apt install -y jq

OWNER_REPO_SLUG="${LC_OWNER_REPO_SLUG}"
REPOSITORY_URL="https://github.com/$OWNER_REPO_SLUG"
EPHEMERAL=${LC_RUNNER_EPHEMERAL:-true}

# we need a temporary registration token first
REGISTRATION_TOKEN=$(curl --silent -X POST \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer $LC_GITHUB_REPO_ADMIN_TOKEN" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    "https://api.github.com/repos/$OWNER_REPO_SLUG/actions/runners/registration-token" | jq -r '.token')

if [ "$REGISTRATION_TOKEN" == "null" ]; then
    echo "ERROR: REGISTRATION_TOKEN is null"
    exit 1
fi

if [ -z "$REGISTRATION_TOKEN" ]; then
    echo "ERROR: REGISTRATION_TOKEN is empty"
    exit 2
fi

# Important: ephemeral runners are removed after a single job is executed on them
# which is inline with the VM lifecycle
./config.sh --unattended --ephemeral "$EPHEMERAL" --url "$REPOSITORY_URL" --token "$REGISTRATION_TOKEN" \
    --name "$LC_RUNNER_VM_NAME" \
    --labels "$LC_RUNNER_VM_NAME,workflow-$LC_WORKFLOW_ID,proxy-$LC_PROXY_ENABLED,secret-$LC_PROXY_SECRET_VARIANT,type-$LC_PROXY_TYPE"
nohup ./run.sh >/dev/null 2>&1 </dev/null &
