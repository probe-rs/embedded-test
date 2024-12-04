#!/bin/bash

touch stdout
touch stderr

/home/probe-rs-runner/probe-rs run --disable-progressbars "$TARGET" $TARGET_CONFIG  --format=json  2> stderr | jq -c 'del(.exec_time)' >stdout

# only fetch log rtt logs from stderr, ignore backtraces for now, remove function addresses
cat stderr | sed -nE 's/^[0-9]{1,2}:[0-9]{2}:[0-9]{2}\.[0-9]{3}: (TRACE|DEBUG|INFO|WARN|ERROR)/\1/p' | sed 's/ function: 0x[0-9a-fA-F]\{1,\},//g'> rtt_logs

echo "------------------------------------------------" >&2
echo "Stdout" >&2
echo "------------------------------------------------" >&2
cat stdout >&2
echo "------------------------------------------------" >&2
echo "Stderr" >&2
echo "------------------------------------------------" >&2
cat stderr >&2
echo "------------------------------------------------" >&2
echo "Rtt logs" >&2
echo "------------------------------------------------" >&2
cat rtt_logs >&2
echo "------------------------------------------------" >&2

res=0

# Output markers around diff to render it nicely in markdown
echo "Test Results diff:"
echo "\`\`\`diff"
if ! diff expected_test_results.txt stdout; then
  res=1
fi


echo "\`\`\`"

echo "Rtt logs diff:"
echo "\`\`\`diff"
if ! diff expected_rtt_logs.txt rtt_logs; then
  res=1
fi
echo "\`\`\`"

rm -f stdout stderr rtt_logs

exit $res