# Функции, агрегаты, операторы и языки

## 1. Тема

Тема посвящена системным каталогам PostgreSQL и связям между ними: где сервер хранит метаданные объектов и какие поля позволяют восстановить структуру и состояние базы.

```text
основной материал темы
        ↓
понимание механизма
        ↓
важные ограничения и связи
        ↓
факты, которые проверяются тестами
```

## 2. Механизм

Эта группа каталогов описывает исполняемые объекты PostgreSQL, приведения типов и процедурные языки.

### pg_proc

`pg_proc` содержит функции, процедуры, агрегаты и оконные функции.

`prokind`:

```text
f  function
p  procedure
a  aggregate
w  window function
```

`provolatile`:

```text
i  IMMUTABLE
s  STABLE
v  VOLATILE
```

`proparallel`:

```text
s  safe
r  restricted: только leader процесса parallel group
u  unsafe
```

### pg_aggregate

Запись агрегатной функции в `pg_aggregate` **дополняет** соответствующую запись в `pg_proc`.

`aggkind`:

```text
n  normal
o  ordered-set
h  hypothetical-set
```

Каталог также хранит transition/final/combine функции, тип внутреннего состояния и другие свойства агрегата.

### pg_operator

Содержит определения SQL-операторов: имя, типы операндов/результата и функцию, реализующую оператор.

### pg_cast

Хранит явно описанные пути приведения типов.

Важные поля:

- `castsource` — исходный тип;
- `casttarget` — целевой;
- `castfunc` — функция приведения, если нужна;
- `castcontext`: `e` explicit, `a` assignment, `i` implicit;
- `castmethod`: например `f` — через функцию, `b` — binary-coercible, `i` — через I/O.

### pg_language

Регистрирует языки функций/процедур.

`lanpltrusted=true` означает доверенный язык: создавать функции на нём могут не только суперпользователи, если имеют необходимые права. Недоверенные языки доступны для создания функций суперпользователю.

### pg_transform

Описывает трансформации типов при обмене значениями с процедурными языками.

### pg_conversion

Описывает преобразования кодировок:

- `conforencoding` — исходная кодировка;
- `contoencoding` — целевая;
- `conproc` — функция;
- `condefault` — преобразование по умолчанию.

### pg_range

Метаданные range/multirange типов. Например:

- `rngsubtype` — тип элемента диапазона;
- `rngcanonical` — функция канонизации, если она есть.

---

## 3. Важные детали и покрытие тестов

Ниже собраны детали, которые проверяются вопросами этой темы. Они сгруппированы по разделам исходной документации, чтобы их можно было связывать с механизмом, а не запоминать как отдельный список ответов.

### 53.2. pg_aggregate

- **Связь записей pg_aggregate с pg_proc.** Запись pg_aggregate дополняет соответствующую запись pg_proc. Для агрегата pg_proc хранит общие свойства подпрограммы, а pg_aggregate — специфические свойства агрегирования.
- **Столбец aggkind.** `n` — normal. aggkind различает обычный, ordered-set и hypothetical-set aggregate.

### 53.39. pg_proc

- **Назначение каталога pg_proc.** Функции, процедуры, агрегатные и оконные функции. pg_proc описывает подпрограммы: functions, procedures, aggregate и window functions.
- **Столбец prokind.** `f` — функция. prokind кодирует четыре вида подпрограмм, перечисленные в документации.
- **Столбец provolatile.** provolatile: i — immutable, s — stable, v — volatile.
- **Столбец proparallel.** `s` — безопасно выполнять в parallel mode. proparallel различает safe, restricted и unsafe execution in parallel mode.

### 53.34. pg_operator

- **Назначение pg_operator.** pg_operator содержит имя, типы операндов/результата и функцию реализации оператора.

### 53.10. pg_cast

- **castsource, casttarget, castcontext, castmethod.** castsource — исходный тип. pg_cast описывает источник/назначение, допустимый контекст приведения и механизм выполнения.

### 53.29. pg_language

- **Назначение pg_language.** pg_language содержит зарегистрированные procedural languages и связанные handler/validator functions.
- **Столбец lanpltrusted.** true означает доверенный язык. lanpltrusted показывает доверенность языка; материал отмечает ограничение на создание функций в недоверенных языках.

### 53.57. pg_transform

- **Назначение pg_transform.** Для адаптации типов данных при обмене с процедурными языками. pg_transform описывает преобразования типов между SQL и процедурными языками.

### 53.43. pg_range

- **Назначение pg_range; rngsubtype; rngcanonical.** pg_conversion описывает функции преобразования кодировок. pg_conversion относится к перекодировкам, а pg_range дополняет pg_type информацией о range types.

### 53.14. pg_conversion

- **conforencoding, contoencoding, conproc, condefault.** conforencoding — ID исходной кодировки. pg_conversion хранит имя и функцию преобразования между исходной и целевой кодировками, а также признак default conversion.

---

# Что запомнить

- **Связь записей pg_aggregate с pg_proc** → Запись pg_aggregate дополняет соответствующую запись pg_proc
- **Столбец aggkind** → `n` — normal
- **Назначение каталога pg_proc** → Функции, процедуры, агрегатные и оконные функции
- **Столбец prokind** → `f` — функция
- **Столбец provolatile** → i
- **Столбец proparallel** → `s` — безопасно выполнять в parallel mode
- **Назначение pg_operator** → pg_operator
- **castsource, casttarget, castcontext, castmethod** → castsource — исходный тип
- **Назначение pg_language** → pg_language
- **Столбец lanpltrusted** → true означает доверенный язык
- **Назначение pg_transform** → Для адаптации типов данных при обмене с процедурными языками
- **Назначение pg_range; rngsubtype; rngcanonical** → pg_conversion описывает функции преобразования кодировок
- **conforencoding, contoencoding, conproc, condefault** → conforencoding — ID исходной кодировки
