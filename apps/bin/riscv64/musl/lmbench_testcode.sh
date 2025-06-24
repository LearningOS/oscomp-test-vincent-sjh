#!./busybox sh

./busybox echo "#### OS COMP TEST GROUP START lmbench-musl ####"

echo latency measurements
./lmbench_all lat_syscall -P 1 null


./busybox echo "#### OS COMP TEST GROUP END lmbench-musl ####"