
// SCHEMA ----------------------------------------------------------------------

/// Schema
static TYPE_SCHEMA = SetType {
    "id": "schema",
    "label": "Schema",
    "description": "Schema",
    "group": None,
    "parent_group": None,
    "types": [
        TYPE_SCHEMA_TYPE_INDEX,
        TYPE_SCHEMA_TYPE_GROUP_INDEX,
    ],
}

// SCHEMA > TYPE INDEX ---------------------------------------------------------

/// Type Index
static TYPE_SCHEMA_TYPE_INDEX = ListType {
    "id": "type_index",
    "label": "Type Index",
    "description": "Type index",
    "group": None,
    "parent_group": None,
    "types": [
        TYPE_TYPE,
    ],
}

// SCHEMA > TYPE INDEX ---------------------------------------------------------

/// Type Group Index
static TYPE_SCHEMA_TYPE_GROUP_INDEX = ListType {
    "id": "type_group_index",
    "label": "Type Group Index",
    "description": "Type Group Index",
    "group": None,
    "parent_group": None,
    "types": [
        TYPE_GROUP,
    ],


}

// TYPE-------------------------------------------------------------------------

/// Type
static TYPE_TYPE = SetType {
    "id": "type",
    "label": "Type",
    "description": "Type",
    "group": Some(TYPE_GROUP_GROUP),
    "parent_group": None,
    "types": [
        TYPE_PRODUCT_TYPE,
        TYPE_SUM_TYPE,
        TYPE_FLOAT_TYPE,
        TYPE_INTEGER_TYPE,
        TYPE_TEXT_TYPE,
    ],
}

/// Type Group Type Group
static TYPE_GROUP_GROUP = TypeGroup {
    "id": "type",
    "label": "Type",
}

// TYPE > Group ----------------------------------------------------------------

/// Type Group
static TYPE_GROUP = SetType {
    "id": "type_group",
    "label": "Type Group",
    "description": "Type Group",
    "group": TYPE_GROUP_GROUP,
    "parent_group": TYPE_GROUP,
    "types": [
        TYPE_GROUP_ID,
        TYPE_GROUP_LABeL,
    ],
}

/// Type Group Type Group
static TYPE_GROUP_GROUP = TypeGroup {
    "id": "type_group",
    "label": "Type Group",
}

/// Type Group Id
static TYPE_GROUP_ID = TextType {
    "id": "id",
    "label": "Id",
    "description": "Type Group Id",
    "group": Some(TYPE_GROUP_GROUP),
    "parent_group": None,
}

/// Type Group Label
static TYPE_GROUP_LABEL = TextType {
    "id": "label",
    "label": "Label",
    "description": "Type Group Id",
    "parent_group": Some(TYPE_GROUP_GROUP),
    "group": None,
}


// TYPE > Set ------------------------------------------------------------------

/// Set Type 
static TYPE_SET_TYPE = SetType {
    "id": "set_type",
    "label": "Set Type",
    "description": "Set Type",
    "parent_group": Some(TYPE_GROUP),
    "group": TYPE_SET_GROUP,
    "types": [
        TYPE_SET_TYPE_ID,
        TYPE_SET_TYPE_LABEL,
        TYPE_SET_TYPE_DESCRIPTION,
        TYPE_SET_TYPE_GROUP,
        TYPE_SET_TYPE_PARENT_GROUP,
        TYPE_SET_TYPE_TYPES,
    ],
}

/// Set Type Group
static TYPE_SET_TYPE_GROUP = TypeGroup {
    "id": "set_type",
    "label": "Set Type",
}

/// Set Type: Id
static TYPE_SET_TYPE_ID = TextType {
    "id": "dd",
    "label": "Id",
    "description": "Id",
    "group": None,
    "parent_group": Some(TYPE_SET_TYPE_GROUP),
}

/// Set Type: Label
static TYPE_SET_TYPE_LABEL = TextType {
    "id": "sum_type_label",
    "label": "Label",
    "description": "Label",
    "group": None,
    "parent_group": Some(TYPE_SET_TYPE_GROUP),
}

/// Set Type: Description
static TYPE_SET_TYPE_DESCRIPTION = TextType {
    "id": "sum_type_description",
    "label": "Description",
    "description": "Description",
    "group": None,
    "parent_group": Some(TYPE_SET_TYPE_GROUP),
}

/// Set Type: Group
static TYPE_SET_TYPE_GROUP = TextType {
    "id": "sum_type_groups",
    "label": "Group",
    "description": "Group",
    "group": None,
    "parent_group": Some(TYPE_SET_TYPE_GROUP),
}

/// Set Type: Types
static TYPE_SET_TYPE_TYPES = ListType {
    "id": "types",
    "label": "Types",
    "description": "Types",
    "group": None,
    "parent_group": Some(TYPE_SET_TYPE_GROUP),
    "groups": [TYPE_SUM_TYPE_GROUP],
    "types": [TYPE_TYPE],
}


// TYPE > List ------------------------------------------------------------------

