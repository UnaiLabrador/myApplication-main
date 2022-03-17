#!/bin/bash
# Copyright (c) Aptos
# SPDX-License-Identifier: Apache-2.0

function usage {
  echo "Usage:"
  echo "build and properly tags images for deployment to docker hub and/or ecr"
  echo "build_push.sh [-p] -g <GITHASH> -b <TARGET_BRANCH> -n <image name> [-u]"
  echo "-p indicates this a prebuild, where images are built and pushed to dockerhub with an prefix of 'pre_', should be run on the 'auto' branch, trigger by bors."
  echo "-b the branch we're building on, or the branch we're targeting if a prebuild"
  echo "-n name, one of init, faucet, validator, validator-tcb, forge"
  echo "-u 'upload', or 'push' the docker images will be pushed to dockerhub, otherwise only locally tag"
  echo "-o the org to target on dockerhub.  Defaults to 'aptos'"
  echo "should be called from the root folder of the aptos project, and must have it's .git history"
}

PREBUILD=false;
INPUT_NAME=
BRANCH=
ORG=aptos
PUSH=false

#parse args
while getopts "pb:n:o:u" arg; do
  case $arg in
    p)
      PREBUILD="true"
      ;;
    n)
      INPUT_NAME=$OPTARG
      ;;
    b)
      BRANCH=$OPTARG
      ;;
    u)
      PUSH="true"
      ;;
    o)
      ORG=$OPTARG
      ;;
    *)
      usage;
      exit 0;
      ;;
  esac
done

GIT_REV=$(git rev-parse --short=8 HEAD)

[ "$BRANCH" != "" ] || { echo "-b branch must be set"; usage; exit 99; }
[ "$GIT_REV" != "" ] || { echo "Could not determine git revision, aborting"; usage; exit 99; }
[ "$INPUT_NAME" != "" ] || { echo "-n name must be set"; usage; exit 99; }

PULLED="-1"

#Compute the tag name generated by the docker files.
LOCAL_TAG=${INPUT_NAME//-/_}
LOCAL_TAG=$ORG/${LOCAL_TAG}

# BEGIN DEALING WITH INCONSISTENT NAMING
DIR=$INPUT_NAME
if [ "$INPUT_NAME" == "validator-tcb" ]; then
  DIR="safety-rules"
fi
#END DEALING WITH INCONSISTENT NAMING

#Convert dashes to underscores to get tag names
tag_name=${INPUT_NAME//-/_}

#The name of the docker image built in the "auto" branch
PRE_NAME=$ORG/${tag_name}:pre_${BRANCH}_${GIT_REV}
#the name of the docker image build in the release branch
PUB_NAME=$ORG/${tag_name}:${BRANCH}_${GIT_REV}

#If not a prebuild *attempt* to pull the previously built image.
if [ $PREBUILD != "true" ]; then
  docker pull "$PRE_NAME"
  export PULLED=$?
fi

success=-1;

#if a prebuild, always -1, else if "docker pull" failed build the image.
if [ "$PULLED" != "0" ]; then
  docker/$DIR/build.sh
  echo retagging "${LOCAL_TAG}" as "$PRE_NAME"
  docker tag "${LOCAL_TAG}" "$PRE_NAME"
  #push our tagged prebuild image if this is a prebuild.  Usually means this is called from bors' auto branch.
  if [ $PREBUILD == "true" ]; then
    if [ $PUSH == "true" ]; then
      echo pushing "$PRE_NAME"
      docker trust sign "$PRE_NAME"
      docker push --disable-content-trust=false "$PRE_NAME"
      pushed="$?"
      if [[ "$pushed" == "0" ]]; then
         success=0
      fi
    else
      echo Dry run, Not pushing "$PRE_NAME"
    fi
  fi
fi

#if not a prebuild tag and push, usually means this is called from a release branch
if [ $PREBUILD != "true" ]; then
  echo retagging "$PRE_NAME" as "$PUB_NAME"
  docker tag "$PRE_NAME" "$PUB_NAME"
  if [ $PUSH == "true" ]; then
    echo signing "$PUB_NAME"
    docker trust sign "$PUB_NAME"
    echo pushing "$PUB_NAME"
    docker push --disable-content-trust=false "$PUB_NAME"
      pushed="$?"
      if [[ "$pushed" == "0" ]]; then
        success=0
      else
        success=-1
      fi
  else
    echo Dry run, Not pushing "$PUB_NAME"
  fi
fi

exit $success
