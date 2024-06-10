# protocol-spec
- Clients send commands to a Redis server as an array of bulk strings. The first (and sometimes also the second) bulk string in the array is the command's name. Subsequent elements of the array are the arguments for the command.
- The server replies with a RESP type. The reply's type is determined by the command's implementation and possibly by the client's protocol version.



https://redis.io/docs/latest/develop/reference/protocol-spec/#request-response-model