/// List Type 
static TYPE_LIST_TYPE = ListType {
    "id": "list_type",
    "label": "List Type",
    "description": "List Type",
    "parent_group": Some(TYPE_GROUP),
    "group": TYPE_LIST_GROUP,
    "types": [
        TYPE_LIST_TYPE_ID,
        TYPE_LIST_TYPE_LABEL,
        TYPE_LIST_TYPE_DESCRIPTION,
        TYPE_LIST_TYPE_GROUP,
        TYPE_LIST_TYPE_PARENT_GROUP,
        TYPE_LIST_TYPE_TYPES,
    ],
}

/// List Type Group
static TYPE_LIST_TYPE_GROUP = TypeGroup {
    "id": "list_type",
    "label": "List Type",
}

/// List Type: Id
static TYPE_LIST_TYPE_ID = TextType {
    "id": "dd",
    "label": "Id",
    "description": "Id",
    "group": None,
    "parent_group": Some(TYPE_LIST_TYPE_GROUP),
}

/// List Type: Label
static TYPE_LIST_TYPE_LABEL = TextType {
    "id": "sum_type_label",
    "label": "Label",
    "description": "Label",
    "group": None,
    "parent_group": Some(TYPE_LIST_TYPE_GROUP),
}

/// List Type: Description
static TYPE_LIST_TYPE_DESCRIPTION = TextType {
    "id": "sum_type_description",
    "label": "Description",
    "description": "Description",
    "group": None,
    "parent_group": Some(TYPE_LIST_TYPE_GROUP),
}

/// List Type: Group
static TYPE_LIST_TYPE_GROUP = TextType {
    "id": "sum_type_groups",
    "label": "Group",
    "description": "Group",
    "group": None,
    "parent_group": Some(TYPE_LIST_TYPE_GROUP),
}

/// List Type: Types
static TYPE_LIST_TYPE_TYPES = ListType {
    "id": "sum_type_types",
    "label": "Types",
    "description": "Types",
    "group": None,
    "parent_group": Some(TYPE_LIST_TYPE_GROUP),
    "groups": [TYPE_SUM_TYPE_GROUP],
    "types": [TYPE_TYPE],
}

// TYPE > Text -----------------------------------------------------------------

/// text Type 
static TYPE_TEXT_TYPE = TextType {
    "id": "text_type",
    "label": "Text Type",
    "description": "Text Type",
    "parent_group": Some(TYPE_GROUP),
    "group": TYPE_TEXT_GROUP,
    "types": [
        TYPE_TEXT_TYPE_ID,
        TYPE_TEXT_TYPE_LABEL,
        TYPE_TEXT_TYPE_DESCRIPTION,
        TYPE_TEXT_TYPE_GROUP,
        TYPE_TEXT_TYPE_PARENT_GROUP,
        TYPE_TEXT_TYPE_TYPES,
    ],
}

/// Text Type Group
static TYPE_TEXT_TYPE_GROUP = TypeGroup {
    "id": "text_type",
    "label": "Text Type",
}

/// Text Type: Id
static TYPE_TEXT_TYPE_ID = TextType {
    "id": "id",
    "label": "Id",
    "description": "Id",
    "group": None,
    "parent_group": Some(TYPE_TEXT_TYPE_GROUP),
}

/// Text Type: Label
static TYPE_TEXT_TYPE_LABEL = TextType {
    "id": "label",
    "label": "Label",
    "description": "Label",
    "group": None,
    "parent_group": Some(TYPE_TEXT_TYPE_GROUP),
}

/// Text Type: Description
static TYPE_TEXT_TYPE_DESCRIPTION = TextType {
    "id": "description",
    "label": "Description",
    "description": "Description",
    "group": None,
    "parent_group": Some(TYPE_TEXT_TYPE_GROUP),
}

/// Text Type: Group
static TYPE_TEXT_TYPE_GROUP = TextType {
    "id": "group",
    "label": "Group",
    "description": "Group",
    "group": None,
    "parent_group": Some(TYPE_TEXT_TYPE_GROUP),
}

// TYPE > Integer --------------------------------------------------------------

/// Integer Type 
static TYPE_INTEGER_TYPE = SetType {
    "id": "integer_type",
    "label": "Integer Type",
    "description": "Integer Type",
    "parent_group": Some(TYPE_GROUP),
    "group": TYPE_INTEGER_GROUP,
    "types": [
        TYPE_INTEGER_TYPE_ID,
        TYPE_INTEGER_TYPE_LABEL,
        TYPE_INTEGER_TYPE_DESCRIPTION,
        TYPE_INTEGER_TYPE_GROUP,
        TYPE_INTEGER_TYPE_PARENT_GROUP,
        TYPE_INTEGER_TYPE_TYPES,
    ],
}

/// Integer Type Group
static TYPE_INTEGER_TYPE_GROUP = TypeGroup {
    "id": "integer_type",
    "label": "Integer Type",
}

