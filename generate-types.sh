DATA_URL=$1
MAIN_TYPE_NAME=$2
MODULE_NAME=$3

if [ -z "$DATA_URL" ] || [ -z "$MAIN_TYPE_NAME" ] || [ -z "$MODULE_NAME" ]; then
    echo "Usage: $0 <data_url> <main_type_name> <module_name>"
    exit 1
fi

JSON_DATA=$(curl -s $DATA_URL)

mkdir -p src/types/$MODULE_NAME

echo $JSON_DATA | quicktype --lang "rust" --derive-debug --derive-clone --derive-partial-eq --visibility public --prefer-maps -t $MAIN_TYPE_NAME > src/types/$MODULE_NAME/types.rs