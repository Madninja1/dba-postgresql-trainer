# CPU, RAM и swap

## 1. Тема

Когда PostgreSQL работает медленно, недостаточно увидеть «высокую нагрузку». Нужно различить как минимум три класса проблем:

```text
CPU bottleneck
memory pressure / swapping
I/O wait
```

Диагностика строится не по одной цифре, а по сочетанию метрик:

```text
CPU     → nproc, lscpu, uptime, top, vmstat
RAM     → free, vmstat
swap    → swapon, free, vmstat
process → ps
DB load → pgbench
```

Главный навык — сначала снять baseline, затем сравнить поведение системы под нагрузкой.

---

## 2. Механизм

### Сколько CPU доступно

Быстрая команда:

```bash
nproc
```

Она показывает количество доступных CPU processors/threads для текущей среды.

Подробная topology:

```bash
lscpu
```

Полезные поля:

```text
CPU(s)
Core(s) per socket
Thread(s) per core
Model name
```

Количество CPU важно для интерпретации очереди runnable-задач и load average.

---

### Load average

```bash
uptime
```

Три значения load average относятся примерно к:

```text
1 минуте
5 минутам
15 минутам
```

Load average — **не процент CPU**. Он отражает количество задач, претендующих на выполнение, а в Linux также может включать задачи в uninterruptible sleep, часто связанном с I/O.

Поэтому одно и то же значение нужно сравнивать с числом CPU:

```text
load 2 при 2 CPU  → одна ситуация
load 2 при 16 CPU → совсем другая
```

---

### `top`: классификация CPU

```bash
top
```

В строке `%Cpu(s)` особенно полезны:

```text
us → user CPU
sy → system/kernel CPU
id → idle
wa → I/O wait
```

CPU-bound сценарий:

```text
us = 90
sy = 5
id = 5
wa = 0
```

Большой `wa` при заметном `id` ведёт скорее к I/O-проблеме, чем к чистому насыщению CPU.

---

### Память через `free`

```bash
free -h
```

Основные поля:

```text
total
used
free
shared
buff/cache
available
```

Критически важно: маленькое значение `free` само по себе **не означает нехватку RAM**. Linux активно использует свободную память как page cache.

Для практической оценки обычно информативнее:

```text
available
```

Оно оценивает, сколько памяти система ещё может предоставить приложениям без активного давления на swap.

---

### Swap

Активные swap areas:

```bash
swapon --show
```

Общий объём и использование видны также в:

```bash
free -h
```

Нужно различать:

```text
swap used > 0
```

и:

```text
идёт активный swap in / swap out прямо сейчас
```

Ненулевой `Swap used` может быть следом прошлой нагрузки и сам по себе ещё не доказывает текущий memory pressure.

---

### `vmstat 1`

```bash
vmstat 1
```

Аргумент `1` задаёт интервал обновления в одну секунду.

Полезные поля:

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

#### Поле `r`

`r` особенно полезно сравнивать с числом CPU.

Например:

```text
CPU = 2
r   = 8
id  = 0
us  = 95
```

Это сильный признак CPU bottleneck: runnable-задач значительно больше, чем CPU, idle нет, user CPU высокий.

#### Поле `b`

Рост `b` означает процессы, заблокированные в ожидании, обычно I/O. В сочетании с высоким `wa` это уводит диагностику от CPU saturation к дисковой подсистеме.

#### `si` и `so`

Постоянные ненулевые `si`/`so` показывают реальное перемещение страниц между RAM и swap.

```text
Swap used > 0, si/so ≈ 0
→ активной подкачки может не быть

si/so постоянно > 0
→ идёт active swapping
```

---

### Baseline

Перед экспериментом нужно зафиксировать состояние:

```bash
vmstat 1
top
free -h
uptime
```

И записать хотя бы:

```text
CPU idle
load average
available RAM
swap used
si/so
wa
```

Для PostgreSQL baseline можно получить через `pgbench`.

---

### CPU stress

Создать контролируемую нагрузку:

```bash
stress-ng --cpu 2 --timeout 120s
```

Параметры:

```text
--cpu 2       → два CPU worker
--timeout 120s→ завершить нагрузку через 120 секунд
```

Ожидаемые признаки CPU pressure:

```text
r ↑
us/sy ↑
id ↓
load average ↑
```

---

### PostgreSQL под CPU pressure

Пример:

```bash
pgbench -c 10 -j 2 -T 60 lab
```

Здесь:

```text
-c 10 → 10 клиентов
-j 2  → 2 worker threads pgbench
-T 60 → 60 секунд
lab   → база данных
```

Сравниваются прежде всего:

```text
TPS
latency average
```

При CPU bottleneck обычно latency растёт, а throughput/TPS ухудшается.

---

### Memory stress

```bash
stress-ng --vm 1 --vm-bytes 70% --timeout 120s
```

Это создаёт один memory worker и просит его работать примерно с указанным объёмом памяти.

Наблюдать:

```text
available ↓
swap used
si/so
```

Не нужно специально доводить учебную систему до OOM.

---

### Найти потребителей ресурсов

По CPU:

```bash
ps aux --sort=-%cpu | head
```

По памяти:

```bash
ps aux --sort=-%mem | head
```

Минус перед ключом сортировки означает descending: самые большие потребители оказываются сверху.

---

## 3. Важные детали

### CPU bottleneck

Характерная комбинация:

```text
r >> CPU count
us/sy высокие
id почти 0
wa низкий
load высокий
```

Пример:

```text
CPU = 2
load = 8
r = 7
us = 92
id = 3
wa = 0
```

### Memory pressure

Признаки:

```text
available сильно падает
swap used растёт
si/so становятся устойчиво ненулевыми
```

Особенно важна активность `si/so`, а не сам факт существования занятого swap.

### I/O wait

Пример:

```text
id = 70
wa = 25
r = 1
```

CPU явно не исчерпан: idle высокий, runnable queue маленькая. Значительный `wa` указывает на ожидание I/O.

### Нельзя диагностировать по одной метрике

Высокий load average не равен автоматически CPU saturation. Маленький `free` не равен автоматически нехватке RAM. Ненулевой swap не равен автоматически активному swapping.

Всегда смотрится комбинация показателей.

---

## Что запомнить

```bash
nproc
lscpu
```

→ сколько CPU и какая topology.

```bash
uptime
```

→ load average за 1/5/15 минут.

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

→ для реальной доступности памяти особенно смотри `available`.

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

Модели:

```text
CPU bottleneck → r↑, us/sy↑, id↓
RAM pressure   → available↓, si/so↑
I/O wait       → wa↑, часто b↑
```

Главный принцип: **сначала baseline, потом контролируемая нагрузка, затем сравнение системных метрик и PostgreSQL TPS/latency**.
