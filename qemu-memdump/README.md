```
export PLATFORM=leon3
docker build . -t ${PLATFORM} -f ${PLATFORM}.docker
./docker.sh ${PLATFORM} ./build.sh
./docker.sh ${PLATFORM} ./run_and_dump.sh
```