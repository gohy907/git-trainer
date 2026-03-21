#!/bin/bash

cd "$HOME"

OUTPUT=$(cd hello-world && git log --oneline --format=%s)
OUTPUT_EXPECTED="Initial commit"

if echo "$OUTPUT" | grep -iq "^$OUTPUT_EXPECTED" ; then
	echo "4. Коммит имеет название \"Initial commit\"."
	exit 0
else 
    echo "4. Убедитесь, что коммит имеет название \"Initial commit\" (найдено: $OUTPUT)."
	exit 1
fi
