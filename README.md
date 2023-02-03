# poeten
Poetry website - based on [Axum server](https://github.com/rksm/axum-yew-setup) + [yew](https://github.com/yewstack/yew) webassembly app. 

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
