if [ -z ${1+x} ]; then
    echo "No argument provided";
    exit 1;
fi

version=$1

docker build -t ghcr.io/linusfri/uno_api:$version -f ./build/Dockerfile-app-prod .
docker push ghcr.io/linusfri/uno_api:$version