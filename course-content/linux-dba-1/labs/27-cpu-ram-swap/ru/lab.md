# Лабораторная №27 — CPU / RAM / swap

**Бюджет: 2 часа**

Цель:

> Научиться отличать CPU bottleneck, нехватку RAM, swap pressure и I/O wait, а также видеть, как системная нагрузка отражается на PostgreSQL.

Инструменты:

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

В `lscpu` смотри:

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

Три значения load average относятся примерно к 1, 5 и 15 минутам.

Load average — не процент CPU. Он отражает количество задач, которые выполняются, готовы выполняться и в некоторых случаях ждут uninterruptible I/O.

Интерпретация зависит от количества CPU. Например, load около 2 на системе с двумя CPU означает совсем не то же самое, что load 2 на системе с 16 CPU.

## 3. `top`

```bash
top
```

Основные поля `%Cpu(s)`:

```text
us = user CPU
sy = system CPU
id = idle
wa = I/O wait
```

Пример CPU-bound:

```text
us 90
sy 5
id 5
wa 0
```

Большой `wa` при заметном `id` больше указывает в сторону I/O wait.

## 4. Память

```bash
free -h
```

Смотри:

```text
total
used
free
shared
buff/cache
available
```

Маленькое `free` само по себе не означает проблему: Linux использует память под cache. Для оценки доступной памяти особенно полезно поле `available`.

## 5. Swap

```bash
swapon --show
free -h
```

Наличие занятого swap и активное swapping — не одно и то же.

## 6. `vmstat`

```bash
vmstat 1
```

`1` — обновлять статистику раз в одну секунду.

Основные поля:

```text
r  → runnable processes
b  → blocked processes, ожидающие I/O
si → swap in
so → swap out
bi → blocks in
bo → blocks out
us → user CPU
sy → system CPU
id → idle CPU
wa → I/O wait
```

Особенно полезно сравнивать `r` с количеством CPU.

## 7. Baseline

До нагрузки зафиксируй:

```bash
vmstat 1
top
free -h
uptime
```

Запиши:

```text
CPU idle
load average
available RAM
swap used
si/so
wa
```

И baseline PostgreSQL через `pgbench`.

## 8. CPU stress

Проверь `stress-ng`:

```bash
stress-ng --version
```

Если его нет:

```bash
sudo apt update
sudo apt install stress-ng
```

Создай CPU-нагрузку:

```bash
stress-ng --cpu 2 --timeout 120s
```

- `--cpu 2` — два CPU worker;
- `--timeout 120s` — остановиться через 120 секунд.

Во время нагрузки наблюдай `top` и `vmstat 1`.

Ожидаемые признаки CPU pressure:

```text
r ↑
us/sy ↑
id ↓
load average ↑
```

## 9. PostgreSQL под CPU pressure

```bash
pgbench -c 10 -j 2 -T 60 lab
```

- `-c 10` — 10 клиентов;
- `-j 2` — 2 worker threads pgbench;
- `-T 60` — 60 секунд.

Сравни с baseline:

```text
latency average
TPS
```

При CPU bottleneck обычно latency растёт, TPS падает.

## 10. Memory stress

После CPU-эксперимента:

```bash
free -h
stress-ng --vm 1 --vm-bytes 70% --timeout 120s
```

- `--vm 1` — один memory worker;
- `--vm-bytes 70%` — примерно 70% памяти для нагрузки.

Смотри:

```text
available ↓
swap used ↑
si/so
```

Если swap отсутствует, не обязательно специально добавлять его. Не доводи лабораторную систему до OOM.

## 11. Swap used vs active swapping

```text
Swap used > 0
```

не обязательно означает проблему.

Гораздо важнее постоянные ненулевые:

```text
si
so
```

которые показывают активное перемещение страниц между RAM и swap.

## 12. PostgreSQL под memory pressure

Во время memory stress:

```bash
pgbench -c 10 -j 2 -T 60 lab
```

Сравни:

```text
TPS
latency
available RAM
si/so
CPU
```

## 13. Сценарии

### CPU bottleneck

```text
CPU = 2
load average = 8
r = 7
us = 92
id = 3
wa = 0
```

Это похоже на CPU bottleneck: есть очередь за CPU, idle почти нет, I/O wait низкий.

### Memory pressure + swapping

```text
available = 100 MB
swap used = 2 GB
si = 500
so = 800
```

Это похоже на memory pressure с активным swapping.

### Не CPU, а I/O wait

```text
id = 70
wa = 25
r = 1
```

Это не похоже на насыщенный CPU; высокий `wa` ведёт диагностику в сторону I/O.

## 14. Быстро найти потребителей CPU/RAM

По CPU:

```bash
ps aux --sort=-%cpu | head
```

По памяти:

```bash
ps aux --sort=-%mem | head
```

`-` перед полем сортировки означает descending.

## 15. Таблица экспериментов

Заполни хотя бы приблизительно:

| Сценарий | load | r | us | sy | id | wa | available RAM | si/so | pgbench TPS | latency |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| baseline | | | | | | | | | | |
| CPU stress | | | | | | | | | | |
| RAM stress | | | | | | | | | | |

## Что нужно запомнить

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

## Мини-экзамен №27

1. Как узнать количество CPU?
2. Что показывает load average?
3. Почему load average 4 может быть нормальным для одной системы и плохим для другой?
4. Что означают `us`, `sy`, `id`, `wa` в `top`?
5. Как посмотреть память в удобном формате?
6. Почему маленькое поле `free` не обязательно означает проблему?
7. Как проверить, есть ли swap?
8. Что показывают `si` и `so` в `vmstat`?
9. Чем отличается `swap used` от активного swapping?
10. Что означает `r` в `vmstat`?
11. Если CPU=2, r=8, id=0, us=95 — какой bottleneck вероятен?
12. Если id=70, wa=25 — похоже ли это на CPU bottleneck?
13. Как вывести процессы, отсортированные по CPU usage?
14. Как вывести процессы, отсортированные по memory usage?
15. Какие две метрики `pgbench` сравниваем до и во время системной нагрузки?
