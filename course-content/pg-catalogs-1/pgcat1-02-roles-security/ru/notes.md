# Роли, права и безопасность в системных каталогах

## Ментальная модель

Метаданные безопасности PostgreSQL распределены по нескольким каталогам. Полезно разделять четыре задачи:

```text
кто существует и что умеет роль       → pg_authid
кто в какую роль входит                → pg_auth_members
какие настройки действуют для роли/БД  → pg_db_role_setting
какие права и политики применяются     → pg_default_acl, pg_parameter_acl,
                                         pg_policy, pg_*seclabel
```

Часть этих каталогов является общей для всего кластера, потому что роли и некоторые настройки не принадлежат одной конкретной базе данных.

---

## `pg_authid`: роли как идентификаторы авторизации

`pg_authid` хранит роли PostgreSQL. Понятия «пользователь» и «группа» здесь сведены к одной сущности: **role**. Роль с `rolcanlogin = true` может использоваться как начальный идентификатор при подключении.

Каталог разделяется всеми базами кластера.

Основные поля:

- `rolname` — имя роли;
- `rolsuper` — superuser;
- `rolinherit` — автоматически наследует привилегии ролей, членом которых является;
- `rolcreaterole` — может создавать роли;
- `rolcreatedb` — может создавать базы;
- `rolcanlogin` — может подключаться;
- `rolreplication` — replication role;
- `rolbypassrls` — обходит row-level security;
- `rolconnlimit` — лимит соединений;
- `rolpassword` — сохранённый verifier пароля;
- `rolvaliduntil` — срок действия пароля.

### Почему обычно используют `pg_roles`

В `pg_authid` присутствует `rolpassword`, поэтому этот каталог не предназначен для свободного чтения всеми пользователями. Для обычного просмотра ролей существует представление `pg_roles`, в котором пароль очищен.

Это хороший общий принцип PostgreSQL:

```text
чувствительный системный каталог
        ↓
безопасное user-facing view
```

---

## `pg_auth_members`: членство ролей

`pg_auth_members` описывает отношения membership между ролями. Иерархия может быть многоуровневой, но не должна содержать циклы.

Ключевые поля:

- `roleid` — роль, **в которую** включают другую;
- `member` — роль, которая является членом `roleid`;
- `grantor` — кто выдал членство;
- `admin_option` — может ли `member` выдавать это членство другим;
- `inherit_option` — наследует ли член привилегии выданной роли;
- `set_option` — может ли член делать `SET ROLE` в выданную роль.

Направление связи удобно запоминать так:

```text
member  --is member of-->  roleid
```

`pg_auth_members`, как и `pg_authid`, разделяется всеми базами кластера.

---

## `pg_db_role_setting`: настройки по роли и базе

Этот каталог хранит значения runtime configuration parameters, заданные по умолчанию для комбинации роли и базы данных.

Поля:

- `setdatabase` — OID базы или `0`, если настройка не привязана к конкретной базе;
- `setrole` — OID роли или `0`, если настройка не привязана к конкретной роли;
- `setconfig` — массив значений параметров.

Каталог shared на весь кластер.

Удобная модель:

```text
ALTER ROLE ... SET ...
ALTER ROLE ... IN DATABASE ... SET ...
        ↓
pg_db_role_setting
```

---

## `pg_default_acl`: права будущих объектов

`pg_default_acl` хранит **начальные права, которые будут назначаться создаваемым объектам**.

Это не каталог «текущих ACL всех объектов». Его смысл именно в формировании ACL в момент создания нового объекта.

Основные поля:

- `defaclrole` — роль-владелец правила;
- `defaclnamespace` — schema или `0` для глобальной записи;
- `defaclobjtype` — вид объекта;
- `defaclacl` — ACL, назначаемый объекту.

Коды `defaclobjtype`, описанные в каталоге:

```text
r  relation
S  sequence
f  function
T  type
n  schema
```

Есть два слоя default privileges:

```text
global entry      defaclnamespace = 0
schema-local      defaclnamespace = конкретная schema
```

