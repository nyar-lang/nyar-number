(component $Root
    (type $array (list u8))

    (import "wasi:io/error@0.2.0" (instance $wasi:io/error@0.2.0
        (export "error" (type (sub resource)))
    ))
    (alias export $wasi:io/error@0.2.0 "error" (type $io-error))

    (type $stream-error (variant
        (case "last-operation-failed" (own $io-error))
        (case "closed")
    ))
    (type $stream-result (result (error $stream-error)))

    (import "wasi:io/streams@0.2.0" (instance $wasi:io/streams@0.2.0
        (export $output-stream "output-stream" (type (sub resource)))
        (alias outer $Root $array (type $array))
        (alias outer $Root $io-error (type $io-error))
        (alias outer $Root $stream-error (type $stream-error))

        (export $stream-error0 "stream-error" (type (eq $stream-error)))
        (type $stream-result (result (error $stream-error0)))

        (export "[method]output-stream.blocking-write-and-flush"
            (func (param "self" (borrow $output-stream)) (param "contents" $array) (result $stream-result))
        )
    ))
    (alias export $wasi:io/streams@0.2.0 "output-stream" (type $output-stream))
    (alias export $wasi:io/streams@0.2.0 "[method]output-stream.blocking-write-and-flush" (func $output-stream.blocking-write-and-flush))

    (import "wasi:cli/stdout@0.2.0" (instance $wasi:cli/stdout@0.2.0
        (alias outer $Root $output-stream (type $output-stream))
        (export "get-stdout" (func (result (own $output-stream))))
    ))
)

