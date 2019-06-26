# Compile-time checks?

API calls often *require* some parameters. Should we miss those parameters when performing a request, either the client will produce a runtime error or the server will reject our request. Our generated client code on the other hand, uses markers to avoid this problem at compile-time.
