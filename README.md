# poeten
Poetry website - based on Axum server + yew webassembly app

To run yew app standalone

```
% cd frontend
% trunk serve --open
```

To let Axum webserver serve the app:
```
% cd frontend
% trunk build

% cd ../server
% cargo run
```
