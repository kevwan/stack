# stack
A cli tool to prettify stacktrace in json logs.

## Why to write this tool?
When we read the stacktrace info from json logs, it's hard to read without pretty print.

## From stdin
```json
{"@timestamp":"2023-01-20T19:56:57.806+08:00","content":"panic\ngoroutine 1 [running]:\nruntime/debug.Stack()\n\t/usr/local/go/src/runtime/debug/stack.go:24 +0x64\ngithub.com/zeromicro/go-zero/core/logx.writeStack({0x14000092008, 0x5})\n\t/Users/kevin/Develop/go/opensource/go-zero/core/logx/logs.go:457 +0x48\ngithub.com/zeromicro/go-zero/core/logx.ErrorStack({0x140002dff58?, 0x1400010e000?, 0x0?})\n\t/Users/kevin/Develop/go/opensource/go-zero/core/logx/logs.go:113 +0x24\nmain.main()\n\t/Users/kevin/Develop/go/opensource/go-zero/adhoc/log/main.go:17 +0x80\n","level":"error"}
```

```shell
$ tail -f error.log | stack -i
{"@timestamp":"2023-01-20T19:57:01.810+08:00","content":"panic
goroutine 1 [running]:
runtime/debug.Stack()
	/usr/local/go/src/runtime/debug/stack.go:24 +0x64
github.com/zeromicro/go-zero/core/logx.writeStack({0x1400012e240, 0x5})
	/Users/kevin/Develop/go/opensource/go-zero/core/logx/logs.go:457 +0x48
github.com/zeromicro/go-zero/core/logx.ErrorStack({0x140002dff58?, 0x1400009c540?, 0x140000b8060?})
	/Users/kevin/Develop/go/opensource/go-zero/core/logx/logs.go:113 +0x24
main.main()
	/Users/kevin/Develop/go/opensource/go-zero/adhoc/log/main.go:17 +0x80
","level":"error"}
```

## From clipboard

### Steps

1. copy the content
2. run `stack` without `-i`

```shell
{"@timestamp":"2023-01-20T19:57:01.810+08:00","content":"panic
goroutine 1 [running]:
runtime/debug.Stack()
	/usr/local/go/src/runtime/debug/stack.go:24 +0x64
github.com/zeromicro/go-zero/core/logx.writeStack({0x1400012e240, 0x5})
	/Users/kevin/Develop/go/opensource/go-zero/core/logx/logs.go:457 +0x48
github.com/zeromicro/go-zero/core/logx.ErrorStack({0x140002dff58?, 0x1400009c540?, 0x140000b8060?})
	/Users/kevin/Develop/go/opensource/go-zero/core/logx/logs.go:113 +0x24
main.main()
	/Users/kevin/Develop/go/opensource/go-zero/adhoc/log/main.go:17 +0x80
","level":"error"}
```
