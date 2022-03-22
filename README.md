# Tesql

Single source truth database inserts and json data for integration and end-to-end testing.

## Demo

#### Create your data file

some-data.json

```json
{
  "data": {
    "user": {
      "columns": [
        { "name": "id", "type": "uuid" },
        { "name": "username", "type": "text" },
        { "name": "is_student", "type": "text" },
        { "name": "simple_arrays", "type": "array" },
        { "name": "nested_arrays", "type": "array" }
      ],
      "inserts": [
        {
          "id": "1",
          "username": "Bob",
          "is_student": true,
          "simple_arrays": ["hello", "world", "hi!"],
          "nested_arrays": [
            ["hey", "man"],
            ["the", "world", "goes", "round"]
          ]
        },
        {
          "id": "2",
          "username": "Bob2",
          "is_student": false,
          "simple_arrays": ["hi", "mom"],
          "nested_arrays": [["hello", "there"], ["hello", "world"], ["hi!"]]
        }
      ]
    },
    "class": {
      "columns": [
        { "name": "id", "type": "uuid" },
        { "name": "subject", "type": "text" },
        { "name": "teacher", "type": "text" }
      ],
      "inserts": [
        {
          "id": "1",
          "subject": "Math",
          "teacher": "James"
        }
      ]
    }
  }
}
```

#### Generate SQL and JSON

Keep entities in one file:

```shell
tesql --in /path/to/some-data.json --json-out /path/to/file.json --sql-out /path/to/file.sql
```

Split each entity into a separate file:

```shell
tesql --split --in /path/to/some-data.json --json-dir /path/to/json_dir --sql-dir /path/to/sql_dir
```

#### Demo Output

/path/to/file.json

```json
{
  "data": {
    "user": [
      {
        "id": "1",
        "username": "Bob",
        "is_student": true,
        "simple_arrays": ["hello", "world", "hi!"],
        "nested_arrays": [
          ["hey", "man"],
          ["the", "world", "goes", "round"]
        ]
      },
      {
        "id": "2",
        "username": "Bob2",
        "is_student": false,
        "simple_arrays": ["hi", "mom"],
        "nested_arrays": [["hello", "there"], ["hello", "world"], ["hi!"]]
      }
    ],
    "class": [{ "id": "1", "subject": "Math", "teacher": "James" }]
  }
}
```

/path/to/file.sql

```sql
INSERT INTO "user" (id, username, is_student, simple_arrays, nested_arrays)
VALUES ('1', 'Bob', true, '{"hello", "world", "hi!"}', '{{"hey", "man"}, {"the", "world", "goes", "round"}}'), ('2', 'Bob2', false, '{"hi", "mom"}', '{{"hello", "there"}, {"hello", "world"}, {"hi!"}}');

INSERT INTO "class" (id, subject, teacher)
VALUES ('1', 'Math', 'James');

```