Schema-local права добавляются к глобальным или встроенным значениям по умолчанию.

### Важное различие с `NULL` в ACL других каталогов

Если ACL-поле конкретного объекта равно `NULL`, это означает встроенные права по умолчанию **для этого объекта**, а не «пересчитать права по текущему `pg_default_acl`». Записи `pg_default_acl` учитываются именно в момент создания объекта.

---

## `pg_init_privs`: исходные права системных и extension-объектов

`pg_init_privs` хранит начальные права объектов, если они отличаются от стандартного `NULL`-набора.

Такие права могли появиться:

```text
i  initdb
e  CREATE EXTENSION
```

Код хранится в `privtype`.

Другие поля:

- `objoid` — объект;
- `classoid` — каталог, где он описан;
- `objsubid` — номер столбца для column-level объекта или `0`;
- `initprivs` — начальный ACL.

Это позволяет PostgreSQL и инструментам различать исходные права объекта и последующие изменения прав.

---

## `pg_parameter_acl`: права на параметры конфигурации

`pg_parameter_acl` содержит только те configuration parameters, для которых были назначены нестандартные права.

- `parname` — имя GUC-параметра;
- `paracl` — ACL этого параметра.

Параметры со стандартными правами в каталоге отсутствуют.

`pg_parameter_acl` является shared catalog для всего кластера.

---

## `pg_policy`: row-level security

`pg_policy` хранит RLS policies таблиц.

Основные поля:

- `polname` — имя policy;
- `polrelid` — таблица;
- `polcmd` — команда, для которой действует policy;
- `polpermissive` — permissive или restrictive;
- `polroles` — роли;
- `polqual` — выражение, добавляемое к security barrier condition при чтении/изменении существующих строк;
- `polwithcheck` — выражение `WITH CHECK` для новых значений строк.

Коды `polcmd`:

```text
r  SELECT
a  INSERT
w  UPDATE
d  DELETE
*  all commands
```

### Запись policy ещё не означает, что RLS действует

Для применения RLS на таблице должен быть включён соответствующий флаг:

```text
pg_policy.polrelid → pg_class.oid
pg_class.relrowsecurity = true
```

Поэтому при диагностике RLS недостаточно посмотреть только `pg_policy`.

---

## Security labels: `pg_seclabel` и `pg_shseclabel`

Security label связывает объект с provider и текстовой меткой.

### `pg_seclabel`

Для объектов конкретной базы:

- `objoid` — объект;
- `classoid` — каталог объекта;
- `objsubid` — номер столбца или `0`;
- `provider` — provider security label;
- `label` — значение метки.

### `pg_shseclabel`

Для глобальных объектов кластера. Этот каталог shared между всеми базами.

Модель:

```text
local database object → pg_seclabel
shared cluster object → pg_shseclabel
```

---

## Практическое исследование ролей

Членство можно читать через self-join к `pg_authid`:

```sql
SELECT
    parent.rolname AS granted_role,
    member.rolname AS member_role,
    m.admin_option,
    m.inherit_option,
    m.set_option
FROM pg_auth_members AS m
JOIN pg_authid AS parent
  ON parent.oid = m.roleid
JOIN pg_authid AS member
  ON member.oid = m.member;
```

Для обычного пользовательского инструмента лучше предпочитать публичные представления, если они предоставляют нужную информацию, а прямое чтение чувствительных каталогов оставлять для административных задач.

---

## Что запомнить

```text
pg_authid          → сама роль и её capabilities
pg_auth_members    → membership между ролями
pg_db_role_setting → role/database-specific GUC defaults
pg_default_acl     → ACL будущих объектов
pg_init_privs      → исходные ACL initdb/extension
pg_parameter_acl   → ACL конфигурационных параметров
pg_policy          → RLS policies
pg_seclabel        → labels объектов текущей базы
pg_shseclabel      → labels shared-объектов кластера
```

При работе с безопасностью всегда сначала определяйте уровень области действия: **объект базы**, **роль**, **комбинация role+database** или **shared object кластера**. Это сразу сужает набор каталогов, которые нужно исследовать.
