#!/bin/sh
set -xe

PERF=$(command -v "/usr/lib/linux-tools/5.15.0-122-generic/perf") \
  || PERF=$(command -v perf)

cargo build --release
time $PERF record --call-graph dwarf ./target/release/main < testing/in/0000.txt

# スクリプト生成は時間がかかるのでコメントアウト。Firefox Profiler とかで見た
# い場合はスクリプトが必要だけど、hotspot で見る場合は不要だし。
#/usr/lib/linux-tools/5.15.0-122-generic/perf script > perf.script
