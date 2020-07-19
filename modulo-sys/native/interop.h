typedef enum FieldType {
  ROW,
  LABEL,
  TEXT,
  MULTILINE_TEXT,
  CHOICE,
  CHECKBOX,
} FieldType;

typedef struct LabelMetadata {
  const char *text;
} LabelMetadata;

typedef struct TextMetadata {
  const char *defaultText;
} TextMetadata;

typedef struct FieldMetadata {
  const char * id;
  FieldType fieldType;
  const void * specific;
} FieldMetadata;

typedef struct RowMetadata {
  const FieldMetadata *fields;
  const int fieldSize;
} RowMetadata;

typedef struct FormMetadata {
  const char *windowTitle;
  const FieldMetadata *fields;
  const int fieldSize;
} FormMetadata;