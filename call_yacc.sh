@echo on

SCRIPT_DIR=$(cd -- "$(dirname -- "$0")" && pwd)

$SCRIPT_DIR/target/debug/yacc "$@" || exit $?

filename=$1
filename="${filename%.c}.s"
executable="${filename%.*}"

echo "filename is $filename and executable is '$executable'"

gcc $filename -o $executable