/// Integer Type: Id
static TYPE_INTEGER_TYPE_ID = TextType {
    "id": "id",
    "label": "Id",
    "description": "Id",
    "group": None,
    "parent_group": Some(TYPE_INTEGER_TYPE_GROUP),
}

/// Integer Type: Label
static TYPE_INTEGER_TYPE_LABEL = TextType {
    "id": "label",
    "label": "Label",
    "description": "Label",
    "group": None,
    "parent_group": Some(TYPE_INTEGER_TYPE_GROUP),
}

/// Integer Type: Description
static TYPE_INTEGER_TYPE_DESCRIPTION = TextType {
    "id": "description",
    "label": "Description",
    "description": "Description",
    "group": None,
    "parent_group": Some(TYPE_INTEGER_TYPE_GROUP),
}

/// Integer Type: Group
static TYPE_INTEGER_TYPE_GROUP = TextType {
    "id": "group",
    "label": "Group",
    "description": "Group",
    "group": None,
    "parent_group": Some(TYPE_INTEGER_TYPE_GROUP),
}

// TYPE > Float ----------------------------------------------------------------

/// Float Type 
static TYPE_FLOAT_TYPE = SetType {
    "id": "type_float",
    "label": "Float Type",
    "description": "Float Type",
    "parent_group": Some(TYPE_GROUP),
    "group": TYPE_FLOAT_GROUP,
    "types": [
        TYPE_FLOAT_TYPE_ID,
        TYPE_FLOAT_TYPE_LABEL,
        TYPE_FLOAT_TYPE_DESCRIPTION,
        TYPE_FLOAT_TYPE_GROUP,
        TYPE_FLOAT_TYPE_PARENT_GROUP,
        TYPE_FLOAT_TYPE_TYPES,
    ],
}

/// FLOAT Type Group
static TYPE_FLOAT_TYPE_GROUP = TypeGroup {
    "id": "FLOAT_type",
    "label": "FLOAT Type",
}

/// FLOAT Type: Id
static TYPE_FLOAT_TYPE_ID = TextType {
    "id": "id",
    "label": "Id",
    "description": "Id",
    "group": None,
    "parent_group": Some(TYPE_FLOAT_TYPE_GROUP),
}

/// FLOAT Type: Label
static TYPE_FLOAT_TYPE_LABEL = TextType {
    "id": "label",
    "label": "Label",
    "description": "Label",
    "group": None,
    "parent_group": Some(TYPE_FLOAT_TYPE_GROUP),
}

/// FLOAT Type: Description
static TYPE_FLOAT_TYPE_DESCRIPTION = TextType {
    "id": "description",
    "label": "Description",
    "description": "Description",
    "group": None,
    "parent_group": Some(TYPE_FLOAT_TYPE_GROUP),
}

/// FLOAT Type: Group
static TYPE_FLOAT_TYPE_GROUP = TextType {
    "id": "group",
    "label": "Group",
    "description": "Group",
    "group": None,
    "parent_group": Some(TYPE_FLOAT_TYPE_GROUP),
}

// TYPE > Symbol ---------------------------------------------------------------

/// Symbol Type 
static TYPE_SYMBOL_TYPE = SetType {
    "id": "type_Symbol",
    "label": "Symbol Type",
    "description": "Symbol Type",
    "parent_group": Some(TYPE_GROUP),
    "group": TYPE_YMBOL_GROUP,
    "types": [
        TYPE_SYMBOL_TYPE_ID,
        TYPE_SYMBOL_TYPE_LABEL,
        TYPE_SYMBOL_TYPE_DESCRIPTION,
        TYPE_SYMBOL_TYPE_GROUP,
        TYPE_SYMBOL_TYPE_PARENT_GROUP,
        TYPE_SYMBOL_TYPE_TYPES,
    ],
}

/// Symbol Type Group
static TYPE_SYMBOL_TYPE_GROUP = TypeGroup {
    "id": "Symbol_type",
    "label": "Symbol Type",
}

/// Symbol Type: Id
static TYPE_SYMBOL_TYPE_ID = TextType {
    "id": "id",
    "label": "Id",
    "description": "Id",
    "group": None,
    "parent_group": Some(TYPE_SYMBOL_TYPE_GROUP),
}

/// Symbol Type: Label
static TYPE_SYMBOL_TYPE_LABEL = TextType {
    "id": "label",
    "label": "Label",
    "description": "Label",
    "group": None,
    "parent_group": Some(TYPE_SYMBOL_TYPE_GROUP),
}

/// Symbol Type: Description
static TYPE_SYMBOL_TYPE_DESCRIPTION = TextType {
    "id": "description",
    "label": "Description",
    "description": "Description",
    "group": None,
    "parent_group": Some(TYPE_SYMBOL_TYPE_GROUP),
}

/// Symbol Type: Group
static TYPE_SYMBOL_TYPE_GROUP = TextType {
    "id": "group",
    "label": "Group",
    "description": "Group",
    "group": None,
    "parent_group": Some(TYPE_SYMBOL_TYPE_GROUP),
}
