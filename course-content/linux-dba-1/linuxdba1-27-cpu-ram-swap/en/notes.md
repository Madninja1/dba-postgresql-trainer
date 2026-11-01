# CPU, RAM, and Swap

## 1. Topic

When PostgreSQL is slow, seeing “high load” is not enough. You need to distinguish at least three classes of problems:

```text
CPU bottleneck
memory pressure / swapping
I/O wait
```

Diagnostics should not rely on one number, but on a combination of metrics:

```text
CPU     → nproc, lscpu, uptime, top, vmstat
RAM     → free, vmstat
swap    → swapon, free, vmstat
process → ps
DB load → pgbench
```

The key skill is to capture a baseline first and then compare system behavior under load.

---

## 2. Mechanism

### How many CPUs are available

Quick command:

```bash
nproc
```

It shows the number of CPU processors/threads available to the current environment.

Detailed topology:

```bash
lscpu
```

Useful fields:

```text
CPU(s)
Core(s) per socket
Thread(s) per core
Model name
```

The CPU count matters when interpreting the runnable queue and load average.

---

### Load average

```bash
uptime
```

The three load-average values correspond approximately to:

```text
1 minute
5 minutes
15 minutes
```

Load average is **not a CPU percentage**. It reflects the number of tasks competing to run and, on Linux, can also include tasks in uninterruptible sleep, often related to I/O.

Therefore, the same load value must be compared with the number of CPUs:

```text
load 2 with 2 CPUs  → one situation
load 2 with 16 CPUs → a very different situation
```

---

### `top`: classifying CPU usage

```bash
top
```

In the `%Cpu(s)` line, these fields are especially useful:

```text
us → user CPU
sy → system/kernel CPU
id → idle
wa → I/O wait
```

A CPU-bound scenario:

```text
us = 90
sy = 5
id = 5
wa = 0
```

High `wa` together with noticeable `id` points more toward an I/O problem than pure CPU saturation.

---

### Memory with `free`

```bash
free -h
```

Main fields:

```text
total
used
free
shared
buff/cache
available
```

A critical point: a small `free` value by itself **does not mean RAM is exhausted**. Linux actively uses otherwise free memory for page cache.

For practical assessment, this field is usually more informative:

```text
available
```

It estimates how much memory the system can still provide to applications without active swap pressure.

---

### Swap

Active swap areas:

```bash
swapon --show
```

Total size and usage are also visible in:

```bash
free -h
```

You must distinguish:

```text
swap used > 0
```

from:

```text
active swap in / swap out is happening right now
```

A nonzero `Swap used` value may be a remnant of earlier load and by itself does not prove current memory pressure.

---

### `vmstat 1`

```bash
vmstat 1
```

The argument `1` sets an update interval of one second.

Useful fields:

```text
r  → runnable processes
b  → blocked processes
si → swap in
so → swap out
bi → blocks in
bo → blocks out
us → user CPU
sy → system CPU
id → idle CPU
wa → I/O wait
```

#### The `r` field

`r` is especially useful when compared with the CPU count.

For example:

```text
CPU = 2
r   = 8
id  = 0
us  = 95
```

This is a strong sign of a CPU bottleneck: there are far more runnable tasks than CPUs, there is no idle time, and user CPU is high.

#### The `b` field

An increase in `b` means processes are blocked waiting, usually for I/O. Together with high `wa`, this shifts diagnostics away from CPU saturation and toward the storage subsystem.

#### `si` and `so`

Sustained nonzero `si`/`so` values show actual page movement between RAM and swap.

```text
Swap used > 0, si/so ≈ 0
→ active swapping may not be happening

si/so continuously > 0
→ active swapping is happening
```

---

### Baseline

Before an experiment, record the current state:

```bash
vmstat 1
top
free -h
uptime
```

Record at least:

```text
CPU idle
load average
available RAM
swap used
si/so
wa
```

For PostgreSQL, a baseline can be obtained with `pgbench`.

---

### CPU stress

Create controlled load:

```bash
stress-ng --cpu 2 --timeout 120s
```

Parameters:

```text
--cpu 2        → two CPU workers
--timeout 120s → stop the load after 120 seconds
```

Expected signs of CPU pressure:

```text
r ↑
us/sy ↑
id ↓
load average ↑
```

---

### PostgreSQL under CPU pressure

Example:

```bash
pgbench -c 10 -j 2 -T 60 lab
```

Here:

```text
-c 10 → 10 clients
-j 2  → 2 pgbench worker threads
-T 60 → run for 60 seconds
lab   → database
```

The main values to compare are:

```text
TPS
latency average
```

With a CPU bottleneck, latency usually rises while throughput/TPS degrades.

---

### Memory stress

```bash
stress-ng --vm 1 --vm-bytes 70% --timeout 120s
```

This creates one memory worker and asks it to work with roughly the specified amount of memory.

Observe:

```text
available ↓
swap used
si/so
```

There is no need to deliberately drive a training system into OOM.

---

### Finding resource consumers

By CPU:

```bash
ps aux --sort=-%cpu | head
```

By memory:

```bash
ps aux --sort=-%mem | head
```

The minus sign before the sort key means descending order: the largest consumers appear first.

---

## 3. Important details

### CPU bottleneck

Typical combination:

```text
r >> CPU count
us/sy high
id near 0
wa low
load high
```

Example:

```text
CPU = 2
load = 8
r = 7
us = 92
id = 3
wa = 0
```

### Memory pressure

Signs:

```text
available drops significantly
swap used grows
si/so become persistently nonzero
```

The activity in `si/so` is especially important, not merely the fact that some swap is occupied.

### I/O wait

Example:

```text
id = 70
wa = 25
r = 1
```

The CPU is clearly not exhausted: idle is high and the runnable queue is small. Significant `wa` points toward I/O waiting.

### Do not diagnose from a single metric

High load average does not automatically mean CPU saturation. Low `free` does not automatically mean a RAM shortage. Nonzero swap does not automatically mean active swapping.

Always evaluate a combination of metrics.

---

## What to remember

```bash
nproc
lscpu
```

→ how many CPUs are available and what the topology is.

```bash
uptime
```

→ load average over 1/5/15 minutes.

```text
top:
us user
sy system
id idle
wa I/O wait
```

```bash
free -h
```

→ for practical memory availability, pay particular attention to `available`.

```bash
vmstat 1
```

```text
r  runnable queue
b  blocked
si swap in
so swap out
wa I/O wait
```

Models:

```text
CPU bottleneck → r↑, us/sy↑, id↓
RAM pressure   → available↓, si/so↑
I/O wait       → wa↑, often b↑
```

Main principle: **capture a baseline first, then apply controlled load, then compare system metrics with PostgreSQL TPS/latency**.
