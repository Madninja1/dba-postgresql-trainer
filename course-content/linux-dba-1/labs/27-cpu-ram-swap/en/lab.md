# Lab 27 — CPU / RAM / swap

**Budget: 2 hours**

Goal:

> Learn to distinguish a CPU bottleneck, RAM shortage, swap pressure, and I/O wait, and see how system load affects PostgreSQL.

Tools:

```text
top
free
vmstat
uptime
nproc
lscpu
swapon
stress-ng
pgbench
ps
```

## 1. CPU

```bash
nproc
lscpu
```

In `lscpu`, look at:

```text
CPU(s)
Core(s) per socket
Thread(s) per core
Model name
```

## 2. Load average

```bash
uptime
```

The three load-average values correspond roughly to 1, 5, and 15 minutes.

Load average is not CPU percentage. It reflects the number of tasks that are running, ready to run, and in some cases waiting in uninterruptible I/O.

Interpretation depends on the number of CPUs. For example, a load around 2 means something very different on a two-CPU system than on a 16-CPU system.

## 3. `top`

```bash
top
```

Main `%Cpu(s)` fields:

```text
us = user CPU
sy = system CPU
id = idle
wa = I/O wait
```

Example of a CPU-bound system:

```text
us 90
sy 5
id 5
wa 0
```

A high `wa` together with noticeable `id` points more toward I/O wait.

## 4. Memory

```bash
free -h
```

Look at:

```text
total
used
free
shared
buff/cache
available
```

A small `free` value alone does not mean there is a problem: Linux uses memory for cache. The `available` field is especially useful when estimating memory that can still be used.

## 5. Swap

```bash
swapon --show
free -h
```

Having swap in use is not the same as active swapping.

## 6. `vmstat`

```bash
vmstat 1
```

`1` means refresh the statistics every second.

Main fields:

```text
r  → runnable processes
b  → blocked processes waiting for I/O
si → swap in
so → swap out
bi → blocks in
bo → blocks out
us → user CPU
sy → system CPU
id → idle CPU
wa → I/O wait
```

It is especially useful to compare `r` with the number of CPUs.

## 7. Baseline

Before applying load, capture:

```bash
vmstat 1
top
free -h
uptime
```

Record:

```text
CPU idle
load average
available RAM
swap used
si/so
wa
```

Also capture a PostgreSQL baseline with `pgbench`.

## 8. CPU stress

Check `stress-ng`:

```bash
stress-ng --version
```

If it is not installed:

```bash
sudo apt update
sudo apt install stress-ng
```

Create CPU load:

```bash
stress-ng --cpu 2 --timeout 120s
```

- `--cpu 2` — two CPU workers;
- `--timeout 120s` — stop after 120 seconds.

While the load is running, watch `top` and `vmstat 1`.

Expected signs of CPU pressure:

```text
r ↑
us/sy ↑
id ↓
load average ↑
```

## 9. PostgreSQL under CPU pressure

```bash
pgbench -c 10 -j 2 -T 60 lab
```

- `-c 10` — 10 clients;
- `-j 2` — 2 pgbench worker threads;
- `-T 60` — 60 seconds.

Compare with the baseline:

```text
latency average
TPS
```

With a CPU bottleneck, latency usually increases and TPS drops.

## 10. Memory stress

After the CPU experiment:

```bash
free -h
stress-ng --vm 1 --vm-bytes 70% --timeout 120s
```

- `--vm 1` — one memory worker;
- `--vm-bytes 70%` — use roughly 70% of memory for the load.

Watch:

```text
available ↓
swap used ↑
si/so
```

If swap is absent, you do not have to add it just for the lab. Do not drive the lab system into OOM.

## 11. Swap used vs active swapping

```text
Swap used > 0
```

does not necessarily mean there is a problem.

Much more important are continuously nonzero:

```text
si
so
```

which indicate active movement of pages between RAM and swap.

## 12. PostgreSQL under memory pressure

While memory stress is running:

```bash
pgbench -c 10 -j 2 -T 60 lab
```

Compare:

```text
TPS
latency
available RAM
si/so
CPU
```

## 13. Scenarios

### CPU bottleneck

```text
CPU = 2
load average = 8
r = 7
us = 92
id = 3
wa = 0
```

This looks like a CPU bottleneck: there is a CPU run queue, idle is almost zero, and I/O wait is low.

### Memory pressure + swapping

```text
available = 100 MB
swap used = 2 GB
si = 500
so = 800
```

This looks like memory pressure with active swapping.

### Not CPU, but I/O wait

```text
id = 70
wa = 25
r = 1
```

This does not look like a saturated CPU; the high `wa` points diagnostics toward I/O.

## 14. Quickly find CPU/RAM consumers

By CPU:

```bash
ps aux --sort=-%cpu | head
```

By memory:

```bash
ps aux --sort=-%mem | head
```

The `-` before the sort field means descending order.

## 15. Experiment table

Fill in at least approximate values:

| Scenario | load | r | us | sy | id | wa | available RAM | si/so | pgbench TPS | latency |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| baseline | | | | | | | | | | |
| CPU stress | | | | | | | | | | |
| RAM stress | | | | | | | | | | |

## What to remember

### CPU

```text
r ↑
us/sy ↑
id ↓
load ↑
```

### RAM pressure

```text
available ↓
swap ↑
si/so ↑
```

### I/O wait

```text
wa ↑
b ↑
```

## Mini-exam 27

1. How do you find the number of CPUs?
2. What does load average show?
3. Why can load average 4 be normal on one system and bad on another?
4. What do `us`, `sy`, `id`, and `wa` mean in `top`?
5. How do you display memory in a human-readable format?
6. Why does a small `free` value not necessarily indicate a problem?
7. How do you check whether swap exists?
8. What do `si` and `so` show in `vmstat`?
9. What is the difference between `swap used` and active swapping?
10. What does `r` mean in `vmstat`?
11. If CPU=2, r=8, id=0, us=95, which bottleneck is likely?
12. If id=70 and wa=25, does that look like a CPU bottleneck?
13. How do you list processes sorted by CPU usage?
14. How do you list processes sorted by memory usage?
15. Which two `pgbench` metrics do we compare before and during system load?
