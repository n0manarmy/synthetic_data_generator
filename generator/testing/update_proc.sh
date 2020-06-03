#!/bin/bash
sudo sh -c "echo -1 > /proc/sys/kernel/perf_event_paranoid"
sudo sh -c "echo 0 > /proc/sys/kernel/kptr_restrict"

echo "run perf like this:"
echo "perf record -F99 --call-graph dwarf {program}"
echo "perf report"
